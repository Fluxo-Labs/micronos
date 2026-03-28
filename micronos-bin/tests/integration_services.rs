use micronos_services::health_monitor::HealthMonitor;
use micronos_services::health_monitor::HealthState;
use micronos_services::process_manager::PMState;
use micronos_services::process_manager::ProcessManager;
use micronos_services::service_registry::RegistryState;
use micronos_services::service_registry::ServiceRegistry;

#[test]
fn test_process_manager_lifecycle() {
    let mut pm = ProcessManager::new();
    assert!(matches!(pm.state, PMState::Spawning));

    pm.activate();
    assert!(matches!(pm.state, PMState::Active));

    pm.suspend_all();
    assert!(matches!(pm.state, PMState::Suspended));

    pm.resume_all();
    assert!(matches!(pm.state, PMState::Active));
}

#[test]
fn test_process_manager_count() {
    let pm = ProcessManager::new();
    assert_eq!(pm.process_count(), 5);
}

#[test]
fn test_service_registry_lifecycle() {
    let mut registry = ServiceRegistry::new();
    assert!(matches!(registry.state, RegistryState::Registering));

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
fn test_health_monitor_lifecycle() {
    let mut monitor = HealthMonitor::new();
    assert!(matches!(monitor.state, HealthState::Idle));

    monitor.start_monitoring();
    assert!(matches!(monitor.state, HealthState::Monitoring));

    monitor.detect_degradation();
    assert!(matches!(monitor.state, HealthState::Degraded));

    monitor.recover_health();
    assert!(matches!(monitor.state, HealthState::Monitoring));
}

#[test]
fn test_health_monitor_critical_recovery() {
    let mut monitor = HealthMonitor::new();
    monitor.start_monitoring();
    monitor.detect_degradation();
    monitor.detect_critical();
    monitor.begin_recovery();
    monitor.recovery_complete();
    assert!(matches!(monitor.state, HealthState::Monitoring));
}

#[test]
fn test_health_score() {
    let monitor = HealthMonitor::new();
    assert_eq!(monitor.health_score(), 100);
}
