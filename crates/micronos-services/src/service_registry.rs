use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
pub enum RegistryState {
    #[default]
    Registering,
    Running,
    Paused,
    Stopped,
}

pub struct ServiceRegistry {
    pub state: RegistryState,
    pub services: Vec<ServiceDescriptor>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            state: RegistryState::Registering,
            services: Vec::new(),
        }
    }

    pub fn start_services(&mut self) {
        if matches!(self.state, RegistryState::Registering) {
            self.state = RegistryState::Running;
        }
    }

    pub fn pause_services(&mut self) {
        if matches!(self.state, RegistryState::Running) {
            self.state = RegistryState::Paused;
        }
    }

    pub fn resume_services(&mut self) {
        if matches!(self.state, RegistryState::Paused) {
            self.state = RegistryState::Running;
        }
    }

    pub fn stop_services(&mut self) {
        if matches!(self.state, RegistryState::Running) {
            self.state = RegistryState::Stopped;
        }
    }

    pub fn restart_services(&mut self) {
        if matches!(self.state, RegistryState::Stopped) {
            self.state = RegistryState::Running;
        }
    }

    pub fn list_services(&self) -> Vec<String> {
        self.services.iter().map(|s| s.name.clone()).collect()
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, RegistryState::Running)
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceDescriptor {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub auto_start: bool,
}

impl ServiceDescriptor {
    pub fn new(name: &str, version: &str) -> Self {
        ServiceDescriptor {
            name: name.to_string(),
            version: version.to_string(),
            dependencies: Vec::new(),
            auto_start: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ServiceRegistry::new();
        assert!(matches!(registry.state, RegistryState::Registering));
    }

    #[test]
    fn test_service_lifecycle() {
        let mut registry = ServiceRegistry::new();
        registry.start_services();
        assert!(matches!(registry.state, RegistryState::Running));

        registry.pause_services();
        assert!(matches!(registry.state, RegistryState::Paused));

        registry.resume_services();
        assert!(matches!(registry.state, RegistryState::Running));

        registry.stop_services();
        assert!(matches!(registry.state, RegistryState::Stopped));
    }

    #[test]
    fn test_list_services() {
        let registry = ServiceRegistry::new();
        let services = registry.list_services();
        assert!(services.is_empty());
    }
}
