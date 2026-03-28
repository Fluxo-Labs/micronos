# MicronOS Roadmap

This roadmap describes the direction of MicronOS as a Rust micronation OS.

## Near-term (stabilize & scale)

- Keep CI green on every PR
- Strengthen crate boundaries and public APIs
- Increase integration tests (shell-driven, syscall-driven)
- Improve documentation coverage (EN)

## Mid-term (capabilities & invariants)

- Introduce capability-based access model
- Replace ad-hoc enums with type-state where it improves invariants
- Expand event tracing and metrics

## Long-term (platform expansion)

- Optional bare-metal target
- More realistic drivers and device models
- Package/service distribution model
