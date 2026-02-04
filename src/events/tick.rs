use std::time::Duration;

/// Represents a tick event for periodic updates
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct TickEvent {
    pub interval: Duration,
}

impl TickEvent {
    #[allow(dead_code)]
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval: Duration::from_millis(interval_ms),
        }
    }
}

impl Default for TickEvent {
    fn default() -> Self {
        Self::new(250)
    }
}
