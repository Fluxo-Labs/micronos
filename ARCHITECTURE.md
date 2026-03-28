# MicronOS Architecture

MicronOS is a Rust 2024, MIT-licensed operating system project designed as a **Rust micronation**: a community-driven, open-source platform that treats operating-system development as a civic, collaborative endeavor.

This document explains the current architecture and the direction of the project.

## Principles

- **Correctness first**
  - Prefer compile-time guarantees (type-state, invariants, explicit state machines).
- **Testability**
  - Everything must be runnable under `cargo test`.
  - Components should be usable in isolation.
- **Modularity**
  - Clear crate boundaries.
  - Stable public APIs.
- **Observability**
  - Events, logs, stats, and tracing are first-class.

## Workspace layout

```
micronos/
├── crates/
│   ├── micronos-core/        # Core types, errors, traits
│   ├── micronos-kernel/      # Kernel lifecycle, scheduler, syscalls, POSIX layer, events
│   ├── micronos-net/         # Network stack (TCP/UDP), antenna abstraction, P2P
│   ├── micronos-fs/          # VFS + storage model
│   ├── micronos-services/    # System services: memory, IPC, logging, config, health, timers
│   └── micronos-drivers/     # Driver framework + device model
└── micronos-bin/             # Hosted runner + interactive shell
```

## High-level data flow

1. `micronos-bin` builds a `MicronOS` instance.
2. `MicronOS::boot()` orchestrates subsystems:
   - kernel lifecycle
   - storage + VFS
   - process manager
   - services registry + health monitor
   - event bus
3. The shell issues commands that call into services and kernel-facing APIs.

## Crate responsibilities

### `micronos-core`
- Owns foundational primitives:
  - identifiers (`ProcessId`, `ThreadId`, `NodeId`)
  - `SystemState` lifecycle state machine
  - common `Error` type and `Result`
  - core traits (Scheduler/Memory/Network/VFS contracts)

### `micronos-kernel`
- Owns kernel-facing APIs and lifecycle:
  - `MicronKernel` + `KernelState`
  - `BootSequence`
  - `Scheduler`
  - syscall dispatcher + handlers
  - POSIX compatibility layer
  - event bus

### `micronos-services`
- Long-lived services with explicit lifecycle:
  - memory manager
  - IPC manager
  - logger
  - config
  - stats
  - timers
  - signals
  - health monitoring

### `micronos-net`
- Networking primitives:
  - TCP and UDP stacks
  - socket types and stats
  - protocol primitives
  - antenna abstraction
  - P2P subsystem

### `micronos-fs`
- Storage and virtual filesystem:
  - mount state machine
  - in-memory file handles
  - capacity accounting

### `micronos-drivers`
- Driver interface and manager:
  - driver lifecycle (`init/start/stop/suspend/resume/shutdown`)
  - device IDs and types
  - registration + bulk ops

## Roadmap: architectural direction

MicronOS is currently a hosted, testable OS model. The roadmap is to evolve towards stronger invariants (type-state), a capability-based security model, and a more realistic separation between kernel services and user services.

See `ROADMAP.md` for planned milestones.
