use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Mutex, RwLock};

use crate::device::{DeviceId, DeviceInfo, DeviceType};
use crate::driver::{Driver, DriverError, DriverInfo, DriverState, DriverStatus};

pub struct DriverManager {
    drivers: RwLock<Vec<RegisteredDriver>>,
    next_id: RwLock<u32>,
}

struct RegisteredDriver {
    #[allow(dead_code)]
    id: u32,
    name: String,
    device_type: DeviceType,
    driver: Arc<Mutex<dyn Driver>>,
}

impl DriverManager {
    pub fn new() -> Self {
        DriverManager {
            drivers: RwLock::new(Vec::new()),
            next_id: RwLock::new(1),
        }
    }

    pub fn register(&self, driver: Arc<Mutex<dyn Driver>>) -> Result<u32, DriverError> {
        let name = driver.lock().info().name.clone();
        let device_type = driver.lock().info().device_type;

        let mut drivers = self.drivers.write();

        if drivers.iter().any(|d| d.name == name) {
            return Err(DriverError::AlreadyExists);
        }

        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;

        drivers.push(RegisteredDriver {
            id,
            name,
            device_type,
            driver,
        });

        Ok(id)
    }

    pub fn unregister(&self, name: &str) -> Result<(), DriverError> {
        let mut drivers = self.drivers.write();
        let pos = drivers
            .iter()
            .position(|d| d.name == name)
            .ok_or(DriverError::NotFound)?;

        let driver = &drivers[pos];
        if driver.driver.lock().state() == DriverState::Running {
            return Err(DriverError::InvalidState);
        }

        drivers.remove(pos);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<Mutex<dyn Driver>>> {
        let drivers = self.drivers.read();
        drivers
            .iter()
            .find(|d| d.name == name)
            .map(|d| d.driver.clone())
    }

    pub fn get_by_device_id(&self, device_id: DeviceId) -> Option<Arc<Mutex<dyn Driver>>> {
        let drivers = self.drivers.read();
        drivers
            .iter()
            .find(|d| {
                d.driver
                    .lock()
                    .info()
                    .device_id
                    .map(|id| id == device_id)
                    .unwrap_or(false)
            })
            .map(|d| d.driver.clone())
    }

    pub fn list_drivers(&self) -> Vec<DriverInfo> {
        let drivers = self.drivers.read();
        drivers
            .iter()
            .map(|d| d.driver.lock().info().clone())
            .collect()
    }

    pub fn list_devices(&self) -> Vec<DeviceInfo> {
        let drivers = self.drivers.read();
        drivers
            .iter()
            .map(|d| {
                let guard = d.driver.lock();
                let info = guard.info();
                DeviceInfo {
                    id: info.device_id.unwrap_or(DeviceId::new(0, 0)),
                    name: info.name.clone(),
                    device_type: info.device_type,
                    status: guard.status().status,
                    size: 0,
                    block_size: 512,
                }
            })
            .collect()
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.read().len()
    }

    pub fn device_count(&self) -> usize {
        self.drivers.read().len()
    }

    pub fn init_all(&self) -> Result<(), DriverError> {
        let drivers = self.drivers.read();
        for driver in drivers.iter() {
            driver.driver.lock().init()?;
        }
        Ok(())
    }

    pub fn start_all(&self) -> Result<(), DriverError> {
        let drivers = self.drivers.read();
        for driver in drivers.iter() {
            driver.driver.lock().start()?;
        }
        Ok(())
    }

    pub fn stop_all(&self) -> Result<(), DriverError> {
        let drivers = self.drivers.read();
        for driver in drivers.iter() {
            driver.driver.lock().stop()?;
        }
        Ok(())
    }

    pub fn shutdown_all(&self) -> Result<(), DriverError> {
        let mut drivers = self.drivers.write();
        for driver in drivers.iter_mut() {
            driver.driver.lock().shutdown()?;
        }
        Ok(())
    }

    pub fn status_all(&self) -> Vec<DriverStatus> {
        let drivers = self.drivers.read();
        drivers.iter().map(|d| d.driver.lock().status()).collect()
    }

    pub fn find_by_type(&self, device_type: DeviceType) -> Vec<Arc<Mutex<dyn Driver>>> {
        let drivers = self.drivers.read();
        drivers
            .iter()
            .filter(|d| d.device_type == device_type)
            .map(|d| d.driver.clone())
            .collect()
    }

    pub fn healthy_count(&self) -> usize {
        let drivers = self.drivers.read();
        drivers
            .iter()
            .filter(|d| d.driver.lock().status().is_healthy())
            .count()
    }

    pub fn format_status(&self) -> String {
        let drivers = self.list_drivers();
        let statuses = self.status_all();

        let mut output = alloc::string::String::from(
            "╔════════════════════════════════════════════════════════════════╗\n",
        );
        output.push_str("║                    Driver Manager                           ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str(&alloc::format!(
            "║  Total Drivers: {:>3}                                              ║\n",
            drivers.len()
        ));
        output.push_str(&alloc::format!(
            "║  Healthy:       {:>3}                                              ║\n",
            self.healthy_count()
        ));
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");

        if drivers.is_empty() {
            output.push_str("║  No drivers registered                                      ║\n");
        } else {
            output.push_str("║  Name            │ Type      │ State                    ║\n");
            output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
            for (i, info) in drivers.iter().enumerate() {
                let status = &statuses[i];
                let state = format!("{:?}", status.state);
                let device_type = format!("{}", info.device_type);
                output.push_str(&alloc::format!(
                    "║  {:<15} │ {:<8} │ {:<24} ║\n",
                    info.name,
                    device_type,
                    state
                ));
            }
        }
        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::NullDriver;

    #[test]
    fn test_driver_manager_creation() {
        let manager = DriverManager::new();
        assert_eq!(manager.driver_count(), 0);
        assert!(manager.list_drivers().is_empty());
    }

    #[test]
    fn test_register_driver() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        let id = manager.register(driver.clone()).unwrap();
        assert_eq!(id, 1);
        assert_eq!(manager.driver_count(), 1);
    }

    #[test]
    fn test_register_duplicate() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        manager.register(driver.clone()).unwrap();
        let result = manager.register(driver);
        assert_eq!(result, Err(DriverError::AlreadyExists));
    }

    #[test]
    fn test_unregister_driver() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        manager.register(driver).unwrap();
        assert_eq!(manager.driver_count(), 1);

        manager.unregister("null").unwrap();
        assert_eq!(manager.driver_count(), 0);
    }

    #[test]
    fn test_get_driver() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        manager.register(driver.clone()).unwrap();

        let found = manager.get("null");
        assert!(found.is_some());
        assert_eq!(found.unwrap().lock().info().name, "null");
    }

    #[test]
    fn test_list_devices() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        manager.register(driver).unwrap();

        let devices = manager.list_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].name, "null");
    }

    #[test]
    fn test_find_by_type() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        manager.register(driver).unwrap();

        let char_devices = manager.find_by_type(DeviceType::Character);
        assert_eq!(char_devices.len(), 1);

        let block_devices = manager.find_by_type(DeviceType::Block);
        assert!(block_devices.is_empty());
    }

    #[test]
    fn test_init_all() {
        let manager = DriverManager::new();
        let driver: Arc<Mutex<dyn Driver>> = Arc::new(Mutex::new(NullDriver::new()));

        manager.register(driver).unwrap();
        manager.init_all().unwrap();

        let status = manager.status_all();
        assert_eq!(status[0].state, DriverState::Ready);
    }
}
