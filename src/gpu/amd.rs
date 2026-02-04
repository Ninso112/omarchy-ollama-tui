use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::debug;

use super::GpuStats;

pub struct AmdMonitor {
    device_path: String,
}

impl AmdMonitor {
    pub fn new() -> Result<Self> {
        // Try to detect AMD GPU via sysfs
        let base_path = "/sys/class/drm";

        if !Path::new(base_path).exists() {
            anyhow::bail!("AMD GPU sysfs not available");
        }

        // Look for AMD GPU (card0, card1, etc.)
        for entry in fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Look for card* directories
            if name_str.starts_with("card") && !name_str.contains("render") {
                // Check if it's an AMD GPU
                let vendor_path = path.join("device/vendor");
                if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                    // AMD vendor ID is 0x1002
                    if vendor.trim() == "0x1002" {
                        debug!("Found AMD GPU at: {:?}", path);
                        return Ok(Self {
                            device_path: path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        anyhow::bail!("No AMD GPU found")
    }

    pub async fn get_stats(&self) -> Result<GpuStats> {
        let device_path = Path::new(&self.device_path);

        // Get GPU name
        let name = self.get_gpu_name(device_path)?;

        // Get GPU utilization (from GPU busy percentage)
        let utilization = self.get_gpu_utilization(device_path)?;

        // Get VRAM usage
        let (memory_used, memory_total) = self.get_vram_usage(device_path)?;

        // Get temperature
        let temperature = self.get_temperature(device_path)?;

        debug!(
            "AMD GPU Stats: {} - {}% util, {}/{} MB, {}°C",
            name,
            utilization,
            memory_used / (1024 * 1024),
            memory_total / (1024 * 1024),
            temperature
        );

        Ok(GpuStats {
            name,
            utilization,
            memory_used,
            memory_total,
            temperature,
        })
    }

    fn get_gpu_name(&self, device_path: &Path) -> Result<String> {
        // Try to read GPU name from various sources

        // Try device/product_name first
        let product_name_path = device_path.join("device/product_name");
        if let Ok(name) = fs::read_to_string(&product_name_path) {
            return Ok(format!("AMD {}", name.trim()));
        }

        // Try device name
        let device_name_path = device_path.join("device/name");
        if let Ok(name) = fs::read_to_string(&device_name_path) {
            return Ok(format!("AMD {}", name.trim()));
        }

        // Fallback: read device ID and try to identify
        let device_id_path = device_path.join("device/device");
        if let Ok(device_id) = fs::read_to_string(&device_id_path) {
            let id = device_id.trim();
            let gpu_name = match id {
                "0x73bf" => "Radeon RX 7900 XTX",
                "0x744c" => "Radeon RX 7900 XT",
                "0x73df" => "Radeon RX 6950 XT",
                "0x73af" => "Radeon RX 6900 XT",
                "0x73a0" => "Radeon RX 6800 XT",
                "0x73ab" => "Radeon RX 6800",
                "0x73ff" => "Radeon RX 6700 XT",
                "0x73e0" => "Radeon RX 6650 XT",
                "0x73ef" => "Radeon RX 6600 XT",
                "0x73e3" => "Radeon RX 6600",
                _ => "GPU",
            };
            return Ok(format!("AMD {}", gpu_name));
        }

        Ok("AMD GPU".to_string())
    }

    fn get_gpu_utilization(&self, device_path: &Path) -> Result<u32> {
        // Try to read GPU busy percentage
        let gpu_busy_path = device_path.join("device/gpu_busy_percent");
        if let Ok(busy) = fs::read_to_string(&gpu_busy_path) {
            if let Ok(percent) = busy.trim().parse::<u32>() {
                return Ok(percent);
            }
        }

        // Alternative: read from hwmon
        if let Ok(hwmon_path) = self.find_hwmon_path(device_path) {
            // Some AMD drivers expose usage via hwmon
            let usage_path = hwmon_path.join("device/gpu_busy_percent");
            if let Ok(usage) = fs::read_to_string(&usage_path) {
                if let Ok(percent) = usage.trim().parse::<u32>() {
                    return Ok(percent);
                }
            }
        }

        // If we can't get real data, return 0
        debug!("Could not read AMD GPU utilization");
        Ok(0)
    }

    fn get_vram_usage(&self, device_path: &Path) -> Result<(u64, u64)> {
        // Read VRAM usage from sysfs
        let vram_used_path = device_path.join("device/mem_info_vram_used");
        let vram_total_path = device_path.join("device/mem_info_vram_total");

        let used = if let Ok(used_str) = fs::read_to_string(&vram_used_path) {
            used_str.trim().parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let total = if let Ok(total_str) = fs::read_to_string(&vram_total_path) {
            total_str.trim().parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        if total == 0 {
            // Try alternative paths
            let mem_used_vis = device_path.join("device/mem_info_vis_vram_used");
            let mem_total_vis = device_path.join("device/mem_info_vis_vram_total");

            let used_vis = if let Ok(used_str) = fs::read_to_string(&mem_used_vis) {
                used_str.trim().parse::<u64>().unwrap_or(0)
            } else {
                0
            };

            let total_vis = if let Ok(total_str) = fs::read_to_string(&mem_total_vis) {
                total_str.trim().parse::<u64>().unwrap_or(0)
            } else {
                0
            };

            if total_vis > 0 {
                return Ok((used_vis, total_vis));
            }
        }

        Ok((used, total))
    }

    fn get_temperature(&self, device_path: &Path) -> Result<u32> {
        // Try to find temperature via hwmon
        if let Ok(hwmon_path) = self.find_hwmon_path(device_path) {
            // Look for temp1_input (GPU temperature in millidegrees)
            let temp_path = hwmon_path.join("temp1_input");
            if let Ok(temp_str) = fs::read_to_string(&temp_path) {
                if let Ok(temp_millidegrees) = temp_str.trim().parse::<u32>() {
                    return Ok(temp_millidegrees / 1000);
                }
            }

            // Alternative: edge temperature
            let edge_temp_path = hwmon_path.join("temp2_input");
            if let Ok(temp_str) = fs::read_to_string(&edge_temp_path) {
                if let Ok(temp_millidegrees) = temp_str.trim().parse::<u32>() {
                    return Ok(temp_millidegrees / 1000);
                }
            }
        }

        debug!("Could not read AMD GPU temperature");
        Ok(0)
    }

    fn find_hwmon_path(&self, device_path: &Path) -> Result<std::path::PathBuf> {
        let hwmon_base = device_path.join("device/hwmon");

        if !hwmon_base.exists() {
            anyhow::bail!("hwmon directory not found");
        }

        // Find the hwmon* directory
        for entry in fs::read_dir(&hwmon_base)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with("hwmon") {
                return Ok(path);
            }
        }

        anyhow::bail!("No hwmon device found")
    }
}
