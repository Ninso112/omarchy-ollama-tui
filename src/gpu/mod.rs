pub mod amd;
pub mod fallback;
pub mod intel;
pub mod nvidia;

use anyhow::Result;
use tracing::{debug, info};
use async_trait::async_trait;

#[async_trait]
pub trait GpuProvider: Send + Sync {
    async fn get_stats(&self) -> Result<GpuStats>;
}

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
    provider: Box<dyn GpuProvider>,
}

impl GpuMonitor {
    pub fn new() -> Result<Self> {
        info!("Detecting GPU hardware...");

        // Try to detect and initialize GPU in order of preference
        let mut gpu_type = GpuType::Unknown;
        let provider: Box<dyn GpuProvider>;

        // Try NVIDIA first
        if let Ok(monitor) = nvidia::NvidiaMonitor::new() {
            info!("NVIDIA GPU detected");
            gpu_type = GpuType::Nvidia;
            provider = Box::new(monitor);
        } else if let Ok(monitor) = amd::AmdMonitor::new() {
            info!("AMD GPU detected");
            gpu_type = GpuType::Amd;
            provider = Box::new(monitor);
        } else if let Ok(monitor) = intel::IntelMonitor::new() {
            info!("Intel GPU detected");
            gpu_type = GpuType::Intel;
            provider = Box::new(monitor);
        } else {
            info!("No dedicated GPU detected, using fallback monitoring");
            gpu_type = GpuType::Unknown;
            let fallback = fallback::FallbackMonitor::new()?;
            provider = Box::new(fallback);
        }

        Ok(Self { gpu_type, provider })
    }

    pub async fn get_stats(&self) -> Result<GpuStats> {
        // Delegate to the selected provider
        let stats = self.provider.get_stats().await;
        debug!("GPU stats obtained via provider");
        stats
    }

    #[allow(dead_code)]
    pub fn get_gpu_type(&self) -> GpuType {
        self.gpu_type
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| panic!("Failed to create GPU monitor: {}", e))
    }
}
