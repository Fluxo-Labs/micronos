extern crate alloc;

use crate::MicronOS;
use alloc::string::String;
use micronos_core::types::ProcessId;
use micronos_kernel::events::{Event, EventSource, EventType};

pub fn run(os: &mut MicronOS) {
    os.tick(100);

    loop {
        print!("micronos> ");
        std::io::Write::flush(&mut std::io::stdout()).ok();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }

        os.tick(10);

        let cmd = input.trim();
        if cmd.is_empty() {
            continue;
        }

        match cmd {
            "help" => print_help(),
            "status" => print_status(os),
            "ps" => print_processes(os),
            "health" => print_health(os),
            "storage" => print_storage(os),
            "network" => print_network(os),
            "scan" => scan_networks(os),
            "discover" => discover_peers(os),
            "ls" => list_files(os),
            "info" => print_info(os),
            "uptime" => print_uptime(os),
            "version" => print_version(os),
            "kernel" => print_kernel_info(os),
            "top" => top(os),
            "log" => print_log(os),
            "log clear" => clear_log(os),
            "events" => print_events(os),
            "events clear" => {
                os.event_bus.clear_history();
                println!("Event history cleared");
            }
            "ipc" => print_ipc(os),
            "config" => print_config(os),
            "stats" => print_stats(os),
            "timers" => print_timers(os),
            "memory" => print_memory(os),
            "signals" => print_signals(os),
            "drivers" => print_drivers(os),
            "sockets" => print_sockets(os),
            "syscall" => print_syscall_info(os),
            "posix" => print_posix_info(os),
            "benchmark" => run_benchmark(os),
            "ping" => run_ping(os),
            "echo" => {
                println!("Echo: Type 'echo <message>'");
            }
            "exit" | "quit" => {
                println!("Shutting down MicronOS...");
                os.shutdown();
                println!("Goodbye!");
                break;
            }
            _ => {
                if cmd.starts_with("mkdir ") {
                    let dirname = cmd.trim_start_matches("mkdir ");
                    create_dir(os, dirname);
                } else if cmd.starts_with("cat ") {
                    let filename = cmd.trim_start_matches("cat ");
                    read_file(os, filename);
                } else if cmd.starts_with("rm ") {
                    let filename = cmd.trim_start_matches("rm ");
                    remove_file(os, filename);
                } else if cmd.starts_with("create ") {
                    let filename = cmd.trim_start_matches("create ");
                    create_file(os, filename);
                } else if cmd.starts_with("echo ") {
                    let msg = cmd.trim_start_matches("echo ");
                    println!("{}", msg);
                } else if cmd.starts_with("kill ") {
                    let pid = cmd.trim_start_matches("kill ");
                    kill_process(os, pid);
                } else if cmd.starts_with("spawn ") {
                    let name = cmd.trim_start_matches("spawn ");
                    spawn_process(os, name);
                } else if cmd.starts_with("suspend ") {
                    let pid_str = cmd.trim_start_matches("suspend ");
                    suspend_process(os, pid_str);
                } else if cmd.starts_with("resume ") {
                    let pid_str = cmd.trim_start_matches("resume ");
                    resume_process(os, pid_str);
                } else if cmd.starts_with("channel ") {
                    let args = cmd.trim_start_matches("channel ");
                    handle_ipc_command(os, args);
                } else if cmd.starts_with("timer ") {
                    let args = cmd.trim_start_matches("timer ");
                    handle_timer_command(os, args);
                } else if cmd.starts_with("config ") {
                    let args = cmd.trim_start_matches("config ");
                    handle_config_command(os, args);
                } else if cmd.starts_with("syscall ") {
                    let args = cmd.trim_start_matches("syscall ");
                    handle_syscall_command(os, args);
                } else if cmd.starts_with("antenna ") {
                    let args = cmd.trim_start_matches("antenna ");
                    handle_antenna_command(os, args);
                } else if cmd.starts_with("events ") {
                    let args = cmd.trim_start_matches("events ");
                    handle_events_command(os, args);
                } else if cmd.starts_with("memory ") {
                    let args = cmd.trim_start_matches("memory ");
                    handle_memory_command(os, args);
                } else {
                    println!("Unknown command: {}. Type 'help' for help.", cmd);
                }
            }
        }
    }
}

fn print_help() {
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    MicronOS Shell Commands                   ║
╠════════════════════════════════════════════════════════════════╣
║  System Information                                             ║
║  ──────────────────────────────────────────────────────────   ║
║  help     - Show this help message                            ║
║  status   - Show system status                                 ║
║  info     - Show system information                            ║
║  uptime   - Show system uptime                                ║
║  top      - Show system resource usage                         ║
╠════════════════════════════════════════════════════════════════╣
║  Process Management                                            ║
║  ──────────────────────────────────────────────────────────   ║
║  ps       - List running processes                             ║
║  spawn    - Spawn a new process (spawn <name>)                 ║
║  suspend  - Suspend a process (suspend <pid>)                  ║
║  resume   - Resume a process (resume <pid>)                    ║
║  kill     - Kill a process (kill <pid>)                        ║
╠════════════════════════════════════════════════════════════════╣
║  File Operations                                               ║
║  ──────────────────────────────────────────────────────────   ║
║  ls       - List files in current directory                    ║
║  mkdir    - Create a directory (mkdir <name>)                  ║
║  create   - Create a file (create <filename>)                  ║
║  cat      - Read a file (cat <filename>)                       ║
║  rm       - Remove a file (rm <filename>)                      ║
╠════════════════════════════════════════════════════════════════╣
║  Network Operations                                            ║
║  ──────────────────────────────────────────────────────────   ║
║  network  - Show network status                                ║
║  scan     - Scan for available networks                        ║
║  discover - Discover P2P peers                                 ║
║  antenna  - Antenna commands (connect/disconnect/status)       ║
╠════════════════════════════════════════════════════════════════╣
║  IPC & Logging                                                 ║
║  ──────────────────────────────────────────────────────────   ║
║  log      - Show system log                                   ║
║  log clear - Clear system log                                 ║
║  events   - Show event history                                ║
║  events list   - List events (same as 'events')                ║
║  events stats  - Show event statistics                         ║
║  events filter - Filter events by type                         ║
║  events export - Export events (CSV)                           ║
║  events clear - Clear event history                           ║
║  ipc      - Show IPC channels                                 ║
║  channel  - IPC commands (channel create/recv/send)           ║
╠════════════════════════════════════════════════════════════════╣
║  Configuration & Statistics                                    ║
║  ──────────────────────────────────────────────────────────   ║
║  config   - Show system configuration                         ║
║  stats    - Show system statistics                            ║
║  timers   - Show active timers                                 ║
║  timer    - Timer commands (timer create/start/stop)          ║
╠════════════════════════════════════════════════════════════════╣
║  Memory & Signals                                             ║
║  ──────────────────────────────────────────────────────────   ║
║  memory  - Show memory information                           ║
║  memory alloc <bytes> - Allocate memory                       ║
║  memory free <address> - Free memory                          ║
║  signals - Show registered signal handlers                     ║
║  drivers - Show registered drivers                           ║
╠════════════════════════════════════════════════════════════════╣
║  Network Tools                                                ║
║  ──────────────────────────────────────────────────────────   ║
║  ping    - Ping a node                                      ║
║  sockets - Show active network sockets                      ║
╠════════════════════════════════════════════════════════════════╣
║  Health & Storage                                              ║
║  ──────────────────────────────────────────────────────────   ║
║  health   - Show system health status                          ║
║  storage  - Show storage information                           ║
╠════════════════════════════════════════════════════════════════╣
║  System Control                                                ║
║  ──────────────────────────────────────────────────────────   ║
║  echo     - Echo a message (echo <text>)                      ║
║  exit     - Exit and shutdown the system                       ║
║  quit     - Exit and shutdown the system                       ║
╚════════════════════════════════════════════════════════════════╝
"
    );
}

fn print_status(os: &MicronOS) {
    let status = os.status();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    MicronOS System Status                    ║
╠════════════════════════════════════════════════════════════════╣
║  Kernel:       {} v{}                              ║",
        status.kernel.name, status.kernel.version
    );
    println!(
        "║  State:        {}                             ║",
        status.system_state
    );
    println!(
        "║  Uptime:       {} ms                           ║",
        status.uptime_ms
    );
    println!(
        "║  Processes:    {:>4}                                      ║",
        status.process_count
    );
    println!(
        "║  Health:      {:>4}%                                     ║",
        status.health_score
    );
    println!(
        "║  P2P Peers:   {:>4}                                      ║",
        status.p2p_peers
    );
    println!(
        "║  Storage:     {} bytes free              ║",
        status.storage_available
    );
    println!(
        "║  Log Entries: {:>4}                                      ║",
        status.log_entries
    );
    println!(
        "║  IPC Channels: {:>4}                                      ║",
        status.ipc_channels
    );
    println!(
        "║  Config Keys: {:>4}                                      ║",
        status.config_keys
    );
    println!(
        "║  Timers:      {:>4} ({:>3} running)            ║",
        status.timer_count, status.running_timers
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_info(os: &MicronOS) {
    let status = os.status();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                   MicronOS System Information                 ║
╠════════════════════════════════════════════════════════════════╣
║  OS Name:       MicronOS                                    ║
║  Version:       {}                                          ║
║  Architecture:   Rust-based Micronation OS                     ║
║  Kernel:        MicronKernel                                 ║
║  Filesystem:    Virtual Filesystem (VFS)                      ║
║  Network:       P2P Mesh + WiFi Antenna                      ║
║  Memory:        {:>15} bytes total                    ║
║  Storage:      {:>15} bytes total               ║",
        status.kernel.version, status.memory_total, status.memory_total
    );
    println!(
        "║  IPC Channels: {:>4} active                                ║",
        status.ipc_channels
    );
    println!(
        "║  Log Entries:  {:>4}                                       ║",
        status.log_entries
    );
    println!(
        "║  Config Keys:  {:>4}                                       ║",
        status.config_keys
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_uptime(os: &MicronOS) {
    let ms = os.status().uptime_ms;
    let secs = ms / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    println!(
        "System uptime: {} ms ({}:{:02}:{:02})",
        ms,
        hours,
        mins % 60,
        secs % 60
    );
}

fn print_version(_os: &MicronOS) {
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                     MicronOS Version Info                    ║
╠════════════════════════════════════════════════════════════════╣
║  MicronOS Kernel   v0.1.0                                    ║
║  Build Target     x86_64-pc-windows-msvc                   ║
║  Rust Edition     2024                                      ║
║  Build Profile    dev                                       ║
╠════════════════════════════════════════════════════════════════╣
║  Crates:                                                    ║
║  ──────────────────────────────────────────────────────────  ║
║  • micronos-core      - Types, error handling, traits      ║
║  • micronos-kernel    - Kernel, scheduler, syscall, posix  ║
║  • micronos-net       - Network stack, antenna, p2p        ║
║  • micronos-fs        - Virtual filesystem, storage        ║
║  • micronos-services  - Health monitor, IPC, logger, etc.  ║
║  • micronos-drivers   - Driver framework                   ║
╚════════════════════════════════════════════════════════════════╝
"
    );
}

fn print_kernel_info(os: &MicronOS) {
    let kernel = os.kernel.info();
    let status = os.status();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                     Kernel Information                       ║
╠════════════════════════════════════════════════════════════════╣
║  Name:        {}                                        ║
║  Version:     {}                                         ║
║  State:       {:?}                                        ║
║  Health:      {}%                                        ║
║  Processes:   {}                                         ║
║  Memory Used: {:>6} KB / {:>6} KB                         ║
╚════════════════════════════════════════════════════════════════╝
",
        kernel.name,
        kernel.version,
        status.system_state,
        status.health_score,
        status.process_count,
        status.memory_used / 1024,
        status.memory_total / 1024
    );
}

fn print_processes(os: &MicronOS) {
    let processes = os.process_manager.list_processes();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  PID  │ Name              │ State      │ Priority            ║
╠════════════════════════════════════════════════════════════════╣"
    );
    for proc in processes {
        let state = format!("{:?}", proc.state);
        let prio = format!("{:?}", proc.priority);
        let state_short = if state.len() > 10 {
            &state[..10]
        } else {
            &state
        };
        println!(
            "║  {:>3} │ {:<17} │ {:<10} │ {:<19} ║",
            proc.id.0, proc.name, state_short, prio
        );
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn handle_events_command(os: &MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();

    match parts.first().copied() {
        Some("list") | None => {
            print_events(os);
        }
        Some("stats") => {
            print_events_stats(os);
        }
        Some("clear") => {
            os.event_bus.clear_history();
            println!("Event history cleared");
        }
        Some("filter") => {
            if parts.len() > 1 {
                let filter_type = parts[1];
                print_events_filtered(os, filter_type);
            } else {
                println!("Usage: events filter <type>");
                println!("Available types: process, timer, network, memory, disk, signal");
            }
        }
        Some("by") => {
            if parts.len() > 1 {
                let source = parts[1];
                print_events_by_source(os, source);
            } else {
                println!("Usage: events by <source>");
                println!("Available sources: kernel, process, driver, network, timer");
            }
        }
        Some("since") => {
            if parts.len() > 1 {
                if let Ok(ts) = parts[1].parse::<u64>() {
                    print_events_since(os, ts);
                } else {
                    println!("Invalid timestamp: {}", parts[1]);
                }
            } else {
                println!("Usage: events since <timestamp>");
                println!("Show events at or after the specified timestamp (ms)");
            }
        }
        Some("until") => {
            if parts.len() > 1 {
                if let Ok(ts) = parts[1].parse::<u64>() {
                    print_events_until(os, ts);
                } else {
                    println!("Invalid timestamp: {}", parts[1]);
                }
            } else {
                println!("Usage: events until <timestamp>");
                println!("Show events at or before the specified timestamp (ms)");
            }
        }
        Some("between") => {
            if parts.len() > 2 {
                if let (Ok(start), Ok(end)) = (parts[1].parse::<u64>(), parts[2].parse::<u64>()) {
                    print_events_between(os, start, end);
                } else {
                    println!("Invalid timestamps: {} {}", parts[1], parts[2]);
                }
            } else {
                println!("Usage: events between <start> <end>");
                println!("Show events within the time range [start, end] (ms)");
            }
        }
        Some("export") => {
            print_events_export(os);
        }
        Some("help") => {
            println!("Events commands:");
            println!("  events list       - Show event history");
            println!("  events stats      - Show event statistics");
            println!("  events rate       - Show event rate");
            println!("  events timeline   - Show event timeline");
            println!(
                "  events filter     - Filter by type (process|timer|network|memory|disk|signal)"
            );
            println!("  events by         - Filter by source (kernel|process|driver|network|timer)");
            println!("  events since      - Events at/after timestamp (events since <ms>)");
            println!("  events until      - Events at/before timestamp (events until <ms>)");
            println!("  events between    - Events in range (events between <start> <end>)");
            println!("  events export     - Export events (CSV format)");
            println!("  events clear      - Clear event history");
        }
        Some("rate") => {
            print_events_rate(os);
        }
        Some("timeline") => {
            print_events_timeline(os);
        }
        _ => {
            print_events(os);
        }
    }
}

fn print_events_stats(os: &MicronOS) {
    use micronos_kernel::events::EventType;

    let history = os.event_bus.history();
    let mut counts: alloc::collections::BTreeMap<&str, usize> = alloc::collections::BTreeMap::new();

    for event in &history {
        let type_name = match event.event_type {
            EventType::ProcessCreated => "ProcessCreated",
            EventType::ProcessTerminated => "ProcessTerminated",
            EventType::ProcessSuspended => "ProcessSuspended",
            EventType::ProcessResumed => "ProcessResumed",
            EventType::TimerExpired => "TimerExpired",
            EventType::SignalReceived => "SignalReceived",
            EventType::NetworkConnected => "NetworkConnected",
            EventType::NetworkDisconnected => "NetworkDisconnected",
            EventType::DiskRead => "DiskRead",
            EventType::DiskWrite => "DiskWrite",
            EventType::MemoryAllocated => "MemoryAllocated",
            EventType::MemoryFreed => "MemoryFreed",
            EventType::Custom(_) => "Custom",
        };
        *counts.entry(type_name).or_insert(0) += 1;
    }

    let total = history.len();

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Event Statistics                          ║
╠════════════════════════════════════════════════════════════════╣
║  Total events: {:>4}                                         ║",
        total
    );

    if counts.is_empty() {
        println!("║                                                              ║");
        println!("║  (No events recorded)                                        ║");
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  Event Type              │ Count                             ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        for (type_name, count) in &counts {
            println!(
                "║  {:<21} │ {:>4}                                 ║",
                type_name, count
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_filtered(os: &MicronOS, filter_type: &str) {
    use micronos_kernel::events::{EventSource, EventType};

    let history = os.event_bus.history();

    let filtered: Vec<_> = history
        .iter()
        .filter(|e| match filter_type {
            "process" => matches!(
                e.event_type,
                EventType::ProcessCreated
                    | EventType::ProcessTerminated
                    | EventType::ProcessSuspended
                    | EventType::ProcessResumed
            ),
            "timer" => matches!(e.event_type, EventType::TimerExpired),
            "network" => matches!(
                e.event_type,
                EventType::NetworkConnected | EventType::NetworkDisconnected
            ),
            "memory" => matches!(
                e.event_type,
                EventType::MemoryAllocated | EventType::MemoryFreed
            ),
            "disk" => matches!(e.event_type, EventType::DiskRead | EventType::DiskWrite),
            "signal" => matches!(e.event_type, EventType::SignalReceived),
            _ => true,
        })
        .collect();

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Events filtered by: {}                                      ║
╠════════════════════════════════════════════════════════════════╣
║  Matching events: {:>4}                                      ║",
        filter_type,
        filtered.len()
    );

    if filtered.is_empty() {
        println!("║                                                              ║");
        println!("║  (No matching events)                                        ║");
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  ID     │ Type                      │ Source │ Timestamp     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        for event in filtered.iter().rev().take(10) {
            let type_name = match event.event_type {
                EventType::ProcessCreated => "ProcessCreated",
                EventType::ProcessTerminated => "ProcessTerminated",
                EventType::ProcessSuspended => "ProcessSuspended",
                EventType::ProcessResumed => "ProcessResumed",
                EventType::TimerExpired => "TimerExpired",
                EventType::SignalReceived => "SignalReceived",
                EventType::NetworkConnected => "NetworkConnected",
                EventType::NetworkDisconnected => "NetworkDisconnected",
                EventType::DiskRead => "DiskRead",
                EventType::DiskWrite => "DiskWrite",
                EventType::MemoryAllocated => "MemoryAllocated",
                EventType::MemoryFreed => "MemoryFreed",
                EventType::Custom(_) => "Custom",
            };

            let source_name = match event.source {
                EventSource::Kernel => "Kernel",
                EventSource::Process(_) => "Process",
                EventSource::Driver(_) => "Driver",
                EventSource::Network => "Network",
                EventSource::Timer => "Timer",
                EventSource::Custom(_) => "Custom",
            };

            println!(
                "║  {:>6} │ {:<24} │ {:<6} │ {:>10} ms    ║",
                event.id, type_name, source_name, event.timestamp
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_by_source(os: &MicronOS, source: &str) {
    use micronos_kernel::events::{EventSource, EventType};

    let history = os.event_bus.history();

    let filtered: Vec<_> = history
        .iter()
        .filter(|e| match source {
            "kernel" => matches!(e.source, EventSource::Kernel),
            "process" => matches!(e.source, EventSource::Process(_)),
            "driver" => matches!(e.source, EventSource::Driver(_)),
            "network" => matches!(e.source, EventSource::Network),
            "timer" => matches!(e.source, EventSource::Timer),
            "custom" => matches!(e.source, EventSource::Custom(_)),
            _ => false,
        })
        .collect();

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Events by source: {}                                      ║
╠════════════════════════════════════════════════════════════════╣
║  Matching events: {:>4}                                      ║",
        source,
        filtered.len()
    );

    if filtered.is_empty() {
        println!("║                                                              ║");
        println!("║  (No matching events)                                        ║");
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  ID     │ Type                      │ Source │ Timestamp     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        for event in filtered.iter().rev().take(10) {
            let type_name = match event.event_type {
                EventType::ProcessCreated => "ProcessCreated",
                EventType::ProcessTerminated => "ProcessTerminated",
                EventType::ProcessSuspended => "ProcessSuspended",
                EventType::ProcessResumed => "ProcessResumed",
                EventType::TimerExpired => "TimerExpired",
                EventType::SignalReceived => "SignalReceived",
                EventType::NetworkConnected => "NetworkConnected",
                EventType::NetworkDisconnected => "NetworkDisconnected",
                EventType::DiskRead => "DiskRead",
                EventType::DiskWrite => "DiskWrite",
                EventType::MemoryAllocated => "MemoryAllocated",
                EventType::MemoryFreed => "MemoryFreed",
                EventType::Custom(_) => "Custom",
            };

            let source_name = match &event.source {
                EventSource::Kernel => "Kernel",
                EventSource::Process(_) => "Process",
                EventSource::Driver(_) => "Driver",
                EventSource::Network => "Network",
                EventSource::Timer => "Timer",
                EventSource::Custom(_) => "Custom",
            };

            println!(
                "║  {:>6} │ {:<24} │ {:<6} │ {:>10} ms    ║",
                event.id, type_name, source_name, event.timestamp
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_since(os: &MicronOS, timestamp: u64) {
    use micronos_kernel::events::{EventSource, EventType};

    let filtered = os.event_bus.history_since(timestamp);
    let history = os.event_bus.history();

    let min_ts = history.first().map(|e| e.timestamp).unwrap_or(0);
    let max_ts = history.last().map(|e| e.timestamp).unwrap_or(0);

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Events since {} ms                                     ║
╠════════════════════════════════════════════════════════════════╣
║  Time range: {} - {} ms                                  ║
║  Matching events: {:>4}                                      ║",
        timestamp,
        min_ts,
        max_ts,
        filtered.len()
    );

    if filtered.is_empty() {
        println!("║                                                              ║");
        println!("║  (No events at or after {})                              ║", timestamp);
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  ID     │ Type                      │ Source │ Timestamp     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        for event in filtered.iter().rev().take(10) {
            let type_name = match event.event_type {
                EventType::ProcessCreated => "ProcessCreated",
                EventType::ProcessTerminated => "ProcessTerminated",
                EventType::ProcessSuspended => "ProcessSuspended",
                EventType::ProcessResumed => "ProcessResumed",
                EventType::TimerExpired => "TimerExpired",
                EventType::SignalReceived => "SignalReceived",
                EventType::NetworkConnected => "NetworkConnected",
                EventType::NetworkDisconnected => "NetworkDisconnected",
                EventType::DiskRead => "DiskRead",
                EventType::DiskWrite => "DiskWrite",
                EventType::MemoryAllocated => "MemoryAllocated",
                EventType::MemoryFreed => "MemoryFreed",
                EventType::Custom(_) => "Custom",
            };

            let source_name = match event.source {
                EventSource::Kernel => "Kernel",
                EventSource::Process(_) => "Process",
                EventSource::Driver(_) => "Driver",
                EventSource::Network => "Network",
                EventSource::Timer => "Timer",
                EventSource::Custom(_) => "Custom",
            };

            println!(
                "║  {:>6} │ {:<24} │ {:<6} │ {:>10} ms    ║",
                event.id, type_name, source_name, event.timestamp
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_until(os: &MicronOS, timestamp: u64) {
    use micronos_kernel::events::{EventSource, EventType};

    let filtered = os.event_bus.history_until(timestamp);
    let history = os.event_bus.history();

    let min_ts = history.first().map(|e| e.timestamp).unwrap_or(0);
    let max_ts = history.last().map(|e| e.timestamp).unwrap_or(0);

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Events until {} ms                                     ║
╠════════════════════════════════════════════════════════════════╣
║  Time range: {} - {} ms                                  ║
║  Matching events: {:>4}                                      ║",
        timestamp,
        min_ts,
        max_ts,
        filtered.len()
    );

    if filtered.is_empty() {
        println!("║                                                              ║");
        println!("║  (No events at or before {})                             ║", timestamp);
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  ID     │ Type                      │ Source │ Timestamp     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        for event in filtered.iter().take(10) {
            let type_name = match event.event_type {
                EventType::ProcessCreated => "ProcessCreated",
                EventType::ProcessTerminated => "ProcessTerminated",
                EventType::ProcessSuspended => "ProcessSuspended",
                EventType::ProcessResumed => "ProcessResumed",
                EventType::TimerExpired => "TimerExpired",
                EventType::SignalReceived => "SignalReceived",
                EventType::NetworkConnected => "NetworkConnected",
                EventType::NetworkDisconnected => "NetworkDisconnected",
                EventType::DiskRead => "DiskRead",
                EventType::DiskWrite => "DiskWrite",
                EventType::MemoryAllocated => "MemoryAllocated",
                EventType::MemoryFreed => "MemoryFreed",
                EventType::Custom(_) => "Custom",
            };

            let source_name = match event.source {
                EventSource::Kernel => "Kernel",
                EventSource::Process(_) => "Process",
                EventSource::Driver(_) => "Driver",
                EventSource::Network => "Network",
                EventSource::Timer => "Timer",
                EventSource::Custom(_) => "Custom",
            };

            println!(
                "║  {:>6} │ {:<24} │ {:<6} │ {:>10} ms    ║",
                event.id, type_name, source_name, event.timestamp
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_between(os: &MicronOS, start: u64, end: u64) {
    use micronos_kernel::events::{EventSource, EventType};

    let filtered = os.event_bus.history_by_time_range(start, end);
    let history = os.event_bus.history();

    let min_ts = history.first().map(|e| e.timestamp).unwrap_or(0);
    let max_ts = history.last().map(|e| e.timestamp).unwrap_or(0);

    let start_str = if start <= min_ts { "...".to_string() } else { start.to_string() };
    let end_str = if end >= max_ts { "...".to_string() } else { end.to_string() };

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Events between {} - {} ms                              ║
╠════════════════════════════════════════════════════════════════╣
║  Time range: {} - {} ms                                  ║
║  Matching events: {:>4}                                      ║",
        start_str,
        end_str,
        min_ts,
        max_ts,
        filtered.len()
    );

    if filtered.is_empty() {
        println!("║                                                              ║");
        println!("║  (No events in range [{}, {}])                           ║", start, end);
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  ID     │ Type                      │ Source │ Timestamp     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        for event in filtered.iter().take(10) {
            let type_name = match event.event_type {
                EventType::ProcessCreated => "ProcessCreated",
                EventType::ProcessTerminated => "ProcessTerminated",
                EventType::ProcessSuspended => "ProcessSuspended",
                EventType::ProcessResumed => "ProcessResumed",
                EventType::TimerExpired => "TimerExpired",
                EventType::SignalReceived => "SignalReceived",
                EventType::NetworkConnected => "NetworkConnected",
                EventType::NetworkDisconnected => "NetworkDisconnected",
                EventType::DiskRead => "DiskRead",
                EventType::DiskWrite => "DiskWrite",
                EventType::MemoryAllocated => "MemoryAllocated",
                EventType::MemoryFreed => "MemoryFreed",
                EventType::Custom(_) => "Custom",
            };

            let source_name = match event.source {
                EventSource::Kernel => "Kernel",
                EventSource::Process(_) => "Process",
                EventSource::Driver(_) => "Driver",
                EventSource::Network => "Network",
                EventSource::Timer => "Timer",
                EventSource::Custom(_) => "Custom",
            };

            println!(
                "║  {:>6} │ {:<24} │ {:<6} │ {:>10} ms    ║",
                event.id, type_name, source_name, event.timestamp
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_rate(os: &MicronOS) {
    use micronos_kernel::events::EventType;

    let history = os.event_bus.history();
    let total = history.len();

    if total == 0 {
        println!(
            "
╔════════════════════════════════════════════════════════════════╗
║                    Event Rate                               ║
╠════════════════════════════════════════════════════════════════╣
║  No events recorded. Rate: 0.00 events/sec                  ║
╚════════════════════════════════════════════════════════════════╝"
        );
        return;
    }

    let first_ts = history.first().map(|e| e.timestamp).unwrap_or(0);
    let last_ts = history.last().map(|e| e.timestamp).unwrap_or(0);
    let duration_ms = if last_ts > first_ts {
        last_ts - first_ts
    } else {
        1
    };
    let rate = (total as f64) / (duration_ms as f64 / 1000.0);

    let mut type_counts: alloc::collections::BTreeMap<&str, usize> =
        alloc::collections::BTreeMap::new();
    for event in &history {
        let type_name = match event.event_type {
            EventType::ProcessCreated => "ProcessCreated",
            EventType::ProcessTerminated => "ProcessTerminated",
            EventType::ProcessSuspended => "ProcessSuspended",
            EventType::ProcessResumed => "ProcessResumed",
            EventType::TimerExpired => "TimerExpired",
            EventType::SignalReceived => "SignalReceived",
            EventType::NetworkConnected => "NetworkConnected",
            EventType::NetworkDisconnected => "NetworkDisconnected",
            EventType::DiskRead => "DiskRead",
            EventType::DiskWrite => "DiskWrite",
            EventType::MemoryAllocated => "MemoryAllocated",
            EventType::MemoryFreed => "MemoryFreed",
            EventType::Custom(_) => "Custom",
        };
        *type_counts.entry(type_name).or_insert(0) += 1;
    }

    let top_type = type_counts
        .iter()
        .max_by_key(|(_, c)| *c)
        .map(|(k, _)| *k)
        .unwrap_or("N/A");

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Event Rate                               ║
╠════════════════════════════════════════════════════════════════╣
║  Total Events: {:>4}                                           ║
║  Duration:     {:>6} ms                                        ║
║  Rate:         {:>6.2} events/sec                              ║
║  Top Type:     {:<14} ({:>3} events)                           ║
╠════════════════════════════════════════════════════════════════╣
║  Type Breakdown:                                             ║",
        total,
        duration_ms,
        rate,
        top_type,
        type_counts.get(top_type).unwrap_or(&0)
    );

    for (type_name, count) in type_counts.iter().take(6) {
        let bar_len = (*count).min(20);
        let bar: String = "█".repeat(bar_len);
        println!("║  {:<14} {:>4}  {}   ║", type_name, count, bar);
    }

    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_timeline(os: &MicronOS) {
    use micronos_kernel::events::EventType;

    let history = os.event_bus.history();

    if history.is_empty() {
        println!(
            "
╔════════════════════════════════════════════════════════════════╗
║                    Event Timeline                          ║
╠════════════════════════════════════════════════════════════════╣
║  No events recorded.                                       ║
╚════════════════════════════════════════════════════════════════╝"
        );
        return;
    }

    let min_ts = history.iter().map(|e| e.timestamp).min().unwrap_or(0);
    let max_ts = history.iter().map(|e| e.timestamp).max().unwrap_or(0);

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Event Timeline                          ║
╠════════════════════════════════════════════════════════════════╣"
    );

    let events: Vec<_> = history.iter().take(20).collect();

    for (idx, event) in events.iter().enumerate() {
        let type_char = match event.event_type {
            EventType::ProcessCreated => "+",
            EventType::ProcessTerminated => "-",
            EventType::ProcessSuspended => "s",
            EventType::ProcessResumed => "r",
            EventType::TimerExpired => "T",
            EventType::SignalReceived => "!",
            EventType::NetworkConnected => "C",
            EventType::NetworkDisconnected => "D",
            EventType::DiskRead => "R",
            EventType::DiskWrite => "W",
            EventType::MemoryAllocated => "A",
            EventType::MemoryFreed => "F",
            EventType::Custom(_) => "?",
        };

        println!(
            "║ {:>3}: [{:>6} ms] {}                                   ║",
            idx + 1,
            event.timestamp,
            type_char
        );
    }

    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  Legend: +Created -Terminated sSuspended rResumed         ║");
    println!("║          TTimer !Signal CConnected DDisconnected              ║");
    println!("║          RRead WWrite AAlloc FFree ?Custom                   ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!(
        "║  Total events: {:>4}  |  Time range: {} - {} ms       ║",
        history.len(),
        min_ts,
        max_ts
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events_export(os: &MicronOS) {
    use micronos_kernel::events::{EventSource, EventType};

    let history = os.event_bus.history();

    println!("# Event Export (CSV format)");
    println!("# ID,Type,Source,Timestamp,Data_Size");

    for event in &history {
        let type_name = match event.event_type {
            EventType::ProcessCreated => "ProcessCreated",
            EventType::ProcessTerminated => "ProcessTerminated",
            EventType::ProcessSuspended => "ProcessSuspended",
            EventType::ProcessResumed => "ProcessResumed",
            EventType::TimerExpired => "TimerExpired",
            EventType::SignalReceived => "SignalReceived",
            EventType::NetworkConnected => "NetworkConnected",
            EventType::NetworkDisconnected => "NetworkDisconnected",
            EventType::DiskRead => "DiskRead",
            EventType::DiskWrite => "DiskWrite",
            EventType::MemoryAllocated => "MemoryAllocated",
            EventType::MemoryFreed => "MemoryFreed",
            EventType::Custom(_id) => return,
        };

        let source_name = match event.source {
            EventSource::Kernel => "Kernel",
            EventSource::Process(_) => "Process",
            EventSource::Driver(_) => "Driver",
            EventSource::Network => "Network",
            EventSource::Timer => "Timer",
            EventSource::Custom(_) => "Custom",
        };

        println!(
            "{},{},{},{},{}",
            event.id,
            type_name,
            source_name,
            event.timestamp,
            event.data.len()
        );
    }

    println!("# Total: {} events", history.len());
}

fn spawn_process(os: &mut MicronOS, name: &str) {
    match os.process_manager.spawn_process(name) {
        Ok(pid) => {
            println!("Spawned process '{}' with PID {}", name, pid.0);
            os.logger
                .info("SHELL", &format!("Spawned process {} ({})", pid.0, name));
            os.event_bus.publish(Event::new(
                EventType::ProcessCreated,
                EventSource::Process(pid.0 as u32),
                os.status().uptime_ms,
            ));
        }
        Err(e) => {
            println!("Failed to spawn process: {:?}", e);
        }
    }
}

fn kill_process(os: &mut MicronOS, pid_str: &str) {
    if let Ok(pid_num) = pid_str.parse::<u64>() {
        let pid = ProcessId(pid_num);
        match os.process_manager.kill_process(pid) {
            Ok(_) => {
                println!("Killed process {}", pid_num);
                os.logger
                    .info("SHELL", &format!("Killed process {}", pid_num));
                os.event_bus.publish(Event::new(
                    EventType::ProcessTerminated,
                    EventSource::Process(pid_num as u32),
                    os.status().uptime_ms,
                ));
            }
            Err(e) => {
                println!("Failed to kill process: {:?}", e);
            }
        }
    } else {
        println!("Invalid PID: {}", pid_str);
    }
}

fn suspend_process(os: &mut MicronOS, pid_str: &str) {
    if let Ok(pid_num) = pid_str.parse::<u64>() {
        let pid = ProcessId(pid_num);
        match os.process_manager.suspend_process(pid) {
            Ok(_) => {
                println!("Suspended process {}", pid_num);
                os.logger
                    .info("SHELL", &format!("Suspended process {}", pid_num));
                os.event_bus.publish(Event::new(
                    EventType::ProcessSuspended,
                    EventSource::Process(pid_num as u32),
                    os.status().uptime_ms,
                ));
            }
            Err(e) => {
                println!("Failed to suspend process: {:?}", e);
            }
        }
    } else {
        println!("Invalid PID: {}", pid_str);
    }
}

fn resume_process(os: &mut MicronOS, pid_str: &str) {
    if let Ok(pid_num) = pid_str.parse::<u64>() {
        let pid = ProcessId(pid_num);
        match os.process_manager.resume_process(pid) {
            Ok(_) => {
                println!("Resumed process {}", pid_num);
                os.logger
                    .info("SHELL", &format!("Resumed process {}", pid_num));
                os.event_bus.publish(Event::new(
                    EventType::ProcessResumed,
                    EventSource::Process(pid_num as u32),
                    os.status().uptime_ms,
                ));
            }
            Err(e) => {
                println!("Failed to resume process: {:?}", e);
            }
        }
    } else {
        println!("Invalid PID: {}", pid_str);
    }
}

fn print_log(os: &MicronOS) {
    println!("\n{}", os.logger.format_entries());
}

fn clear_log(os: &mut MicronOS) {
    os.logger.clear();
    println!("System log cleared");
}

fn print_ipc(os: &MicronOS) {
    let channels = os.ipc.list_channels();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    IPC Channels                               ║
╠════════════════════════════════════════════════════════════════╣
║  ID    │ Name              │ State   │ Messages               ║
╠════════════════════════════════════════════════════════════════╣"
    );
    if channels.is_empty() {
        println!("║  No active channels                                       ║");
    } else {
        for ch in channels {
            let state = format!("{:?}", ch.state);
            println!(
                "║  {:>4} │ {:<17} │ {:<7} │ {:>18} ║",
                ch.id.0,
                ch.name,
                state,
                ch.len()
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("Total messages in queue: {}", os.ipc.total_messages());
}

fn handle_ipc_command(os: &mut MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.is_empty() {
        println!("Usage: channel create <name> | channel list | channel <id> send <msg>");
        return;
    }

    match parts[0] {
        "create" => {
            if parts.len() > 1 {
                let name = parts[1];
                match os.ipc.create_channel(name, ProcessId(1), ProcessId(2)) {
                    Ok(id) => {
                        println!("Created channel '{}' with ID {}", name, id.0);
                        os.logger
                            .info("IPC", &format!("Created channel {} ({})", id.0, name));
                        os.event_bus.publish(Event::new(
                            EventType::Custom(100),
                            EventSource::Kernel,
                            os.status().uptime_ms,
                        ));
                    }
                    Err(e) => {
                        println!("Failed to create channel: {:?}", e);
                    }
                }
            } else {
                println!("Usage: channel create <name>");
            }
        }
        "list" => {
            print_ipc(os);
        }
        "send" => {
            if parts.len() > 2 {
                if let Ok(channel_id) = parts[1].parse::<u64>() {
                    let msg = parts[2..].join(" ");
                    let message = micronos_services::ipc::Message::new(
                        ProcessId(1),
                        ProcessId(2),
                        msg.into_bytes(),
                    );
                    match os
                        .ipc
                        .send_message(micronos_services::ipc::ChannelId(channel_id), message)
                    {
                        Ok(_) => {
                            println!("Message sent to channel {}", channel_id);
                            os.event_bus.publish(Event::new(
                                EventType::Custom(101),
                                EventSource::Kernel,
                                os.status().uptime_ms,
                            ));
                        }
                        Err(e) => {
                            println!("Failed to send message: {:?}", e);
                        }
                    }
                } else {
                    println!("Invalid channel ID");
                }
            } else {
                println!("Usage: channel <id> send <message>");
            }
        }
        "recv" => {
            if parts.len() > 1 {
                if let Ok(channel_id) = parts[1].parse::<u64>() {
                    match os
                        .ipc
                        .receive_message(micronos_services::ipc::ChannelId(channel_id))
                    {
                        Ok(Some(msg)) => {
                            if let Some(payload) = msg.payload_str() {
                                println!("Received from channel {}: {}", channel_id, payload);
                            }
                            os.event_bus.publish(Event::new(
                                EventType::Custom(102),
                                EventSource::Kernel,
                                os.status().uptime_ms,
                            ));
                        }
                        Ok(None) => {
                            println!("No messages in channel {}", channel_id);
                        }
                        Err(e) => {
                            println!("Failed to receive: {:?}", e);
                        }
                    }
                } else {
                    println!("Invalid channel ID");
                }
            } else {
                println!("Usage: channel <id> recv");
            }
        }
        _ => {
            println!("Unknown IPC command. Use: create, list, send, recv");
        }
    }
}

fn print_config(os: &MicronOS) {
    println!("\n{}", os.config.format_all());
}

fn handle_config_command(os: &mut MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.is_empty() {
        print_config(os);
        return;
    }

    match parts[0] {
        "get" => {
            if parts.len() > 1 {
                if let Some(val) = os.config.get_string(parts[1]) {
                    println!("{} = {}", parts[1], val);
                } else if let Some(val) = os.config.get_integer(parts[1]) {
                    println!("{} = {}", parts[1], val);
                } else if let Some(val) = os.config.get_bool(parts[1]) {
                    println!("{} = {}", parts[1], val);
                } else {
                    println!("Key not found: {}", parts[1]);
                }
            } else {
                println!("Usage: config get <key>");
            }
        }
        "set" => {
            if parts.len() > 2 {
                os.config.set_string(parts[1], parts[2]);
                println!("Set {} = {}", parts[1], parts[2]);
                os.event_bus.publish(Event::new(
                    EventType::Custom(400),
                    EventSource::Kernel,
                    os.status().uptime_ms,
                ));
            } else {
                println!("Usage: config set <key> <value>");
            }
        }
        "list" => {
            for key in os.config.keys() {
                println!("{}", key);
            }
        }
        _ => {
            print_config(os);
        }
    }
}

fn print_stats(os: &MicronOS) {
    println!("\n{}", os.stats.format_current_stats());
}

fn print_timers(os: &MicronOS) {
    let timers = os.timer.list_timers();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Active Timers                             ║
╠════════════════════════════════════════════════════════════════╣
║  ID    │ Name              │ State    │ Progress    │ Remaining ║
╠════════════════════════════════════════════════════════════════╣"
    );
    if timers.is_empty() {
        println!("║  No active timers                                         ║");
    } else {
        for timer in timers {
            let state = format!("{:?}", timer.state);
            let progress = timer.progress_percent();
            println!(
                "║  {:>4} │ {:<17} │ {:<8} │ {:>6.1}%     │ {:>8} ms ║",
                timer.id.0, timer.name, state, progress, timer.remaining_ms
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!(
        "Total timers: {} ({} running)",
        os.timer.timer_count(),
        os.timer.running_count()
    );
}

fn handle_timer_command(os: &mut MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.is_empty() {
        print_timers(os);
        return;
    }

    match parts[0] {
        "create" => {
            if parts.len() > 2 {
                let name = parts[1];
                if let Ok(duration) = parts[2].parse::<u64>() {
                    let id = os.timer.create_timer(name, duration);
                    println!(
                        "Created timer '{}' with ID {} ({} ms)",
                        name, id.0, duration
                    );
                    os.logger
                        .info("TIMER", &format!("Created timer {} ({})", id.0, name));
                    os.event_bus.publish(Event::new(
                        EventType::Custom(200),
                        EventSource::Timer,
                        os.status().uptime_ms,
                    ));
                } else {
                    println!("Invalid duration: {}", parts[2]);
                }
            } else {
                println!("Usage: timer create <name> <duration_ms>");
            }
        }
        "start" => {
            if parts.len() > 1 {
                if let Ok(id) = parts[1].parse::<u64>() {
                    if let Some(timer) = os.timer.start_timer(micronos_services::timer::TimerId(id))
                    {
                        println!("Started timer {} ({})", id, timer.name);
                    } else {
                        println!("Timer {} not found", id);
                    }
                } else {
                    println!("Invalid timer ID");
                }
            } else {
                println!("Usage: timer start <id>");
            }
        }
        "stop" => {
            if parts.len() > 1 {
                if let Ok(id) = parts[1].parse::<u64>() {
                    if let Some(timer) = os.timer.stop_timer(micronos_services::timer::TimerId(id))
                    {
                        println!("Stopped timer {} ({})", id, timer.name);
                    } else {
                        println!("Timer {} not found", id);
                    }
                } else {
                    println!("Invalid timer ID");
                }
            } else {
                println!("Usage: timer stop <id>");
            }
        }
        "list" => {
            print_timers(os);
        }
        _ => {
            print_timers(os);
        }
    }
}

fn print_health(os: &MicronOS) {
    let score = os.health_monitor.health_score();
    let state = format!("{:?}", os.health_monitor.state);

    let health_bar = match score {
        90..=100 => "████████████ 100%",
        70..=89 => "██████████░░  80%",
        50..=69 => "████████░░░░  60%",
        30..=49 => "██████░░░░░░  40%",
        _ => "████░░░░░░░░  20%",
    };

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                   System Health Monitor                       ║
╠════════════════════════════════════════════════════════════════╣
║  Health Score: {}%                                          ║
║  State:        {}                                          ║
║  Checks:       {}                                          ║
║  Failures:     {}                                          ║
║                                                                  ║
║  [{}]                                  ║
╚════════════════════════════════════════════════════════════════╝
",
        score, state, os.health_monitor.checks, os.health_monitor.failures, health_bar
    );
}

fn print_storage(os: &MicronOS) {
    let total = os.storage.total();
    let used = os.storage.used();
    let available = os.storage.available();
    let usage = os.storage.usage_percent();

    let used_mb = used as f64 / (1024.0 * 1024.0);
    let total_mb = total as f64 / (1024.0 * 1024.0);
    let available_mb = available as f64 / (1024.0 * 1024.0);

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Storage Manager                            ║
╠════════════════════════════════════════════════════════════════╣
║  Total:        {:>10.1} MB ({:>15} bytes)         ║",
        total_mb, total
    );
    println!(
        "║  Used:         {:>10.1} MB ({:>15} bytes)         ║",
        used_mb, used
    );
    println!(
        "║  Available:    {:>10.1} MB ({:>15} bytes)         ║",
        available_mb, available
    );
    println!(
        "║  Usage:        {:>6.1}%                                ║",
        usage
    );
    println!(
        "║  State:        {:?}                       ║",
        os.storage.state
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_network(os: &MicronOS) {
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Network Status                            ║
╠════════════════════════════════════════════════════════════════╣
║  P2P Status:                                                 ║"
    );
    println!("║    Local Node: {:?}        ║", &os.p2p.local_id.0[..8]);
    println!(
        "║    Peers:       {:>3}                                        ║",
        os.p2p.peer_count()
    );
    println!(
        "║    State:       {:?}                      ║",
        os.p2p.state
    );
    println!("║                                                                  ║");
    println!("║  Antenna Status:                                             ║");
    println!(
        "║    Connected:   {}                                        ║",
        os.antenna.is_connected()
    );
    println!(
        "║    State:       {:?}                   ║",
        os.antenna.state
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn scan_networks(os: &mut MicronOS) {
    println!("Scanning for networks...");
    let channels = os.antenna.scan_channels();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Channel │ Frequency │ SSID            │ Signal            ║
╠════════════════════════════════════════════════════════════════╣"
    );
    for ch in &channels {
        let ssid = ch.ssid.as_deref().unwrap_or("<hidden>");
        println!(
            "║  {:>7} │ {:>9} │ {:<15} │ {:>6.1} dBm         ║",
            ch.freq, ch.freq, ssid, ch.rssi
        );
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("Found {} networks", channels.len());
}

fn discover_peers(os: &mut MicronOS) {
    println!("Starting P2P discovery...");
    match os.p2p.start_discovery() {
        Ok(_) => {
            println!("Discovery started successfully");
            println!("P2P state: {:?}", os.p2p.state);
        }
        Err(e) => {
            println!("Discovery failed: {:?}", e);
        }
    }
}

fn handle_antenna_command(os: &mut MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();

    match parts.first().copied() {
        Some("connect") => {
            println!("Connecting to network...");
            os.antenna.discover();
            let channels = os.antenna.scan_channels();
            if !channels.is_empty() {
                let node_id = micronos_core::types::NodeId::default();
                match os.antenna.connect_to(node_id) {
                    Ok(_) => {
                        println!("Connected successfully");
                        os.event_bus.publish(Event::new(
                            EventType::NetworkConnected,
                            EventSource::Network,
                            os.status().uptime_ms,
                        ));
                    }
                    Err(e) => println!("Connection failed: {:?}", e),
                }
            } else {
                println!("No networks available to connect");
            }
        }
        Some("disconnect") => {
            os.antenna.disconnect();
            println!("Disconnected");
            os.event_bus.publish(Event::new(
                EventType::NetworkDisconnected,
                EventSource::Network,
                os.status().uptime_ms,
            ));
        }
        Some("status") => {
            println!("Antenna Status: {:?}", os.antenna.state);
        }
        _ => {
            println!("Usage: antenna [connect|disconnect|status]");
        }
    }
}

fn list_files(os: &MicronOS) {
    let entries = os.vfs.root_entries();
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║  Name              │ Type │ Size                          ║
╠════════════════════════════════════════════════════════════════╣"
    );
    for entry in &entries {
        let ftype = if entry.is_dir { "DIR" } else { "FILE" };
        println!(
            "║  {:<17} │ {:<4} │ {:>10} bytes               ║",
            entry.name, ftype, entry.size
        );
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn create_dir(os: &mut MicronOS, dirname: &str) {
    println!("Created directory: {}", dirname);
    os.logger
        .info("VFS", &format!("Created directory: {}", dirname));
}

fn create_file(os: &mut MicronOS, filename: &str) {
    println!("Created file: {}", filename);
    os.logger
        .info("VFS", &format!("Created file: {}", filename));
    os.event_bus.publish(Event::new(
        EventType::DiskWrite,
        EventSource::Kernel,
        os.status().uptime_ms,
    ));
}

fn read_file(os: &MicronOS, filename: &str) {
    println!("Reading file: {} (file not found)", filename);
    os.event_bus.publish(Event::new(
        EventType::DiskRead,
        EventSource::Kernel,
        os.status().uptime_ms,
    ));
}

fn remove_file(os: &mut MicronOS, filename: &str) {
    println!("Removed file: {}", filename);
    os.logger
        .info("VFS", &format!("Removed file: {}", filename));
    os.event_bus.publish(Event::new(
        EventType::DiskWrite,
        EventSource::Kernel,
        os.status().uptime_ms,
    ));
}

fn top(os: &MicronOS) {
    let status = os.status();
    let stats = os.stats.get_current_stats();

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    System Resources                          ║
╠════════════════════════════════════════════════════════════════╣"
    );

    let cpu_bar = match stats.cpu_usage as u32 {
        0..=20 => "█░░░░░░░░░░",
        21..=40 => "██░░░░░░░░░",
        41..=60 => "███░░░░░░░░",
        61..=80 => "████░░░░░░░",
        _ => "█████░░░░░░",
    };
    println!(
        "║  CPU Usage:      {}  {:>5.1}%                         ║",
        cpu_bar, stats.cpu_usage
    );

    let mem_percent = if stats.memory_total > 0 {
        (stats.memory_used as f64 / stats.memory_total as f64) * 100.0
    } else {
        0.0
    };
    let mem_bar = match mem_percent as u32 {
        0..=20 => "█░░░░░░░░░░",
        21..=40 => "██░░░░░░░░░",
        41..=60 => "███░░░░░░░░",
        61..=80 => "████░░░░░░░",
        _ => "█████░░░░░░░",
    };
    println!(
        "║  Memory Usage:    {}  {:>5.1}%                         ║",
        mem_bar, mem_percent
    );

    println!("║  Storage:         █░░░░░░░░░░   10%                          ║");
    println!(
        "║  Health:         ██████████  {}%                          ║",
        status.health_score
    );
    println!("║                                                                  ║");
    println!(
        "║  Running Processes: {:>3}                                ║",
        status.process_count
    );
    println!(
        "║  P2P Connections:   {:>3}                                ║",
        status.p2p_peers
    );
    println!(
        "║  Log Entries:       {:>3}                                ║",
        status.log_entries
    );
    println!(
        "║  Active Timers:     {:>3}                                ║",
        status.timer_count
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_memory(os: &MicronOS) {
    println!("\n{}", os.memory.format_stats());
}

fn handle_memory_command(os: &mut MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();

    match parts.first().copied() {
        Some("alloc") => {
            if parts.len() > 1 {
                if let Ok(size) = parts[1].parse::<u64>() {
                    if let Some(addr) = os.memory.allocate(size) {
                        println!("Allocated {} bytes at 0x{:x}", size, addr);
                        os.event_bus.publish(Event::new(
                            EventType::MemoryAllocated,
                            EventSource::Kernel,
                            os.status().uptime_ms,
                        ));
                    } else {
                        println!("Allocation failed: out of memory");
                    }
                } else {
                    println!("Invalid size. Usage: memory alloc <bytes>");
                }
            } else {
                println!("Usage: memory alloc <bytes>");
            }
        }
        Some("free") => {
            if parts.len() > 2 {
                if let (Ok(addr), Ok(size)) = (parts[1].parse::<usize>(), parts[2].parse::<u64>()) {
                    if os.memory.deallocate(addr, size) {
                        println!("Deallocated region at 0x{:x} ({} bytes)", addr, size);
                        os.event_bus.publish(Event::new(
                            EventType::MemoryFreed,
                            EventSource::Kernel,
                            os.status().uptime_ms,
                        ));
                    } else {
                        println!("Deallocation failed: invalid address");
                    }
                } else {
                    println!("Invalid arguments. Usage: memory free <address> <size>");
                }
            } else {
                println!("Usage: memory free <address> <size>");
            }
        }
        Some(_) | None => {
            print_memory(os);
            println!("\nMemory commands:");
            println!("  memory alloc <bytes> - Allocate memory");
            println!("  memory free <address> <size> - Free memory");
        }
    }
}

fn print_signals(os: &MicronOS) {
    let registered_signals = os.signals.list_signals();
    let pending = os.signals.pending_count();

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Signal Handlers                            ║
╠════════════════════════════════════════════════════════════════╣
║  Registered Signals: {:>3}                                    ║
║  Pending Signals:   {:>3}                                    ║
╠════════════════════════════════════════════════════════════════╣",
        registered_signals.len(),
        pending
    );

    if registered_signals.is_empty() {
        println!("║  No signal handlers registered                             ║");
    } else {
        println!("║  Signal Name  │ Status                                  ║");
        println!("╠════════════════════════════════════════════════════════════════╣");
        for signal in registered_signals {
            println!(
                "║  {:<14} │ Registered                             ║",
                signal.name()
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn run_ping(os: &mut MicronOS) {
    let node = micronos_core::types::NodeId::default();
    println!("Pinging {:?}...", &node.0[..8]);
    println!();

    for seq in 1..=5 {
        let result = os.network_tools.ping(node, seq);
        if result.success {
            println!(
                "64 bytes from {:?}: seq={} ttl={} time={} ms",
                &result.node_id.0[..4],
                result.seq,
                result.ttl,
                result.latency_ms
            );
        } else {
            println!("Request timeout for seq {}", result.seq);
        }
    }

    println!();
    println!("{}", os.network_tools.format_ping_stats());
}

fn print_drivers(os: &MicronOS) {
    println!("\n{}", os.driver_manager.format_status());
}

fn print_sockets(os: &MicronOS) {
    println!("\n{}", os.network_stack.format_status());
}

fn print_syscall_info(_os: &MicronOS) {
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    System Call Interface                      ║
╠════════════════════════════════════════════════════════════════╣
║  Syscall Groups:                                             ║
║  ──────────────────────────────────────────────────────────  ║
║  0xxx - Process syscalls (fork, exec, exit, getpid, etc.)    ║
║  1xxx - Memory syscalls (brk, mmap, munmap, mprotect)        ║
║  2xxx - File syscalls (open, close, read, write, seek)       ║
║  3xxx - Network syscalls (socket, bind, connect, listen)     ║
║  4xxx - Signal syscalls (signal, sigaction, sigprocmask)     ║
║  5xxx - Time syscalls (time, gettimeofday, nanosleep)        ║
║  6xxx - IPC syscalls (msgget, semget, shmget)               ║
╠════════════════════════════════════════════════════════════════╣
║  Usage Examples:                                              ║
║  ──────────────────────────────────────────────────────────  ║
║  syscall info      - Show this information                   ║
║  syscall invoke 4  - Invoke getpid (syscall 1004)            ║
║  syscall invoke 5  - Invoke getppid (syscall 1005)           ║
║  syscall invoke 0 0 - Invoke fork (syscall 1000)             ║
║  syscall list      - List available syscalls                 ║
╚════════════════════════════════════════════════════════════════╝
"
    );
}

fn handle_syscall_command(os: &mut MicronOS, args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.is_empty() {
        print_syscall_info(os);
        return;
    }

    match parts[0] {
        "info" => {
            print_syscall_info(os);
        }
        "list" => {
            println!("\nAvailable System Calls:");
            println!("\nProcess (0xxx):");
            println!("  0000: fork");
            println!("  0001: exec");
            println!("  0002: exit");
            println!("  0004: getpid");
            println!("  0005: getppid");
            println!("  0006: getuid");
            println!("  0007: getgid");
            println!("\nMemory (1xxx):");
            println!("  1000: brk");
            println!("  1001: mmap");
            println!("  1002: munmap");
            println!("  1003: mprotect");
            println!("\nFile (2xxx):");
            println!("  2000: open");
            println!("  2001: close");
            println!("  2002: read");
            println!("  2003: write");
            println!("  2004: lseek");
            println!("\nNetwork (3xxx):");
            println!("  3000: socket");
            println!("  3001: bind");
            println!("  3002: connect");
            println!("  3003: listen");
            println!("  3004: accept");
            println!("\nSignal (4xxx):");
            println!("  4000: signal");
            println!("  4001: sigaction");
            println!("  4002: sigprocmask");
            println!("\nTime (5xxx):");
            println!("  5000: time");
            println!("  5001: gettimeofday");
            println!("  5011: nanosleep");
            println!("\nIPC (6xxx):");
            println!("  6000: msgget");
            println!("  6004: semget");
            println!("  6008: shmget");
        }
        "invoke" => {
            if parts.len() > 1 {
                if let Ok(syscall_num) = parts[1].parse::<usize>() {
                    let syscall_full = 1000 + syscall_num;
                    let context = micronos_kernel::syscall::SyscallContext::new(
                        syscall_full,
                        if parts.len() > 2 {
                            parts[2].parse().unwrap_or(0)
                        } else {
                            0
                        },
                        if parts.len() > 3 {
                            parts[3].parse().unwrap_or(0)
                        } else {
                            0
                        },
                        if parts.len() > 4 {
                            parts[4].parse().unwrap_or(0)
                        } else {
                            0
                        },
                        if parts.len() > 5 {
                            parts[5].parse().unwrap_or(0)
                        } else {
                            0
                        },
                        if parts.len() > 6 {
                            parts[6].parse().unwrap_or(0)
                        } else {
                            0
                        },
                        if parts.len() > 7 {
                            parts[7].parse().unwrap_or(0)
                        } else {
                            0
                        },
                    );

                    let result = os.syscall_dispatcher.dispatch(&context);
                    if result.is_success() {
                        println!("syscall {} -> {} (success)", syscall_full, result.value);
                    } else {
                        let error_str = if let Some(e) = &result.error {
                            format!("{:?}", e)
                        } else {
                            "Unknown error".to_string()
                        };
                        println!(
                            "syscall {} -> {} (error: {})",
                            syscall_full, result.value, error_str
                        );
                    }
                } else {
                    println!("Invalid syscall number: {}", parts[1]);
                }
            } else {
                println!("Usage: syscall invoke <number> [arg0] [arg1] ...");
            }
        }
        _ => {
            print_syscall_info(os);
        }
    }
}

fn print_posix_info(_os: &MicronOS) {
    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    POSIX Compatibility Layer                  ║
╠════════════════════════════════════════════════════════════════╣
║  Process Syscalls (0xxx):                                   ║
║  ──────────────────────────────────────────────────────────  ║
║  fork(0)        - Create new process                      ║
║  getpid(4)      - Get current process ID                  ║
║  getppid(5)     - Get parent process ID                   ║
║  getuid(6)      - Get user ID                             ║
║  getgid(7)      - Get group ID                            ║
╠════════════════════════════════════════════════════════════════╣
║  File Syscalls (2xxx):                                      ║
║  ──────────────────────────────────────────────────────────  ║
║  open(2000)     - Open file                               ║
║  close(2001)   - Close file                              ║
║  read(2002)    - Read from file                          ║
║  write(2003)   - Write to file                           ║
║  lseek(2004)   - Seek in file                            ║
╠════════════════════════════════════════════════════════════════╣
║  Network Syscalls (3xxx):                                    ║
║  ──────────────────────────────────────────────────────────  ║
║  socket(3000)   - Create socket                           ║
║  bind(3001)     - Bind socket                            ║
║  listen(3003)   - Listen for connections                 ║
║  accept(3004)   - Accept connection                      ║
╠════════════════════════════════════════════════════════════════╣
║  Time Syscalls (5xxx):                                      ║
║  ──────────────────────────────────────────────────────────  ║
║  time(5000)     - Get current time                        ║
╠════════════════════════════════════════════════════════════════╣
║  Quick Commands:                                             ║
║  ──────────────────────────────────────────────────────────  ║
║  posix              - Show this info                      ║
║  syscall info        - Show syscall groups                 ║
║  syscall invoke N    - Invoke syscall N                    ║
╚════════════════════════════════════════════════════════════════╝
"
    );
}

fn run_benchmark(os: &mut MicronOS) {
    use core::hint::black_box;

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    MicronOS Benchmark                        ║
╠════════════════════════════════════════════════════════════════╣"
    );

    let iterations = 10000;

    let start = os.uptime_ms;
    for _ in 0..iterations {
        black_box(42u64);
    }
    let loop_time = os.uptime_ms.saturating_sub(start);

    let start = os.uptime_ms;
    for i in 0..iterations {
        black_box(i);
    }
    let arithmetic_time = os.uptime_ms.saturating_sub(start);

    let start = os.uptime_ms;
    let _ = os.process_manager.process_count();
    let _ = os.process_manager.process_count();
    let _ = os.process_manager.process_count();
    let _ = os.process_manager.process_count();
    let _ = os.process_manager.process_count();
    let pm_time = os.uptime_ms.saturating_sub(start);

    let start = os.uptime_ms;
    let _ = os.timer.timer_count();
    let _ = os.timer.timer_count();
    let _ = os.timer.timer_count();
    let _ = os.timer.timer_count();
    let _ = os.timer.timer_count();
    let timer_time = os.uptime_ms.saturating_sub(start);

    let start = os.uptime_ms;
    let _ = os.logger.entries_count();
    let _ = os.logger.entries_count();
    let _ = os.logger.entries_count();
    let _ = os.logger.entries_count();
    let _ = os.logger.entries_count();
    let log_time = os.uptime_ms.saturating_sub(start);

    let start = os.uptime_ms;
    let _ = os.stats.get_current_stats();
    let _ = os.stats.get_current_stats();
    let _ = os.stats.get_current_stats();
    let _ = os.stats.get_current_stats();
    let _ = os.stats.get_current_stats();
    let stats_time = os.uptime_ms.saturating_sub(start);

    println!("║  Test                │ Iterations │ Time (ms) │ ops/sec       ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!(
        "║  Empty Loop         │ {:>9} │ {:>9} │ {:>12} ║",
        iterations,
        loop_time.max(1),
        (iterations as u64 * 1000 / loop_time.max(1))
    );
    println!(
        "║  Arithmetic         │ {:>9} │ {:>9} │ {:>12} ║",
        iterations,
        arithmetic_time.max(1),
        (iterations as u64 * 1000 / arithmetic_time.max(1))
    );
    println!(
        "║  Process Manager    │ {:>9} │ {:>9} │ {:>12} ║",
        5,
        pm_time.max(1),
        (5u64 * 1000 / pm_time.max(1))
    );
    println!(
        "║  Timer Manager      │ {:>9} │ {:>9} │ {:>12} ║",
        5,
        timer_time.max(1),
        (5u64 * 1000 / timer_time.max(1))
    );
    println!(
        "║  Logger             │ {:>9} │ {:>9} │ {:>12} ║",
        5,
        log_time.max(1),
        (5u64 * 1000 / log_time.max(1))
    );
    println!(
        "║  Stats Collector    │ {:>9} │ {:>9} │ {:>12} ║",
        5,
        stats_time.max(1),
        (5u64 * 1000 / stats_time.max(1))
    );
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!(
        "║  System Memory: {:>6} KB / {:>6} KB                      ║",
        os.memory.used_memory() / 1024,
        os.memory.total_memory() / 1024
    );
    println!(
        "║  Processes: {:>6} running                                ║",
        os.process_manager.process_count()
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn print_events(os: &MicronOS) {
    use micronos_kernel::events::{EventSource, EventType};

    let history = os.event_bus.history();

    println!(
        "
╔════════════════════════════════════════════════════════════════╗
║                    Event History                             ║
╠════════════════════════════════════════════════════════════════╣
║  Total events in history: {:>4}                               ║",
        history.len()
    );

    if history.is_empty() {
        println!("║                                                              ║");
        println!("║  (No events recorded)                                        ║");
    } else {
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  ID     │ Type                      │ Source │ Timestamp     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");

        let display_events: Vec<_> = history.iter().rev().take(10).collect();
        for event in display_events {
            let type_name = match event.event_type {
                EventType::ProcessCreated => "ProcessCreated",
                EventType::ProcessTerminated => "ProcessTerminated",
                EventType::ProcessSuspended => "ProcessSuspended",
                EventType::ProcessResumed => "ProcessResumed",
                EventType::TimerExpired => "TimerExpired",
                EventType::SignalReceived => "SignalReceived",
                EventType::NetworkConnected => "NetworkConnected",
                EventType::NetworkDisconnected => "NetworkDisconnected",
                EventType::DiskRead => "DiskRead",
                EventType::DiskWrite => "DiskWrite",
                EventType::MemoryAllocated => "MemoryAllocated",
                EventType::MemoryFreed => "MemoryFreed",
                EventType::Custom(_) => continue,
            };

            let source_name = match event.source {
                EventSource::Kernel => "Kernel",
                EventSource::Process(_) => "Process",
                EventSource::Driver(_) => "Driver",
                EventSource::Network => "Network",
                EventSource::Timer => "Timer",
                EventSource::Custom(_) => "Custom",
            };

            println!(
                "║  {:>6} │ {:<24} │ {:<6} │ {:>10} ms    ║",
                event.id, type_name, source_name, event.timestamp
            );
        }
    }
    println!("╚════════════════════════════════════════════════════════════════╝");
}
