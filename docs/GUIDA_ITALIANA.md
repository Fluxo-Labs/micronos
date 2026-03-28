# Documentazione MicronOS - Guida in Italiano

## Panoramica

MicronOS è un sistema operativo sperimentale scritto interamente in Rust 2024. È stato creato come proof-of-concept per dimostrare l'uso innovativo dei pattern type-state nella costruzione di sistemi operativi.

**Nota importante**: Questo progetto è un'idea originale di **alisio85**. Il codice è stato generato come dimostrazione delle capacità di AI coding, ma l'idea e la visione sono di alisio85.

## Struttura del Progetto

```
micronos/
├── crates/
│   ├── micronos-core/     # Tipi base, trait, macchine a stati
│   ├── micronos-kernel/   # Implementazione del kernel
│   ├── micronos-net/      # Sottosistema di rete
│   ├── micronos-fs/       # Sottosistema filesystem
│   └── micronos-services/ # Servizi di sistema
├── .github/workflows/      # Automazioni CI/CD
├── Cargo.toml             # Workspace configuration
└── LICENSE               # Licenza MIT
```

## Crate Principali

### 1. micronos-core

Contiene le definizioni fondamentali:

- **Tipi di sistema**: `ProcessId`, `ThreadId`, `MemoryAddress`, `NodeId`
- **Enumeration degli stati**: Pattern type-state per la gestione degli stati del sistema
- **Trait di sistema**: `Scheduler`, `MemoryManager`, `NetworkStack`, `FileSystem`

### 2. micronos-kernel

Implementazione del kernel basata su `os_kernel_foundry`:

- **MicronKernel**: Macchina a stati per il lifecycle del kernel
- **BootSequence**: Pipeline di boot modulare
- **MicronScheduler**: Scheduler cooperativo con type-state

### 3. micronos-net

Sottosistema di rete con `micronet-antenna`:

- **MicronetAntenna**: Gestione connessioni WiFi/network
- **P2PStack**: Stack peer-to-peer con `libp2p`

### 4. micronos-fs

Filesystem virtuale:

- **VirtualFileSystem**: VFS con mount points
- **StorageManager**: Gestione storage

### 5. micronos-services

Servizi di sistema:

- **ProcessManager**: Gestione processi con lifecycle completo
- **ServiceRegistry**: Registro servizi
- **HealthMonitor**: Monitoraggio salute sistema

## Pattern Type-State

MicronOS utilizza due crate per il type-state pattern:

### fluxo-typestate

Macro procedurali per macchine a stati type-safe:

```rust
use fluxo_typestate::state_machine;

#[state_machine]
pub enum SystemState {
    #[transition(Off -> Initializing: boot)]
    Off,
    Initializing,
    Running { uptime_ticks: u64 },
}
```

### state-shift

API più ergonomica per type-state:

```rust
use state_shift::{impl_state, type_state};

#[type_state(states = (Idle, Active, Paused), slots = (Scheduler))]
pub struct Scheduler { ... }
```

## Build e Test

### Build locale

```bash
cd micronos
cargo build --workspace
```

### Esecuzione test

```bash
cargo test --workspace --all-features
```

### Generazione documentazione

```bash
cargo doc --workspace --no-deps
```

## Automazioni GitHub

Il progetto include tre workflow CI:

1. **ci.yml** - Test, linting, coverage
2. **release.yml** - Pubblicazione su crates.io
3. **deny.toml** - Audit sicurezza dipendenze

## Dipendenze Principali

| Crate | Versione | Scopo |
|-------|---------|-------|
| fluxo-typestate | 0.1 | Type-state pattern |
| state-shift | 2.1 | Type-state pattern |
| os_kernel_foundry | 0.1 | Kernel foundation |
| os_foundry_suite | 0.1 | Suite OS building |
| micronet-antenna | 0.1 | Network discovery |
| libp2p | 0.55 | P2P networking |

## Roadmap

1. ✅ Implementazione core types e trait
2. ✅ Kernel base con state machines
3. ✅ Networking subsystem
4. ✅ Filesystem subsystem
5. ⏳ Servizi avanzati
6. ⏳ Boot reale su hardware
7. ⏳ Driver di sistema

## Contributi

Questo è un progetto sperimentale. Contributi e feedback sono benvenuti.

## Licenza

MIT License - Basata su un'idea originale di **alisio85**
