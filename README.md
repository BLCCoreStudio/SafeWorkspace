# SafeWorkspace

> **Project status: sunset / discontinued.**

SafeWorkspace was a focused Linux sandbox experiment for agent-assisted development using `bubblewrap` (`bwrap`). Its isolation direction was later integrated into AgentGuard, which has itself now been discontinued as BLCCoreStudio reduces overlapping experimental projects.

The repository remains public for historical reference and to preserve existing links and commit history, but **no further feature development or routine maintenance is planned**.

## Historical scope

SafeWorkspace explored restricted Linux workspaces that:

- mounted only the selected project writable at `/workspace`;
- exposed selected system paths read-only;
- cleared the inherited environment and used a minimal `PATH`;
- used a private temporary directory;
- unshared Linux namespaces, including network access, by default;
- refused to run when the required sandbox backend was unavailable.

The project was intentionally conservative about its security boundary: it was a bubblewrap-based process sandbox, not a virtual machine, and the kernel, bubblewrap, mounted system files, and invoked tools remained part of the trusted computing base.

## Historical source

Previous implementation details, tests, documentation, and development history remain available through the Git history.

## License

MIT © BLC Core Studio
