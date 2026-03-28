use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeviceType {
    Character = 0,
    Block = 1,
    Network = 2,
    Virtual = 3,
    Pseudo = 4,
}

impl DeviceType {
    pub fn prefix(&self) -> char {
        match self {
            DeviceType::Character => 'c',
            DeviceType::Block => 'b',
            DeviceType::Network => 'n',
            DeviceType::Virtual => 'v',
            DeviceType::Pseudo => 'p',
        }
    }
}

impl core::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeviceType::Character => write!(f, "character"),
            DeviceType::Block => write!(f, "block"),
            DeviceType::Network => write!(f, "network"),
            DeviceType::Virtual => write!(f, "virtual"),
            DeviceType::Pseudo => write!(f, "pseudo"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeviceStatus {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Error = 3,
    Suspended = 4,
}

impl DeviceStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, DeviceStatus::Online)
    }
}

impl core::fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeviceStatus::Offline => write!(f, "offline"),
            DeviceStatus::Online => write!(f, "online"),
            DeviceStatus::Busy => write!(f, "busy"),
            DeviceStatus::Error => write!(f, "error"),
            DeviceStatus::Suspended => write!(f, "suspended"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DeviceId(u32);

impl DeviceId {
    pub fn new(major: u8, minor: u8) -> Self {
        DeviceId(((major as u32) << 8) | (minor as u32))
    }

    pub fn major(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn minor(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn device_type(&self) -> DeviceType {
        match self.major() {
            0 => DeviceType::Character,
            1 => DeviceType::Block,
            2 => DeviceType::Network,
            3 => DeviceType::Virtual,
            _ => DeviceType::Pseudo,
        }
    }
}

impl core::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let prefix = self.device_type().prefix();
        write!(f, "{}{}:{}", prefix, self.major(), self.minor())
    }
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: DeviceId,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub size: u64,
    pub block_size: u32,
}

impl DeviceInfo {
    pub fn new(name: &str, device_type: DeviceType, major: u8, minor: u8) -> Self {
        DeviceInfo {
            id: DeviceId::new(major, minor),
            name: name.to_string(),
            device_type,
            status: DeviceStatus::Offline,
            size: 0,
            block_size: 512,
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_block_size(mut self, block_size: u32) -> Self {
        self.block_size = block_size;
        self
    }

    pub fn with_status(mut self, status: DeviceStatus) -> Self {
        self.status = status;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceRequest {
    pub device_id: DeviceId,
    pub operation: DeviceOperation,
    pub buffer: Option<Vec<u8>>,
    pub offset: u64,
    pub size: usize,
}

impl DeviceRequest {
    pub fn new(device_id: DeviceId, operation: DeviceOperation) -> Self {
        DeviceRequest {
            device_id,
            operation,
            buffer: None,
            offset: 0,
            size: 0,
        }
    }

    pub fn with_buffer(mut self, buffer: Vec<u8>) -> Self {
        self.buffer = Some(buffer);
        self.size = self.buffer.as_ref().map(|b| b.len()).unwrap_or(0);
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceOperation {
    Read,
    Write,
    Sync,
    Flush,
    Trim,
    Ioctl(u32),
}

pub struct DeviceResponse {
    pub success: bool,
    pub bytes_transferred: usize,
    pub error: Option<String>,
}

impl DeviceResponse {
    pub fn success(bytes: usize) -> Self {
        DeviceResponse {
            success: true,
            bytes_transferred: bytes,
            error: None,
        }
    }

    pub fn failure(error: &str) -> Self {
        DeviceResponse {
            success: false,
            bytes_transferred: 0,
            error: Some(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_prefix() {
        assert_eq!(DeviceType::Character.prefix(), 'c');
        assert_eq!(DeviceType::Block.prefix(), 'b');
        assert_eq!(DeviceType::Network.prefix(), 'n');
    }

    #[test]
    fn test_device_id_construction() {
        let id = DeviceId::new(5, 3);
        assert_eq!(id.major(), 5);
        assert_eq!(id.minor(), 3);
    }

    #[test]
    fn test_device_id_to_string() {
        let id = DeviceId::new(1, 0);
        assert_eq!(id.to_string(), "b1:0");

        let id = DeviceId::new(0, 5);
        assert_eq!(id.to_string(), "c0:5");
    }

    #[test]
    fn test_device_status_available() {
        assert!(DeviceStatus::Online.is_available());
        assert!(!DeviceStatus::Offline.is_available());
        assert!(!DeviceStatus::Busy.is_available());
    }

    #[test]
    fn test_device_info_builder() {
        let info = DeviceInfo::new("sda", DeviceType::Block, 8, 0)
            .with_size(1_000_000_000)
            .with_block_size(512)
            .with_status(DeviceStatus::Online);

        assert_eq!(info.name, "sda");
        assert_eq!(info.size, 1_000_000_000);
        assert_eq!(info.block_size, 512);
        assert_eq!(info.status, DeviceStatus::Online);
    }

    #[test]
    fn test_device_request() {
        let req = DeviceRequest::new(DeviceId::new(0, 1), DeviceOperation::Read)
            .with_offset(1024)
            .with_buffer(vec![0u8; 512]);

        assert_eq!(req.offset, 1024);
        assert_eq!(req.size, 512);
    }
}
