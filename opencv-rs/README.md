# opencv-rs

User-facing facade of the hexagonal `opencv-rs` workspace.

This crate re-exports the port layer (traits + DTOs) from
`opencv-rs-core` at its root, and — under the optional `opencv`
Cargo feature — the production adapters from `opencv-rs-ffi`.

## Feature flags

| Feature | Effect |
| --- | --- |
| `opencv` (off by default) | Pulls in `opencv-rs-ffi/opencv` and exposes `RealVideoCapture`, `RealImageEncoder`, `RealImageOps` aliases plus the `ffi` submodule. Requires OpenCV 4.x on the host. |

Without the `opencv` feature, only the port/DTO layer is usable. This
lets downstream crates compile and test against the fakes without the
OpenCV native libraries being installed — the architectural win.

## Crate layout

| Crate | Description | Depends on |
| --- | --- | --- |
| [`opencv-rs-core`](../opencv-rs-core) | Traits + DTOs, zero FFI | `thiserror` |
| [`opencv-rs-fake`](../opencv-rs-fake) | Scripted in-memory impls | `opencv-rs-core` |
| [`opencv-rs-ffi`](../opencv-rs-ffi) | Production impls via the `opencv` crate | `opencv-rs-core`, `opencv` |
| `opencv-rs` (this) | Facade re-exporting the above | `opencv-rs-core`, optionally `opencv-rs-ffi` |

## Quick start

```toml
[dependencies]
opencv-rs = { version = "0.1", features = ["opencv"] }
```

```rust,no_run
use opencv_rs::{Backend, RealVideoCapture, VideoCaptureError, VideoCapturePort};
use std::path::Path;

fn main() -> opencv_rs::Result<()> {
    let capture = RealVideoCapture::new();
    let mut stream = capture.open(Path::new("clip.mp4"), Backend::Auto)?;
    loop {
        match stream.read_frame() {
            Ok(_frame) => { /* process frame */ }
            Err(VideoCaptureError::EndOfStream) => break,
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
```

Tests in downstream crates typically depend on `opencv-rs` without any
features and on `opencv-rs-fake` as a dev-dependency.

## License

`MIT OR Apache-2.0`.
