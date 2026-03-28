use crate::device::{DeviceId, DeviceStatus, DeviceType};
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DriverState {
    #[default]
    Uninitialized,
    Initializing,
    Ready,
    Running,
    Suspended,
    Error,
    Removed,
}

#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub device_id: Option<DeviceId>,
    pub device_type: DeviceType,
}

impl DriverInfo {
    pub fn new(name: &str, device_type: DeviceType) -> Self {
        DriverInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            author: String::new(),
            description: String::new(),
            device_id: None,
            device_type,
        }
    }

    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
}

pub trait Driver: Send + Sync {
    fn info(&self) -> &DriverInfo;

    fn state(&self) -> DriverState;

    fn init(&mut self) -> Result<(), DriverError> {
        if self.state() != DriverState::Uninitialized {
            return Err(DriverError::InvalidState);
        }
        Ok(())
    }

    fn start(&mut self) -> Result<(), DriverError> {
        if self.state() != DriverState::Ready {
            return Err(DriverError::InvalidState);
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), DriverError> {
        if self.state() != DriverState::Running {
            return Err(DriverError::InvalidState);
        }
        Ok(())
    }

    fn suspend(&mut self) -> Result<(), DriverError> {
        if self.state() != DriverState::Running {
            return Err(DriverError::InvalidState);
        }
        Ok(())
    }

    fn resume(&mut self) -> Result<(), DriverError> {
        if self.state() != DriverState::Suspended {
            return Err(DriverError::InvalidState);
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), DriverError> {
        match self.state() {
            DriverState::Running | DriverState::Suspended | DriverState::Ready => Ok(()),
            _ => Err(DriverError::InvalidState),
        }
    }

    fn reset(&mut self) -> Result<(), DriverError> {
        Ok(())
    }

    fn status(&self) -> DriverStatus {
        DriverStatus {
            info: self.info().clone(),
            state: self.state(),
            status: DeviceStatus::Online,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DriverStatus {
    pub info: DriverInfo,
    pub state: DriverState,
    pub status: DeviceStatus,
}

impl DriverStatus {
    pub fn is_healthy(&self) -> bool {
        self.status == DeviceStatus::Online
            && matches!(self.state, DriverState::Running | DriverState::Ready)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    NotFound,
    AlreadyExists,
    InvalidState,
    IoError,
    Timeout,
    Busy,
    NotSupported,
    PermissionDenied,
    OutOfMemory,
    InvalidParameter,
    Unknown,
}

impl core::fmt::Display for DriverError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DriverError::NotFound => write!(f, "Driver not found"),
            DriverError::AlreadyExists => write!(f, "Driver already exists"),
            DriverError::InvalidState => write!(f, "Invalid driver state"),
            DriverError::IoError => write!(f, "I/O error"),
            DriverError::Timeout => write!(f, "Operation timed out"),
            DriverError::Busy => write!(f, "Device busy"),
            DriverError::NotSupported => write!(f, "Operation not supported"),
            DriverError::PermissionDenied => write!(f, "Permission denied"),
            DriverError::OutOfMemory => write!(f, "Out of memory"),
            DriverError::InvalidParameter => write!(f, "Invalid parameter"),
            DriverError::Unknown => write!(f, "Unknown error"),
        }
    }
}

#[allow(dead_code)]
pub struct NullDriver {
    info: DriverInfo,
    state: DriverState,
    bytes_written: u64,
    bytes_read: u64,
}

impl NullDriver {
    pub fn new() -> Self {
        NullDriver {
            info: DriverInfo::new("null", DeviceType::Character)
                .with_version("1.0.0")
                .with_author("MicronOS")
                .with_description("Null device driver - discards all writes, returns EOF on read"),
            state: DriverState::Uninitialized,
            bytes_written: 0,
            bytes_read: 0,
        }
    }
}

impl Default for NullDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for NullDriver {
    fn info(&self) -> &DriverInfo {
        &self.info
    }

    fn state(&self) -> DriverState {
        self.state
    }

    fn init(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Uninitialized {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Ready;
        Ok(())
    }

    fn start(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Ready {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Running {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Ready;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), DriverError> {
        self.state = DriverState::Removed;
        Ok(())
    }

    fn status(&self) -> DriverStatus {
        DriverStatus {
            info: self.info.clone(),
            state: self.state,
            status: if self.state == DriverState::Running {
                DeviceStatus::Online
            } else {
                DeviceStatus::Offline
            },
        }
    }
}

#[allow(dead_code)]
pub struct ZeroDriver {
    info: DriverInfo,
    state: DriverState,
    bytes_read: u64,
}

impl ZeroDriver {
    pub fn new() -> Self {
        ZeroDriver {
            info: DriverInfo::new("zero", DeviceType::Character)
                .with_version("1.0.0")
                .with_author("MicronOS")
                .with_description("Zero device driver - returns infinite zeros on read"),
            state: DriverState::Uninitialized,
            bytes_read: 0,
        }
    }
}

impl Default for ZeroDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for ZeroDriver {
    fn info(&self) -> &DriverInfo {
        &self.info
    }

    fn state(&self) -> DriverState {
        self.state
    }

    fn init(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Uninitialized {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Ready;
        Ok(())
    }

    fn start(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Ready {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Running {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Ready;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), DriverError> {
        self.state = DriverState::Removed;
        Ok(())
    }
}

#[allow(dead_code)]
pub struct RandomDriver {
    info: DriverInfo,
    state: DriverState,
    bytes_read: u64,
}

impl RandomDriver {
    pub fn new() -> Self {
        RandomDriver {
            info: DriverInfo::new("random", DeviceType::Character)
                .with_version("1.0.0")
                .with_author("MicronOS")
                .with_description("Random device driver - returns random bytes on read"),
            state: DriverState::Uninitialized,
            bytes_read: 0,
        }
    }
}

impl Default for RandomDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for RandomDriver {
    fn info(&self) -> &DriverInfo {
        &self.info
    }

    fn state(&self) -> DriverState {
        self.state
    }

    fn init(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Uninitialized {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Ready;
        Ok(())
    }

    fn start(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Ready {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), DriverError> {
        if self.state != DriverState::Running {
            return Err(DriverError::InvalidState);
        }
        self.state = DriverState::Ready;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), DriverError> {
        self.state = DriverState::Removed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_info() {
        let info = DriverInfo::new("test", DeviceType::Character);
        assert_eq!(info.name, "test");
        assert_eq!(info.device_type, DeviceType::Character);
    }

    #[test]
    fn test_driver_state() {
        let driver = NullDriver::new();
        assert_eq!(driver.state(), DriverState::Uninitialized);
    }

    #[test]
    fn test_null_driver_lifecycle() {
        let mut driver = NullDriver::new();

        assert_eq!(driver.state(), DriverState::Uninitialized);

        driver.init().unwrap();
        assert_eq!(driver.state(), DriverState::Ready);

        driver.start().unwrap();
        assert_eq!(driver.state(), DriverState::Running);

        driver.stop().unwrap();
        assert_eq!(driver.state(), DriverState::Ready);

        driver.shutdown().unwrap();
        assert_eq!(driver.state(), DriverState::Removed);
    }

    #[test]
    fn test_driver_error_display() {
        assert_eq!(DriverError::NotFound.to_string(), "Driver not found");
        assert_eq!(
            DriverError::InvalidState.to_string(),
            "Invalid driver state"
        );
    }

    #[test]
    fn test_driver_status_healthy() {
        let driver = NullDriver::new();
        let status = driver.status();
        assert!(!status.is_healthy());
    }

    #[test]
    fn test_driver_status_running() {
        let mut driver = NullDriver::new();
        driver.init().unwrap();
        driver.start().unwrap();

        let status = driver.status();
        assert!(status.is_healthy());
    }
}
