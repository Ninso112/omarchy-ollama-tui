use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::debug;

use super::GpuStats;

pub struct IntelMonitor {
    device_path: String,
}

impl IntelMonitor {
    pub fn new() -> Result<Self> {
        // Try to detect Intel GPU via sysfs
        let base_path = "/sys/class/drm";

        if !Path::new(base_path).exists() {
            anyhow::bail!("Intel GPU sysfs not available");
        }

        // Look for Intel GPU (card0, card1, etc.)
        for entry in fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Look for card* directories
            if name_str.starts_with("card") && !name_str.contains("render") {
                // Check if it's an Intel GPU
                let vendor_path = path.join("device/vendor");
                if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                    // Intel vendor ID is 0x8086
                    if vendor.trim() == "0x8086" {
                        debug!("Found Intel GPU at: {:?}", path);
                        return Ok(Self {
                            device_path: path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        anyhow::bail!("No Intel GPU found")
    }

    pub async fn get_stats(&self) -> Result<GpuStats> {
        let device_path = Path::new(&self.device_path);

        // Get GPU name
        let name = self.get_gpu_name(device_path)?;

        // Get GPU utilization
        let utilization = self.get_gpu_utilization(device_path)?;

        // Get memory usage (Intel integrated GPUs share system RAM)
        let (memory_used, memory_total) = self.get_memory_usage(device_path)?;

        // Get temperature
        let temperature = self.get_temperature(device_path)?;

        debug!(
            "Intel GPU Stats: {} - {}% util, {}/{} MB, {}°C",
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

        // Try i915 specific paths first
        let gt_name_path = device_path.join("gt/gt0/name");
        if let Ok(name) = fs::read_to_string(&gt_name_path) {
            return Ok(format!("Intel {}", name.trim()));
        }

        // Try device name
        let device_name_path = device_path.join("device/name");
        if let Ok(name) = fs::read_to_string(&device_name_path) {
            return Ok(format!("Intel {}", name.trim()));
        }

        // Fallback: read device ID and try to identify
        let device_id_path = device_path.join("device/device");
        if let Ok(device_id) = fs::read_to_string(&device_id_path) {
            let id = device_id.trim();
            let gpu_name = match id {
                // Arc Series
                "0x56a0" | "0x56a1" | "0x56a2" => "Arc A770",
                "0x5690" | "0x5691" | "0x5692" => "Arc A750",
                "0x56a5" | "0x56a6" => "Arc A580",
                "0x56b0" | "0x56b1" => "Arc A380",
                "0x56c0" | "0x56c1" => "Arc A310",

                // Iris Xe / Xe Graphics
                "0x9a49" | "0x9a40" | "0x9a59" => "Iris Xe Graphics",
                "0x9a60" | "0x9a68" | "0x9a70" => "UHD Graphics",

                // Tiger Lake
                "0x9a78" => "UHD Graphics (Tiger Lake)",

                // Alder Lake
                "0x4680" | "0x4682" | "0x4688" => "UHD Graphics (Alder Lake)",

                // Raptor Lake
                "0xa780" | "0xa781" | "0xa782" => "UHD Graphics (Raptor Lake)",

                _ => "Graphics",
            };
            return Ok(format!("Intel {}", gpu_name));
        }

        Ok("Intel GPU".to_string())
    }

    fn get_gpu_utilization(&self, device_path: &Path) -> Result<u32> {
        // Try i915 specific frequency info to estimate utilization
        let cur_freq_path = device_path.join("gt/gt0/rps_cur_freq_mhz");
        let max_freq_path = device_path.join("gt/gt0/rps_max_freq_mhz");

        if let (Ok(cur), Ok(max)) = (
            fs::read_to_string(&cur_freq_path),
            fs::read_to_string(&max_freq_path),
        ) {
            if let (Ok(cur_freq), Ok(max_freq)) =
                (cur.trim().parse::<u32>(), max.trim().parse::<u32>())
            {
                if max_freq > 0 {
                    let utilization = ((cur_freq as f64 / max_freq as f64) * 100.0) as u32;
                    return Ok(utilization.min(100));
                }
            }
        }

        // Alternative: try to read from sysfs engine usage
        let render_path = device_path.join("engine/rcs0/busy");
        if let Ok(busy) = fs::read_to_string(&render_path) {
            if let Ok(percent) = busy.trim().parse::<u32>() {
                return Ok(percent);
            }
        }

        // If we can't get real data, return 0
        debug!("Could not read Intel GPU utilization");
        Ok(0)
    }

    fn get_memory_usage(&self, device_path: &Path) -> Result<(u64, u64)> {
        // Intel integrated GPUs typically share system RAM
        // Try to read from i915 gem objects

        let objects_path = device_path.join("i915_gem_objects");
        if let Ok(content) = fs::read_to_string(&objects_path) {
            // Parse the gem objects info
            for line in content.lines() {
                if line.contains("bytes") {
                    // Try to extract memory usage
                    if let Some(bytes_str) = line.split_whitespace().next() {
                        if let Ok(bytes) = bytes_str.parse::<u64>() {
                            // Estimate total as 2GB for integrated graphics (conservative)
                            let total = 2 * 1024 * 1024 * 1024; // 2GB
                            return Ok((bytes, total));
                        }
                    }
                }
            }
        }

        // Fallback: try to read from sysfs memory info
        let mem_used_path = device_path.join("mem_info_used");
        let mem_total_path = device_path.join("mem_info_total");

        let used = if let Ok(used_str) = fs::read_to_string(&mem_used_path) {
            used_str.trim().parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let total = if let Ok(total_str) = fs::read_to_string(&mem_total_path) {
            total_str.trim().parse::<u64>().unwrap_or(0)
        } else {
            // Default to 2GB if we can't determine
            2 * 1024 * 1024 * 1024
        };

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

            // Try other temperature sensors
            for i in 1..10 {
                let temp_label_path = hwmon_path.join(format!("temp{}_label", i));
                let temp_input_path = hwmon_path.join(format!("temp{}_input", i));

                if let Ok(label) = fs::read_to_string(&temp_label_path) {
                    if label.to_lowercase().contains("gpu")
                        || label.to_lowercase().contains("package")
                    {
                        if let Ok(temp_str) = fs::read_to_string(&temp_input_path) {
                            if let Ok(temp_millidegrees) = temp_str.trim().parse::<u32>() {
                                return Ok(temp_millidegrees / 1000);
                            }
                        }
                    }
                }
            }
        }

        // Try reading from thermal zone
        let thermal_path = "/sys/class/thermal";
        if Path::new(thermal_path).exists() {
            if let Ok(entries) = fs::read_dir(thermal_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let type_path = path.join("type");

                    if let Ok(thermal_type) = fs::read_to_string(&type_path) {
                        if thermal_type.to_lowercase().contains("gpu")
                            || thermal_type.to_lowercase().contains("pch")
                        {
                            let temp_path = path.join("temp");
                            if let Ok(temp_str) = fs::read_to_string(&temp_path) {
                                if let Ok(temp_millidegrees) = temp_str.trim().parse::<u32>() {
                                    return Ok(temp_millidegrees / 1000);
                                }
                            }
                        }
                    }
                }
            }
        }

        debug!("Could not read Intel GPU temperature");
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
