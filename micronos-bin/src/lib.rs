extern crate alloc;

use alloc::string::String;
use alloc::sync::Arc;
use micronos_core::SystemState;
use micronos_core::types::NodeId;
use micronos_drivers::driver::{NullDriver, RandomDriver, ZeroDriver};
use micronos_drivers::manager::DriverManager;
use micronos_fs::storage::StorageManager;
use micronos_fs::vfs::VirtualFileSystem;
use micronos_kernel::events::{EventBus, EventSource, EventType, SystemEventHandler};
use micronos_kernel::posix::PosixLib;
use micronos_kernel::syscall::{
    FileHandler, IpcHandler, MemoryHandler, NetworkHandler, ProcessHandler, SignalHandler,
    SyscallDispatcher, TimeHandler,
};
use micronos_kernel::{boot::BootSequence, kernel::MicronKernel};
use micronos_net::antenna::MicronetAntenna;
use micronos_net::network_stack::NetworkStack;
use micronos_net::network_tools::NetworkTools;
use micronos_net::p2p::P2PStack;
use micronos_services::config::Config;
use micronos_services::health_monitor::HealthMonitor;
use micronos_services::ipc::IpcManager;
use micronos_services::logger::Logger;
use micronos_services::memory::MemoryManager;
use micronos_services::process_manager::ProcessManager;
use micronos_services::service_registry::ServiceRegistry;
use micronos_services::signals::SignalManager;
use micronos_services::stats::StatsCollector;
use micronos_services::timer::TimerManager;
use spin::Mutex;

pub mod shell;

pub struct MicronOS {
    kernel: MicronKernel,
    system_state: SystemState,
    vfs: VirtualFileSystem,
    storage: StorageManager,
    antenna: MicronetAntenna,
    p2p: P2PStack,
    pub process_manager: ProcessManager,
    service_registry: ServiceRegistry,
    health_monitor: HealthMonitor,
    pub logger: Logger,
    pub ipc: IpcManager,
    pub config: Config,
    pub stats: StatsCollector,
    pub timer: TimerManager,
    pub memory: MemoryManager,
    pub signals: SignalManager,
    pub network_tools: NetworkTools,
    pub network_stack: NetworkStack,
    pub driver_manager: DriverManager,
    pub syscall_dispatcher: SyscallDispatcher,
    pub posix: PosixLib,
    pub event_bus: EventBus,
    uptime_ms: u64,
}

impl MicronOS {
    pub fn new() -> Self {
        let mut system = SystemState::new();
        system.boot();
        system.initialize();
        system.complete_boot();

        let driver_manager = DriverManager::new();

        let null_driver: Arc<Mutex<dyn micronos_drivers::driver::Driver>> =
            Arc::new(Mutex::new(NullDriver::new()));
        let zero_driver: Arc<Mutex<dyn micronos_drivers::driver::Driver>> =
            Arc::new(Mutex::new(ZeroDriver::new()));
        let random_driver: Arc<Mutex<dyn micronos_drivers::driver::Driver>> =
            Arc::new(Mutex::new(RandomDriver::new()));

        let _ = driver_manager.register(null_driver);
        let _ = driver_manager.register(zero_driver);
        let _ = driver_manager.register(random_driver);

        let _ = driver_manager.init_all();
        let _ = driver_manager.start_all();

        let mut syscall_dispatcher = SyscallDispatcher::new();
        syscall_dispatcher.register_process_handler(Arc::new(ProcessHandler::new()));
        syscall_dispatcher.register_memory_handler(Arc::new(MemoryHandler::new()));
        syscall_dispatcher.register_file_handler(Arc::new(FileHandler::new()));
        syscall_dispatcher.register_network_handler(Arc::new(NetworkHandler::new()));
        syscall_dispatcher.register_signal_handler(Arc::new(SignalHandler::new()));
        syscall_dispatcher.register_time_handler(Arc::new(TimeHandler::new()));
        syscall_dispatcher.register_ipc_handler(Arc::new(IpcHandler::new()));

        let posix = PosixLib::new(Arc::new(syscall_dispatcher.clone()));

        MicronOS {
            kernel: MicronKernel::new(),
            system_state: system,
            vfs: VirtualFileSystem::new(),
            storage: StorageManager::new(1_000_000_000),
            antenna: MicronetAntenna::new(),
            p2p: P2PStack::new(NodeId::default()),
            process_manager: ProcessManager::new(),
            service_registry: ServiceRegistry::new(),
            health_monitor: HealthMonitor::new(),
            logger: Logger::new(),
            ipc: IpcManager::new(),
            config: Config::new(),
            stats: StatsCollector::new(),
            timer: TimerManager::new(),
            memory: MemoryManager::new(256 * 1024 * 1024),
            signals: SignalManager::new(),
            network_tools: NetworkTools::new(),
            network_stack: NetworkStack::new(),
            driver_manager,
            syscall_dispatcher,
            posix,
            event_bus: EventBus::new(1000),
            uptime_ms: 0,
        }
    }

    pub fn boot(&mut self) -> Result<(), &'static str> {
        self.logger.set_silent(true);
        self.logger
            .info("BOOT", "Starting MicronOS boot sequence...");

        self.event_bus
            .subscribe(Arc::new(SystemEventHandler::new()));

        BootSequence::run()?;
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::MemoryAllocated,
            EventSource::Kernel,
            0,
        ));
        self.logger.info("KERNEL", "Kernel initialized");

        self.kernel.initialize();
        self.logger.info("KERNEL", "Kernel started");

        self.system_state.boot();
        self.system_state.initialize();
        self.system_state.complete_boot();
        self.logger.info("SYSTEM", "System state: Running");

        self.storage.initialize();
        self.storage.go_online();
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::DiskRead,
            EventSource::Kernel,
            1,
        ));
        self.logger.info("STORAGE", "Storage online");

        self.process_manager.activate();
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            2,
        ));
        self.logger.info(
            "PROCESS",
            &alloc::format!("{} processes running", self.process_manager.process_count()),
        );

        self.service_registry.start_services();
        self.logger.info("SERVICES", "Service registry started");
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::Custom(500),
            EventSource::Kernel,
            6,
        ));

        self.health_monitor.start_monitoring();
        self.logger.info("HEALTH", "Health monitoring active");
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::Custom(501),
            EventSource::Kernel,
            7,
        ));

        self.vfs.mount();
        self.vfs.complete_mount();
        self.logger.info("VFS", "Virtual filesystem mounted");

        let driver_count = self.driver_manager.list_drivers().len();
        self.logger.info(
            "DRIVERS",
            &alloc::format!("{} drivers loaded", driver_count),
        );
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::Custom(300),
            EventSource::Driver(0),
            5,
        ));

        let _boot_timer = self.timer.create_timer("boot_timer", 5000);
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::TimerExpired,
            EventSource::Timer,
            3,
        ));
        self.logger.info("TIMER", "Timer system initialized");

        self.logger.info(
            "CONFIG",
            &alloc::format!("Loaded {} configuration keys", self.config.keys().len()),
        );

        self.logger.info("SIGNALS", "Signal handler registered");
        self.event_bus.publish(micronos_kernel::events::Event::new(
            EventType::MemoryAllocated,
            EventSource::Kernel,
            4,
        ));
        self.logger.info("MEMORY", "Memory manager initialized");
        self.logger.info("NETWORK", "Network tools loaded");

        self.logger.info("BOOT", "MicronOS boot complete!");
        self.logger.set_silent(false);
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.logger
            .info("SHUTDOWN", "Initiating system shutdown...");
        self.health_monitor.stop_monitoring();
        self.logger.info("HEALTH", "Health monitor stopped");
        self.process_manager.cleanup();
        self.logger.info("PROCESS", "Processes terminated");
        self.service_registry.stop_services();
        self.logger.info("SERVICES", "Services stopped");
        self.vfs.unmount();
        self.logger.info("VFS", "Filesystem unmounted");
        self.storage.take_offline();
        self.logger.info("STORAGE", "Storage offline");
        let _ = self.kernel.stop();
        self.logger.info("SHUTDOWN", "Shutdown complete");
    }

    pub fn tick(&mut self, delta_ms: u64) {
        self.uptime_ms += delta_ms;
        self.timer.tick(delta_ms);

        let stats = micronos_services::stats::SystemStats {
            cpu_usage: 25.0 + (self.uptime_ms as f64 % 20.0),
            memory_used: self.memory.used_memory() + (self.uptime_ms / 100) % 1000,
            memory_total: self.memory.total_memory(),
            storage_used: self.storage.used(),
            storage_total: self.storage.total(),
            process_count: self.process_manager.process_count(),
            network_packets_in: self.uptime_ms / 1000,
            network_packets_out: self.uptime_ms / 1200,
            uptime_ms: self.uptime_ms,
        };
        self.stats.update_stats(stats);
    }

    pub fn status(&self) -> OsStatus {
        OsStatus {
            kernel: self.kernel.info(),
            system_state: format!("{:?}", self.system_state),
            uptime_ms: self.uptime_ms,
            memory_used: self.memory.used_memory(),
            memory_total: self.memory.total_memory(),
            process_count: self.process_manager.process_count(),
            health_score: self.health_monitor.health_score(),
            storage_available: self.storage.available(),
            p2p_peers: self.p2p.peer_count(),
            antenna_connected: self.antenna.is_connected(),
            log_entries: self.logger.entries_count(),
            ipc_channels: self.ipc.channel_count(),
            config_keys: self.config.keys().len(),
            timer_count: self.timer.timer_count(),
            running_timers: self.timer.running_count(),
            memory_regions: self.memory.get_regions().len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OsStatus {
    pub kernel: micronos_kernel::kernel::KernelInfo,
    pub system_state: String,
    pub uptime_ms: u64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub process_count: u32,
    pub health_score: u8,
    pub storage_available: u64,
    pub p2p_peers: usize,
    pub antenna_connected: bool,
    pub log_entries: usize,
    pub ipc_channels: usize,
    pub config_keys: usize,
    pub timer_count: usize,
    pub running_timers: usize,
    pub memory_regions: usize,
}

impl Default for MicronOS {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() {
    let mut os = MicronOS::new();

    println!("MicronOS v{} - A Rust Micronation", MicronKernel::VERSION);
    println!("====================================\n");

    if let Err(e) = os.boot() {
        eprintln!("Boot failed: {}", e);
        return;
    }

    println!("System booted successfully!\n");
    println!("Type 'help' for available commands.\n");

    shell::run(&mut os);
}
