/// Kernel lifecycle state machine.
///
/// This enum is intentionally explicit to make illegal transitions easy to
/// detect and test.
#[derive(Debug, Clone, Default)]
pub enum KernelState {
    #[default]
    Uninitialized,
    Initialized {
        components: u32,
    },
    Running {
        uptime_ms: u64,
        threads: u32,
    },
    Paused {
        uptime_ms: u64,
        threads: u32,
    },
    Stopping,
    Stopped,
}

pub struct MicronKernel {
    /// Current kernel lifecycle state.
    pub state: KernelState,
}

impl MicronKernel {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const NAME: &'static str = "MicronOS";

    /// Create a new kernel in the `Uninitialized` state.
    pub fn new() -> Self {
        MicronKernel {
            state: KernelState::Uninitialized,
        }
    }

    /// Return static kernel metadata.
    pub fn info(&self) -> KernelInfo {
        KernelInfo {
            name: Self::NAME,
            version: Self::VERSION,
        }
    }

    /// Transition the kernel to `Initialized`.
    ///
    /// This is idempotent when called in other states.
    pub fn initialize(&mut self) {
        if matches!(self.state, KernelState::Uninitialized) {
            self.state = KernelState::Initialized { components: 0 };
        }
    }

    /// Transition from `Initialized` to `Running`.
    pub fn start(&mut self) -> Result<(), &'static str> {
        match self.state {
            KernelState::Initialized { .. } => {
                self.state = KernelState::Running {
                    uptime_ms: 0,
                    threads: 0,
                };
                Ok(())
            }
            _ => Err("Cannot start from current state"),
        }
    }

    /// Transition from `Running` to `Paused`.
    pub fn pause(&mut self) -> Result<(), &'static str> {
        if let KernelState::Running { uptime_ms, threads } = self.state {
            self.state = KernelState::Paused { uptime_ms, threads };
            Ok(())
        } else {
            Err("Cannot pause from current state")
        }
    }

    /// Transition from `Paused` to `Running`.
    pub fn resume(&mut self) -> Result<(), &'static str> {
        if let KernelState::Paused { uptime_ms, threads } = self.state {
            self.state = KernelState::Running { uptime_ms, threads };
            Ok(())
        } else {
            Err("Cannot resume from current state")
        }
    }

    /// Transition from `Running` to `Stopped`.
    ///
    /// Note: the current implementation models stopping synchronously.
    pub fn stop(&mut self) -> Result<(), &'static str> {
        if matches!(self.state, KernelState::Running { .. }) {
            self.state = KernelState::Stopping;
            self.state = KernelState::Stopped;
            Ok(())
        } else {
            Err("Cannot stop from current state")
        }
    }

    /// Returns whether the kernel is currently running.
    pub fn is_running(&self) -> bool {
        matches!(self.state, KernelState::Running { .. })
    }
}

impl Default for MicronKernel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct KernelInfo {
    pub name: &'static str,
    pub version: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_info() {
        let kernel = MicronKernel::new();
        let info = kernel.info();
        assert_eq!(info.name, "MicronOS");
        assert!(!info.version.is_empty());
    }

    #[test]
    fn test_kernel_lifecycle() {
        let mut kernel = MicronKernel::new();
        kernel.initialize();
        assert!(matches!(kernel.state, KernelState::Initialized { .. }));

        kernel.start().unwrap();
        assert!(matches!(kernel.state, KernelState::Running { .. }));

        kernel.pause().unwrap();
        assert!(matches!(kernel.state, KernelState::Paused { .. }));

        kernel.resume().unwrap();
        assert!(matches!(kernel.state, KernelState::Running { .. }));

        kernel.stop().unwrap();
        assert!(matches!(kernel.state, KernelState::Stopped));
    }
}
