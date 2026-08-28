# SafeWorkspace

**Linux-first restricted workspaces for running agent-assisted commands with explicit OS-level boundaries.**

> **Status:** development preview. No stable release has been published.

SafeWorkspace uses Linux **bubblewrap (`bwrap`)** as its current isolation backend. It does not pretend that application prompts alone are a security boundary, and it refuses to run the sandbox command when the required backend is unavailable.

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
