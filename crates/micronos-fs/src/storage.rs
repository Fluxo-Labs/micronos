#[derive(Debug, Clone, Default)]
pub enum StorageState {
    #[default]
    Offline,
    Initializing,
    Online,
    Suspended,
    Faulted,
}

pub struct StorageManager {
    pub state: StorageState,
    pub total_bytes: u64,
    pub used_bytes: u64,
}

impl StorageManager {
    pub const BLOCK_SIZE: usize = 4096;

    pub fn new(total_bytes: u64) -> Self {
        StorageManager {
            state: StorageState::Offline,
            total_bytes,
            used_bytes: 0,
        }
    }

    pub fn initialize(&mut self) {
        if matches!(self.state, StorageState::Offline) {
            self.state = StorageState::Initializing;
        }
    }

    pub fn go_online(&mut self) {
        if matches!(self.state, StorageState::Initializing) {
            self.state = StorageState::Online;
        }
    }

    pub fn suspend(&mut self) {
        if matches!(self.state, StorageState::Online) {
            self.state = StorageState::Suspended;
        }
    }

    pub fn resume(&mut self) {
        if matches!(self.state, StorageState::Suspended) {
            self.state = StorageState::Online;
        }
    }

    pub fn take_offline(&mut self) {
        self.state = StorageState::Offline;
    }

    pub fn fault(&mut self) {
        self.state = StorageState::Faulted;
    }

    pub fn available(&self) -> u64 {
        self.total_bytes - self.used_bytes
    }

    pub fn total(&self) -> u64 {
        self.total_bytes
    }

    pub fn used(&self) -> u64 {
        self.used_bytes
    }

    pub fn usage_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f32 / self.total_bytes as f32) * 100.0
        }
    }

    pub fn is_online(&self) -> bool {
        matches!(self.state, StorageState::Online)
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new(1024 * 1024 * 1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_creation() {
        let storage = StorageManager::new(1_000_000);
        assert!(matches!(storage.state, StorageState::Offline));
    }

    #[test]
    fn test_init_workflow() {
        let mut storage = StorageManager::new(1_000_000);
        storage.initialize();
        assert!(matches!(storage.state, StorageState::Initializing));
        storage.go_online();
        assert!(matches!(storage.state, StorageState::Online));
    }

    #[test]
    fn test_suspend_resume() {
        let mut storage = StorageManager::new(1_000_000);
        storage.initialize();
        storage.go_online();
        storage.suspend();
        assert!(matches!(storage.state, StorageState::Suspended));
        storage.resume();
        assert!(matches!(storage.state, StorageState::Online));
    }

    #[test]
    fn test_available_space() {
        let storage = StorageManager::new(1_000_000);
        assert_eq!(storage.available(), 1_000_000);
    }

    #[test]
    fn test_usage_percent() {
        let storage = StorageManager::new(1_000_000);
        assert_eq!(storage.usage_percent(), 0.0);
    }
}
