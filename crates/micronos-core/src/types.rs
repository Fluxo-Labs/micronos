/// Unique identifier for a process.
///
/// This is intentionally a lightweight newtype so APIs remain explicit and
/// cannot accidentally mix process IDs with other identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessId(pub u64);

/// Unique identifier for a thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadId(pub u64);

/// Virtual or physical address depending on the subsystem using it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryAddress(pub usize);

/// Memory size expressed in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySize(pub usize);

/// Identifier for a network node.
///
/// The size is fixed to make it easy to use in `no_std` contexts and to allow
/// deterministic serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NodeId(pub [u8; 32]);

impl NodeId {
    pub fn new(data: [u8; 32]) -> Self {
        NodeId(data)
    }
}

/// Scheduling priority.
///
/// Values are ordered from lowest (`Idle`) to highest (`RealTime`).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Priority {
    #[default]
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    RealTime = 4,
}

/// System lifecycle state machine.
///
/// This is the top-level state used by the hosted runner to coordinate boot and
/// shutdown transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemState {
    #[default]
    Off,
    Initializing,
    Booting,
    Running {
        uptime_ticks: u64,
    },
    ShuttingDown,
    Error {
        error_code: u32,
    },
}

impl SystemState {
    /// Create a new system state.
    pub fn new() -> Self {
        SystemState::Off
    }

    /// Transition from `Off` to `Initializing`.
    ///
    /// Returns `true` if the transition was applied.
    pub fn boot(&mut self) -> bool {
        if matches!(self, SystemState::Off) {
            *self = SystemState::Initializing;
            true
        } else {
            false
        }
    }

    /// Transition from `Initializing` to `Booting`.
    ///
    /// Returns `true` if the transition was applied.
    pub fn initialize(&mut self) -> bool {
        if matches!(self, SystemState::Initializing) {
            *self = SystemState::Booting;
            true
        } else {
            false
        }
    }

    /// Transition from `Booting` to `Running`.
    ///
    /// Returns `true` if the transition was applied.
    pub fn complete_boot(&mut self) -> bool {
        if matches!(self, SystemState::Booting) {
            *self = SystemState::Running { uptime_ticks: 0 };
            true
        } else {
            false
        }
    }

    /// Transition from `Running` to `ShuttingDown`.
    ///
    /// Returns `true` if the transition was applied.
    pub fn shutdown(&mut self) -> bool {
        if matches!(self, SystemState::Running { .. }) {
            *self = SystemState::ShuttingDown;
            true
        } else {
            false
        }
    }

    /// Transition from `ShuttingDown` to `Off`.
    ///
    /// Returns `true` if the transition was applied.
    pub fn power_off(&mut self) -> bool {
        if matches!(self, SystemState::ShuttingDown) {
            *self = SystemState::Off;
            true
        } else {
            false
        }
    }

    /// Returns whether the system is in the `Running` state.
    pub fn is_running(&self) -> bool {
        matches!(self, SystemState::Running { .. })
    }
}
