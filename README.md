# SafeWorkspace

**Linux-first restricted workspaces for safely running AI coding agents and automation.**

> **Status:** early development. No stable release has been published.

SafeWorkspace is intended to create explicit execution boundaries for agent-assisted development rather than relying only on application-level prompts.

## Planned v0.1

The first milestone will investigate Linux-native isolation with a narrow, auditable model:

- explicit workspace root
- read/write path policy
- optional network restriction
- process/environment minimization
- no silent privilege escalation
- dry-run/plan output before execution
- clear detection of unsupported isolation capabilities

The current repository is a development scaffold only. It does **not** provide a security sandbox yet and should not be represented as one.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build
cargo test
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
