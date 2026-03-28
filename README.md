<p align="center">
  <img src="assets/micronos-logo.svg" alt="MicronOS" width="720" />
</p>

# MicronOS

A Rust-based operating system written from scratch, featuring a modular architecture with support for processes, networking, filesystems, and POSIX compatibility.

## Vision

MicronOS is a **Rust micronation** by Rust programmers, for Rust programmers. Built entirely in Rust, it leverages cutting-edge type-state patterns and modern systems programming to create an OS that's:

- **Memory Safe** - No buffer overflows, use-after-free, or data races by design
- **Innovative** - Type-state patterns for compile-time safety guarantees
- **Testable** - 100% testable code, runnable with `cargo test`
- **Modular** - Composable architecture with 8 crates

## Architecture

```
micronos/
├── crates/
│   ├── micronos-core/       # Types, error handling, traits
│   ├── micronos-kernel/     # Kernel, scheduler, syscall, posix
│   │   ├── kernel/         # MicronKernel implementation
│   │   ├── scheduler/      # Task scheduler
│   │   ├── syscall/        # System call interface
│   │   │   ├── mod.rs     # Dispatcher, context, error types
│   │   │   └── handlers.rs # 7 syscall handler implementations
│   │   └── posix/         # POSIX compatibility layer
│   ├── micronos-net/       # Network stack (TCP/UDP), antenna, p2p
│   ├── micronos-fs/        # VFS, storage
│   ├── micronos-services/   # Health monitor, IPC, logger, config, timer, signals
│   └── micronos-drivers/    # Driver framework
└── micronos-bin/           # Main binary, shell
```

## Features

### Kernel
- Process management (spawn, kill, list)
- Memory management
- Scheduler with priority support
- Boot sequence

### POSIX Compatibility Layer
- Process syscalls: fork, getpid, getppid, getuid, getgid
- File syscalls: open, close, read, write, lseek
- Network syscalls: socket, bind, listen, accept
- Time syscalls: time
- Error handling: errno_name(), is_error_retryable(), is_error_fatal()

### Network Stack
- TCP with full state machine (Closed, Listen, SynSent, Established, etc.)
- UDP datagrams
- Socket abstraction layer
- P2P networking
- Antenna management

### Services
- Health monitoring with recovery
- IPC (inter-process communication)
- Logger with log levels
- Configuration management
- Statistics collection
- Timer management
- Signal handling
- **Event System** - Centralized event bus for inter-component communication

### Driver Framework
- Driver trait with lifecycle (init, start, stop, suspend, resume, shutdown)
- Device types: Character, Block, Network, Virtual, Pseudo
- Built-in drivers: null, zero, random

### Event System
- **EventBus** - Centralized pub/sub event system
- **Event types**:
  - Process: ProcessCreated, ProcessTerminated, ProcessSuspended, ProcessResumed
  - Timer: TimerExpired
  - Network: NetworkConnected, NetworkDisconnected
  - Memory: MemoryAllocated, MemoryFreed
  - Disk: DiskRead, DiskWrite
  - Signal: SignalReceived
  - Custom: IPC (100-102), Timer (200), Driver (300), Config (400), Service (500), Health (501)
- **Event sources**: Kernel, Process, Driver, Network, Timer
- **History**: Configurable event history buffer (default 1000 events)
- **Filtering**: Filter events by type (process, timer, network, memory, disk, signal)

## Shell Commands

```
System Info:    info, status, version, kernel, uptime, top, benchmark
Processes:      ps, spawn, suspend, resume, kill
Filesystem:     ls, mkdir, create, cat, rm
Network:        network, scan, discover, antenna, ping, sockets
System:         log, events, events stats, events filter, events export,
                ipc, channel, config, stats, timers, timer,
                memory, memory alloc, memory free, signals, drivers
Advanced:       syscall, posix
Utilities:       echo, exit/quit
```

## System Calls

### Process (0xxx)
| Number | Name   | Description           |
|--------|--------|----------------------|
| 0     | fork   | Create new process   |
| 4     | getpid | Get process ID      |
| 5     | getppid| Get parent PID      |
| 6     | getuid | Get user ID         |
| 7     | getgid | Get group ID        |

### File (2xxx)
| Number | Name   | Description           |
|--------|--------|----------------------|
| 2000   | open   | Open file            |
| 2001   | close  | Close file          |
| 2002   | read   | Read from file      |
| 2003   | write  | Write to file       |
| 2004   | lseek  | Seek in file        |

### Network (3xxx)
| Number | Name   | Description           |
|--------|--------|----------------------|
| 3000   | socket | Create socket        |
| 3001   | bind   | Bind socket         |
| 3003   | listen | Listen for conn.    |
| 3004   | accept | Accept connection   |

### Time (5xxx)
| Number | Name   | Description           |
|--------|--------|----------------------|
| 5000   | time   | Get current time     |

## Building

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run clippy lints
cargo clippy

# Run the OS
cargo run
```

## Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p micronos-kernel
cargo test -p micronos-net

# Run clippy
cargo clippy
```

### Test Results
- **183 tests** across all crates
- **3 doctests** for POSIX module
- **0 clippy warnings**

## Usage Examples

### Shell
```bash
micronos> help
micronos> info
micronos> ps
micronos> top
micronos> benchmark
```

### Syscall Interface
```bash
micronos> syscall info
micronos> syscall invoke 4    # getpid
micronos> syscall invoke 5    # getppid
```

### POSIX
```bash
micronos> posix
```

### Event System
```bash
micronos> events              # Show event history
micronos> events list         # List all events
micronos> events stats        # Show event statistics
micronos> events filter process  # Filter by type
micronos> events filter timer    # Filter by type
micronos> events filter network   # Filter by type
micronos> events by kernel     # Filter by source
micronos> events by process    # Filter by source
micronos> events export       # Export events (CSV)
micronos> events clear        # Clear event history
```

### Memory & IPC
```bash
micronos> memory alloc 1024   # Allocate 1024 bytes
micronos> memory free 0x10000 1024  # Free memory region
micronos> channel create test  # Create IPC channel
micronos> channel 0 send hello  # Send message
micronos> channel 0 recv       # Receive message
micronos> timer create mytimer 5000  # Create timer (5s)
```

## Error Codes

The system uses POSIX-compatible error codes:
- EPERM (1): Operation not permitted
- ENOENT (2): No such file or directory
- EBADF (9): Bad file descriptor
- ENOMEM (12): Out of memory
- EINVAL (22): Invalid argument
- ENOSYS (38): Function not implemented

## Status

| Component   | Status |
|-------------|--------|
| BUILD       | ✅ Pass |
| TESTS       | ✅ 183 Pass |
| DOCTESTS    | ✅ 3 Pass |
| CLIPPY      | ✅ 0 Warnings |

## License

MIT License

Copyright (c) 2026 AI System  
Based on an original idea by **alisio85**
