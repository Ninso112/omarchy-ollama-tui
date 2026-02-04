pub mod amd;
pub mod fallback;
pub mod intel;
pub mod nvidia;

use anyhow::Result;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct GpuStats {
    pub name: String,
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

pub struct GpuMonitor {
    gpu_type: GpuType,
    nvidia_monitor: Option<nvidia::NvidiaMonitor>,
    amd_monitor: Option<amd::AmdMonitor>,
    intel_monitor: Option<intel::IntelMonitor>,
    fallback_monitor: fallback::FallbackMonitor,
}

impl GpuMonitor {
    pub fn new() -> Result<Self> {
        info!("Detecting GPU hardware...");

        // Try to detect and initialize GPU in order of preference
        let mut gpu_type = GpuType::Unknown;
        let mut nvidia_monitor = None;
        let mut amd_monitor = None;
        let mut intel_monitor = None;

        // Try NVIDIA first
        if let Ok(monitor) = nvidia::NvidiaMonitor::new() {
            info!("NVIDIA GPU detected");
            gpu_type = GpuType::Nvidia;
            nvidia_monitor = Some(monitor);
        }
        // Try AMD if NVIDIA failed
        else if let Ok(monitor) = amd::AmdMonitor::new() {
            info!("AMD GPU detected");
            gpu_type = GpuType::Amd;
            amd_monitor = Some(monitor);
        }
        // Try Intel if both NVIDIA and AMD failed
        else if let Ok(monitor) = intel::IntelMonitor::new() {
            info!("Intel GPU detected");
            gpu_type = GpuType::Intel;
            intel_monitor = Some(monitor);
        } else {
            info!("No dedicated GPU detected, using fallback monitoring");
        }

        let fallback_monitor = fallback::FallbackMonitor::new()?;

        Ok(Self {
            gpu_type,
            nvidia_monitor,
            amd_monitor,
            intel_monitor,
            fallback_monitor,
        })
    }

    pub async fn get_stats(&self) -> Result<GpuStats> {
        // Try the detected GPU type first
        match self.gpu_type {
            GpuType::Nvidia => {
                if let Some(ref monitor) = self.nvidia_monitor {
                    if let Ok(stats) = monitor.get_stats().await {
                        debug!("Got NVIDIA GPU stats");
                        return Ok(stats);
                    }
                }
            }
            GpuType::Amd => {
                if let Some(ref monitor) = self.amd_monitor {
                    if let Ok(stats) = monitor.get_stats().await {
                        debug!("Got AMD GPU stats");
                        return Ok(stats);
                    }
                }
            }
            GpuType::Intel => {
                if let Some(ref monitor) = self.intel_monitor {
                    if let Ok(stats) = monitor.get_stats().await {
                        debug!("Got Intel GPU stats");
                        return Ok(stats);
                    }
                }
            }
            GpuType::Unknown => {}
        }

        // Fallback to sysinfo-based monitoring
        debug!("Using fallback GPU monitoring");
        self.fallback_monitor.get_stats().await
    }

    #[allow(dead_code)]
    pub fn get_gpu_type(&self) -> GpuType {
        self.gpu_type
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        Self::new().expect("Failed to create GPU monitor")
    }
}
