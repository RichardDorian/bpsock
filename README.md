# `bpsock`

A collection of Rust crates providing a POSIX-compatible API for the Bundle Protocol Version 7 ([RFC 9171](https://www.rfc-editor.org/info/rfc9171/)).

## Workspace overview

| Crate | Description |
| ---- | ---- |
| `libbp` | Core structs. |
| `libbps` | Client and server implementations (`bp_socket`, `bp_bind`, `bp_recv`, `bp_send`, ...) and forwarding to Bundle Protocol Agents (BPAs). |
| `libbprun` | `seccomp` filters and handlers. |
| `bprun` | Process launcher with built-in supervisor support. |
| `bprund` | `seccomp` supervisor daemon. |
| `agents/*` | Adapters to BPAs (ION, µD3TN, dtn7, Hardy, ...) used by `libbps`. |
