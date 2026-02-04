use anyhow::Result;
use tracing::{debug, error};

#[cfg(feature = "nvidia")]
use nvml_wrapper::Nvml;

use super::GpuStats;

pub struct NvidiaMonitor {
    #[cfg(feature = "nvidia")]
    nvml: Nvml,
}

impl NvidiaMonitor {
    #[cfg(feature = "nvidia")]
    pub fn new() -> Result<Self> {
        match Nvml::init() {
            Ok(nvml) => {
                debug!("NVML initialized successfully");
                Ok(Self { nvml })
            }
            Err(e) => {
                error!("Failed to initialize NVML: {}", e);
                anyhow::bail!("Failed to initialize NVML: {}", e)
            }
        }
    }

    #[cfg(not(feature = "nvidia"))]
    pub fn new() -> Result<Self> {
        anyhow::bail!("NVIDIA support not compiled in")
    }

    #[cfg(feature = "nvidia")]
    pub async fn get_stats(&self) -> Result<GpuStats> {
        // Get the first GPU device
        let device = self.nvml.device_by_index(0)?;

        // Get device name
        let name = device.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());

        // Get GPU utilization
        let utilization = device
            .utilization_rates()
            .map(|rates| rates.gpu)
            .unwrap_or(0);

        // Get memory info
        let memory_info = device.memory_info()?;
        let memory_used = memory_info.used;
        let memory_total = memory_info.total;

        // Get temperature
        let temperature = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0);

        debug!(
            "GPU Stats: {} - {}% util, {}/{} MB, {}°C",
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

    #[cfg(not(feature = "nvidia"))]
    pub async fn get_stats(&self) -> Result<GpuStats> {
        anyhow::bail!("NVIDIA support not compiled in")
    }
}
