# SafeWorkspace

**Focused Linux sandbox research for agent-assisted development.**

> **Companion research status:** the current bubblewrap isolation direction from SafeWorkspace has been integrated into [AgentGuard](https://github.com/BLCCoreStudio/AgentGuard). This repository remains public as a focused implementation reference and development history; new integrated runtime-policy and isolation work targets AgentGuard.

SafeWorkspace explores Linux-first restricted workspaces for commands using **bubblewrap (`bwrap`)**. It does not pretend that application prompts alone are a security boundary, and it refuses to run the sandbox command when the required backend is unavailable.

## Current preview

Check whether the backend is available:

```bash
safeworkspace status
```

Inspect the exact bubblewrap plan without executing it:

```bash
safeworkspace plan ./project -- sh -c 'pwd && ls'
```

Run inside the restricted workspace:

```bash
safeworkspace run ./project -- sh -c 'pwd && ls'
```

The current backend:

- mounts only the selected project writable at `/workspace`
- exposes core system paths read-only when present
- clears the inherited environment and sets a minimal `PATH`
- uses a private temporary `/tmp`
- unshares Linux namespaces, including network access, by default
- performs no privilege escalation

## Why this repository still exists

SafeWorkspace is intentionally retained rather than deleted or republished. It preserves the smaller isolation experiment that informed AgentGuard's integrated sandbox mode, keeps existing links and commit history valid, and provides a narrow place to understand the bubblewrap boundary without the rest of AgentGuard's policy logic.

For active integration work, use **AgentGuard**.

## Requirements and limitations

- Linux only in the current preview
- `bwrap` / bubblewrap must already be installed
- this is a bubblewrap-based process sandbox, **not a virtual machine**
- kernel, bubblewrap, mounted system files, and invoked tools remain part of the trusted computing base
- the intentionally minimal environment can make some developer toolchains unavailable until explicit policy support is added

Use `plan` before `run` when evaluating the generated boundary.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build --locked
cargo test --locked
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
