# opencv-rs

Hexagonal Rust wrapper over OpenCV I/O and image-processing primitives,
mirroring the architecture of [`vmb-rs`](https://github.com/GoCleverOrg/vmb-rs).

This crate is the user-facing facade. It re-exports the port layer from
`opencv-rs-core` and — under the optional `opencv` feature — the
production adapters from `opencv-rs-ffi`.

| Crate | Description | Depends on |
| --- | --- | --- |
| [`opencv-rs-core`](../opencv-rs-core) | Traits + DTOs; zero FFI | `thiserror` |
| [`opencv-rs-fake`](../opencv-rs-fake) | In-memory scripted impls | `opencv-rs-core` |
| [`opencv-rs-ffi`](../opencv-rs-ffi) | Production impls via `opencv` crate | `opencv-rs-core`, `opencv` |
| `opencv-rs` (this) | Facade re-exporting the above | `opencv-rs-core`, optionally `opencv-rs-ffi` |
