# Security Policy

SafeWorkspace is under active development and should not yet be treated as a production-grade or complete security sandbox.

The current Linux preview uses **bubblewrap (`bwrap`)** to create an OS-level process boundary. It makes the selected workspace writable at `/workspace`, exposes selected system paths read-only, clears the inherited environment, provides a private temporary directory, and unshares Linux namespaces including network access by default.

This boundary has important limitations: it is not a virtual machine, and the Linux kernel, bubblewrap, mounted host files, and invoked tools remain part of the trusted computing base. Future policy changes may also alter the exact boundary. Use `safeworkspace plan` to inspect the generated bubblewrap command before relying on `run`.

Please report suspected vulnerabilities privately through GitHub private vulnerability reporting when available or another appropriate private channel. Include reproduction details without sharing real credentials or sensitive data.
