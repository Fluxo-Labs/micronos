extern crate alloc;

use alloc::sync::Arc;
use micronos_kernel::syscall::{
    FileHandler, IpcHandler, MemoryHandler, NetworkHandler, ProcessHandler, SignalHandler,
    SyscallContext, SyscallDispatcher, TimeHandler,
};
use micronos_kernel::{BootSequence, MicronKernel};

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
    assert!(matches!(
        kernel.state,
        micronos_kernel::kernel::KernelState::Initialized { .. }
    ));

    kernel.start().unwrap();
    assert!(matches!(
        kernel.state,
        micronos_kernel::kernel::KernelState::Running { .. }
    ));

    kernel.pause().unwrap();
    assert!(matches!(
        kernel.state,
        micronos_kernel::kernel::KernelState::Paused { .. }
    ));

    kernel.resume().unwrap();
    assert!(matches!(
        kernel.state,
        micronos_kernel::kernel::KernelState::Running { .. }
    ));

    kernel.stop().unwrap();
    assert!(matches!(
        kernel.state,
        micronos_kernel::kernel::KernelState::Stopped
    ));
}

#[test]
fn test_boot_sequence() {
    let result = BootSequence::run();
    assert!(result.is_ok());
}

#[test]
fn test_syscall_dispatcher_creation() {
    let dispatcher = SyscallDispatcher::new();
    let name = dispatcher.syscall_name(0);
    assert!(name.contains("sys_process"));
}

#[test]
fn test_syscall_dispatcher_process_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_process_handler(Arc::new(ProcessHandler::new()));

    let ctx = SyscallContext::new(4, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_dispatcher_memory_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_memory_handler(Arc::new(MemoryHandler::new()));

    let ctx = SyscallContext::new(1000, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_dispatcher_file_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_file_handler(Arc::new(FileHandler::new()));

    let ctx = SyscallContext::new(2000, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_dispatcher_network_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_network_handler(Arc::new(NetworkHandler::new()));

    let ctx = SyscallContext::new(3000, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_dispatcher_signal_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_signal_handler(Arc::new(SignalHandler::new()));

    let ctx = SyscallContext::new(4000, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_dispatcher_time_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_time_handler(Arc::new(TimeHandler::new()));

    let ctx = SyscallContext::new(5000, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_dispatcher_ipc_handlers() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_ipc_handler(Arc::new(IpcHandler::new()));

    let ctx = SyscallContext::new(6000, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(result.is_success());
}

#[test]
fn test_syscall_context_args() {
    let ctx = SyscallContext::new(42, 1, 2, 3, 4, 5, 6);
    assert_eq!(ctx.get_arg(0), 1);
    assert_eq!(ctx.get_arg(1), 2);
    assert_eq!(ctx.get_arg(2), 3);
    assert_eq!(ctx.get_arg(3), 4);
    assert_eq!(ctx.get_arg(4), 5);
    assert_eq!(ctx.get_arg(5), 6);
    assert_eq!(ctx.get_arg(6), 0);
}

#[test]
fn test_syscall_result() {
    let success = micronos_kernel::syscall::SyscallResult::success(42);
    assert!(success.is_success());
    assert_eq!(success.value, 42);
    assert!(success.error.is_none());

    let error = micronos_kernel::syscall::SyscallResult::error(
        micronos_kernel::syscall::SyscallError::ENOENT,
    );
    assert!(!error.is_success());
    assert_eq!(error.value, -1);
    assert!(error.error.is_some());
}

#[test]
fn test_all_handlers_registered() {
    let mut dispatcher = SyscallDispatcher::new();
    dispatcher.register_process_handler(Arc::new(ProcessHandler::new()));
    dispatcher.register_memory_handler(Arc::new(MemoryHandler::new()));
    dispatcher.register_file_handler(Arc::new(FileHandler::new()));
    dispatcher.register_network_handler(Arc::new(NetworkHandler::new()));
    dispatcher.register_signal_handler(Arc::new(SignalHandler::new()));
    dispatcher.register_time_handler(Arc::new(TimeHandler::new()));
    dispatcher.register_ipc_handler(Arc::new(IpcHandler::new()));

    for syscall_group in 0..=6 {
        let syscall_num = syscall_group * 1000;
        let ctx = SyscallContext::new(syscall_num, 0, 0, 0, 0, 0, 0);
        let result = dispatcher.dispatch(&ctx);
        assert!(
            result.is_success(),
            "Syscall group {} should succeed",
            syscall_group
        );
    }
}

#[test]
fn test_syscall_error_handling() {
    let dispatcher = SyscallDispatcher::new();
    let ctx = SyscallContext::new(9999, 0, 0, 0, 0, 0, 0);
    let result = dispatcher.dispatch(&ctx);
    assert!(!result.is_success());
    assert!(result.error.is_some());
}

#[test]
fn test_syscall_context_get_arg() {
    let ctx = SyscallContext::new(100, 10, 20, 30, 40, 50, 60);
    assert_eq!(ctx.get_arg(0), 10);
    assert_eq!(ctx.get_arg(1), 20);
    assert_eq!(ctx.get_arg(2), 30);
    assert_eq!(ctx.get_arg(3), 40);
    assert_eq!(ctx.get_arg(4), 50);
    assert_eq!(ctx.get_arg(5), 60);
    assert_eq!(ctx.get_arg(6), 0);
    assert_eq!(ctx.get_arg(100), 0);
}

#[test]
fn test_posix_errno_functions() {
    use micronos_kernel::posix::{errno_name, is_error_fatal, is_error_retryable};

    assert!(errno_name(11).contains("EAGAIN"));
    assert!(errno_name(1).contains("EPERM"));

    assert!(is_error_retryable(11));
    assert!(is_error_retryable(4));
    assert!(!is_error_retryable(1));

    assert!(is_error_fatal(12));
    assert!(is_error_fatal(13));
    assert!(!is_error_fatal(11));
}

#[test]
fn test_posix_file_descriptor() {
    use micronos_kernel::posix::FileDescriptor;

    let fd = FileDescriptor::new(5);
    assert!(fd.is_some());
    let fd = fd.unwrap();
    assert_eq!(fd.fd(), 5);
    assert!(fd.is_valid());

    let invalid = FileDescriptor::new(-5);
    assert!(invalid.is_none());
}

#[test]
fn test_posix_process_thread_id() {
    use micronos_kernel::posix::{ProcessId, ThreadId};

    let pid = ProcessId::new(12345);
    assert_eq!(pid.value(), 12345);

    let tid = ThreadId::new(67890);
    assert_eq!(tid.value(), 67890);

    let default_pid = ProcessId::default();
    assert_eq!(default_pid.value(), 0);
}

#[test]
fn test_event_bus_integration() {
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType, SystemEventHandler};

    let bus = EventBus::new(100);
    let handler = Arc::new(SystemEventHandler::new());
    let handler_clone = Arc::clone(&handler);
    bus.subscribe(handler);

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        100,
    ));
    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        101,
    ));
    bus.publish(Event::new(EventType::TimerExpired, EventSource::Timer, 102));

    let counts = handler_clone.get_counts();
    assert_eq!(counts.len(), 2);

    let history = bus.history();
    assert_eq!(history.len(), 3);
}

#[test]
fn test_event_bus_history_limit() {
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType};

    let bus = EventBus::new(5);

    for i in 0..10u64 {
        bus.publish(Event::new(
            EventType::Custom(i as u32),
            EventSource::Kernel,
            i,
        ));
    }

    let history = bus.history();
    assert_eq!(history.len(), 5);
    assert_eq!(history[0].timestamp, 5);
    assert_eq!(history[4].timestamp, 9);
}

#[test]
fn test_event_bus_clear_history() {
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType};

    let bus = EventBus::new(100);

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        1,
    ));
    bus.publish(Event::new(EventType::TimerExpired, EventSource::Timer, 2));

    assert_eq!(bus.history().len(), 2);

    bus.clear_history();

    assert_eq!(bus.history().len(), 0);
}

#[test]
fn test_event_types_all_variants() {
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType};

    let bus = EventBus::new(100);

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        1,
    ));
    bus.publish(Event::new(
        EventType::ProcessTerminated,
        EventSource::Kernel,
        2,
    ));
    bus.publish(Event::new(EventType::TimerExpired, EventSource::Timer, 3));
    bus.publish(Event::new(
        EventType::NetworkConnected,
        EventSource::Network,
        4,
    ));
    bus.publish(Event::new(
        EventType::NetworkDisconnected,
        EventSource::Network,
        5,
    ));
    bus.publish(Event::new(EventType::DiskRead, EventSource::Kernel, 6));
    bus.publish(Event::new(EventType::DiskWrite, EventSource::Kernel, 7));
    bus.publish(Event::new(
        EventType::MemoryAllocated,
        EventSource::Kernel,
        8,
    ));
    bus.publish(Event::new(EventType::MemoryFreed, EventSource::Kernel, 9));

    let history = bus.history();
    assert_eq!(history.len(), 9);
}

#[test]
fn test_event_source_variants() {
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType};

    let bus = EventBus::new(100);

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        1,
    ));
    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Process(42),
        2,
    ));
    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Driver(10),
        3,
    ));
    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Network,
        4,
    ));
    bus.publish(Event::new(EventType::ProcessCreated, EventSource::Timer, 5));

    let history = bus.history();
    assert_eq!(history.len(), 5);
}

#[test]
fn test_event_handler_counts() {
    use alloc::sync::Arc;
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType, SystemEventHandler};

    let bus = EventBus::new(100);
    let handler = Arc::new(SystemEventHandler::new());
    let handler_clone = Arc::clone(&handler);
    bus.subscribe(handler);

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        1,
    ));
    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        2,
    ));
    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        3,
    ));
    bus.publish(Event::new(EventType::TimerExpired, EventSource::Timer, 4));

    let counts = handler_clone.get_counts();
    assert_eq!(counts.len(), 2);

    let process_count = counts
        .iter()
        .find(|(t, _)| matches!(t, EventType::ProcessCreated))
        .map(|(_, c)| *c);
    assert_eq!(process_count, Some(3));
}

#[test]
fn test_event_subscription_multiple_handlers() {
    use alloc::sync::Arc;
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType, SystemEventHandler};

    let bus = EventBus::new(100);

    let handler1 = Arc::new(SystemEventHandler::new());
    let handler1_clone = Arc::clone(&handler1);
    let handler2 = Arc::new(SystemEventHandler::new());
    let handler2_clone = Arc::clone(&handler2);

    bus.subscribe(handler1);
    bus.subscribe(handler2);

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        1,
    ));
    bus.publish(Event::new(EventType::TimerExpired, EventSource::Timer, 2));

    let counts1 = handler1_clone.get_counts();
    let counts2 = handler2_clone.get_counts();

    assert_eq!(counts1.len(), 2);
    assert_eq!(counts2.len(), 2);
}

#[test]
fn test_event_unsubscribe() {
    use alloc::sync::Arc;
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType, SystemEventHandler};

    let bus = EventBus::new(100);
    let handler = Arc::new(SystemEventHandler::new());
    let handler_clone = Arc::clone(&handler);

    bus.subscribe(handler);
    bus.unsubscribe(handler_clone.clone());

    bus.publish(Event::new(
        EventType::ProcessCreated,
        EventSource::Kernel,
        1,
    ));

    let counts = handler_clone.get_counts();
    assert_eq!(counts.len(), 0);
}

#[test]
fn test_event_with_data() {
    use micronos_kernel::events::{Event, EventBus, EventSource, EventType};

    let bus = EventBus::new(100);

    let data = vec![1u8, 2, 3, 4];
    let event = Event::new(EventType::Custom(42), EventSource::Kernel, 100).with_data(data.clone());

    bus.publish(event);

    let history = bus.history();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].data, data);
}
