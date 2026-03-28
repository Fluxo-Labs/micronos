use micronos_core::{
    SystemState,
    types::{NodeId, Priority, ProcessId, ThreadId},
};

#[test]
fn test_system_state_machine() {
    let mut system = SystemState::new();
    assert!(matches!(system, SystemState::Off));

    let _ = system.boot();
    assert!(matches!(system, SystemState::Initializing));

    let _ = system.initialize();
    assert!(matches!(system, SystemState::Booting));

    let _ = system.complete_boot();
    assert!(matches!(system, SystemState::Running { uptime_ticks: 0 }));
}

#[test]
fn test_system_state_transitions() {
    let mut system = SystemState::new();
    let result = system.boot();
    assert!(result);
    assert!(matches!(system, SystemState::Initializing));
}

#[test]
fn test_node_id_default() {
    let node_id = NodeId::default();
    assert_eq!(node_id.0, [0u8; 32]);
}

#[test]
fn test_priority_default() {
    let priority = Priority::default();
    assert_eq!(priority, Priority::Idle);
}

#[test]
fn test_process_id_creation() {
    let pid = ProcessId(42);
    assert_eq!(pid.0, 42);
}

#[test]
fn test_thread_id_creation() {
    let tid = ThreadId(123);
    assert_eq!(tid.0, 123);
}
