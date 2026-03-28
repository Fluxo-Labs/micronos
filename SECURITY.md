# Security Policy

MicronOS is an experimental OS project. Security is a core goal, but the project is still evolving.

## Supported versions

Only the latest `main` branch is supported.

## Reporting a vulnerability

Please do **not** open a public issue for security vulnerabilities.

- Email: **TBD** (maintainers will add an address once the project has a security inbox)
- Until then: open a GitHub issue with the title prefix **"[SECURITY]"** and request private handling.

## Dependency security

The repository includes `deny.toml` and CI runs a dependency audit.

## Security goals

- Memory safety by default (Rust)
- Explicit state machines for critical lifecycles
- Strong testing culture (`cargo test --workspace`)
- Future direction: capability-based access control
