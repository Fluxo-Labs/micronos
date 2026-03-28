extern crate alloc;

pub mod device;
pub mod driver;
pub mod manager;

pub use device::{DeviceId, DeviceStatus, DeviceType};
pub use driver::{Driver, DriverInfo};
pub use manager::DriverManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id() {
        let id = DeviceId::new(0, 0);
        assert_eq!(id.major(), 0);
        assert_eq!(id.minor(), 0);
        assert_eq!(id.device_type(), DeviceType::Character);
    }

    #[test]
    fn test_device_id_format() {
        let id = DeviceId::new(1, 0);
        assert_eq!(id.to_string(), "b1:0");

        let id = DeviceId::new(0, 0);
        assert_eq!(id.to_string(), "c0:0");
    }

    #[test]
    fn test_driver_manager() {
        let manager = DriverManager::new();
        assert_eq!(manager.driver_count(), 0);
        assert!(manager.list_devices().is_empty());
    }

    #[test]
    fn test_device_status() {
        assert_eq!(DeviceStatus::Offline as u8, 0);
        assert_eq!(DeviceStatus::Online as u8, 1);
    }
}
