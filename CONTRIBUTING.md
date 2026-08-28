# Contributing

Contributions around Linux isolation research, capability detection, policy design, tests, and documentation are welcome.

Before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Security-boundary claims require tests and documentation. Never weaken isolation silently. Follow `SECURITY.md` for vulnerabilities.
