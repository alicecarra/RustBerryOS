#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/time.rs"]
mod arch_time;

use core::time::Duration;

pub struct TimeManager;

static TIME_MANAGER: TimeManager = TimeManager::new();

pub fn time_manager() -> &'static TimeManager {
    &TIME_MANAGER
}

impl TimeManager {
    pub const fn new() -> Self {
        Self
    }

    pub fn resolution(&self) -> Duration {
        arch_time::resolution()
    }

    /// The uptime since power-on of the device.
    ///
    /// This includes time consumed by firmware and bootloaders.
    pub fn uptime(&self) -> Duration {
        arch_time::uptime()
    }

    pub fn spin_for(&self, duration: Duration) {
        arch_time::spin_for(duration)
    }
}
