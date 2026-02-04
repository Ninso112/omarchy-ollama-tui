use anyhow::Result;
use tracing::debug;

use super::GpuStats;

pub struct FallbackMonitor {}

impl FallbackMonitor {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn get_stats(&self) -> Result<GpuStats> {
        // Note: sysinfo doesn't provide GPU-specific information
        // This is a fallback that provides basic system info

        debug!("Using fallback GPU monitoring (limited functionality)");

        // We can't get real GPU stats without vendor-specific APIs
        // Return placeholder values
        Ok(GpuStats {
            name: "Unknown GPU (no NVML)".to_string(),
            utilization: 0,
            memory_used: 0,
            memory_total: 0,
            temperature: 0,
        })
    }
}

impl Default for FallbackMonitor {
    fn default() -> Self {
        Self::new().expect("Failed to create fallback monitor")
    }
}
