# pd-controller

`pd-controller` is the control-plane service, state model, RPC surface, remote-debug orchestration, and Web UI split from the original `project-d` history.

## Repository split

- RustScript core VM and standard library: https://github.com/rustscript-lang/rustscript
- CLR VM: https://github.com/rustscript-lang/rustscript-clr-vm
- Edge runtime and ABI: https://github.com/rustscript-lang/pd-edge
- Controller: https://github.com/rustscript-lang/pd-controller

## Local crates

The split keeps local VM and edge crates so controller tests and UI code generation tests can run before remote split crates are published.

For downstream Cargo manifests, the intended repository references are:

```toml
pd-vm = { git = "https://github.com/rustscript-lang/rustscript", package = "pd-vm" }
pd-edge = { git = "https://github.com/rustscript-lang/pd-edge", package = "pd-edge" }
pd-controller = { git = "https://github.com/rustscript-lang/pd-controller", package = "pd-controller" }
```

## Test

```bash
cargo test --workspace --jobs 4
cargo build --workspace --release --jobs 4
```
