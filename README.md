# opencv-rs

Hexagonal Rust wrapper over OpenCV I/O and image-processing primitives.

Follows the same ports-and-adapters layout as
[`vmb-rs`](https://github.com/GoCleverOrg/vmb-rs): the domain and test
layers do not depend on the FFI, and a production adapter wraps the
real backend behind a Cargo feature.

## Requirements

OpenCV 4.10 or later (when using the `opencv` feature). The `cvt_color` and `gaussian_blur` adapters use `AlgorithmHint::ALGO_HINT_DEFAULT`, introduced in OpenCV 4.10.

## Architecture

```text
                 opencv-rs-core
              (traits + DTOs, zero FFI)
                        ▲
             ┌──────────┴──────────┐
             │                     │
       opencv-rs-fake        opencv-rs-ffi
   (scripted in-memory     (OpenCV-backed
    impls for tests)        production adapters)
                                   ▲
                                   │
                              opencv-rs
                        (user-facing facade)
```

Arrows point from dependent to dependency. `opencv-rs` depends on
`opencv-rs-core` unconditionally and on `opencv-rs-ffi` only when the
`opencv` feature is enabled.

**`opencv-rs-core` and `opencv-rs-fake` have zero dependency on the
`opencv` crate.** The workspace compiles and its unit tests run
without OpenCV installed when the `opencv` feature is off. Only
`opencv-rs-ffi` (and, transitively, `opencv-rs` with the `opencv`
feature) links against the native OpenCV libraries.

## Crates

| Crate | Responsibility |
| --- | --- |
| [`opencv-rs-core`](./opencv-rs-core) | Runtime-agnostic ports (`VideoCapturePort`, `ImageEncoderPort`, `ImageOpsPort`, `MatView`) and DTOs. |
| [`opencv-rs-fake`](./opencv-rs-fake) | Deterministic scripted implementations of every port for unit tests. |
| [`opencv-rs-ffi`](./opencv-rs-ffi) | Production adapters (`OpenCvVideoCapture`, `OpenCvImageEncoder`, `OpenCvImageOps`) wrapping the `opencv` crate. |
| [`opencv-rs`](./opencv-rs) | User-facing facade re-exporting the ports and, under the `opencv` feature, the production adapters. |

## Quick start

Library consumers depend on the facade with the `opencv` feature:

```toml
[dependencies]
opencv-rs = { version = "0.1", features = ["opencv"] }
```

```rust,no_run
use opencv_rs::{Backend, RealVideoCapture, VideoCapturePort};
use std::path::Path;

fn main() -> opencv_rs::Result<()> {
    let capture = RealVideoCapture::new();
    let mut stream = capture.open(Path::new("clip.mp4"), Backend::Auto)?;
    loop {
        match stream.read_frame() {
            Ok(_frame) => { /* process frame */ }
            Err(opencv_rs::VideoCaptureError::EndOfStream) => break,
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
```

## Testing with the fake

Downstream tests depend on `opencv-rs-fake` as a dev-dependency:

```toml
[dev-dependencies]
opencv-rs = "0.1"            # no features -> no OpenCV linkage
opencv-rs-fake = "0.1"
```

```rust
use opencv_rs::{Backend, CapturedFrame, PixelFormat, VideoCapturePort};
use opencv_rs_fake::ScriptedVideoCapture;
use std::path::Path;
use std::sync::Arc;

let frame = CapturedFrame {
    width: 2,
    height: 1,
    pixel_format: PixelFormat::Bgr8,
    data: Arc::from([0u8, 0, 255, 255, 0, 0].as_slice()),
};
let capture = ScriptedVideoCapture::with_frames(vec![frame]);
let mut stream = capture.open(Path::new("ignored"), Backend::Auto).unwrap();
assert!(stream.next_frame().unwrap().is_some());
```

## Equivalence-tests policy

Any pure-Rust reimplementation of an `ImageOpsPort` method living in
`opencv-rs-core` (for example the `PureRustImageOps` struct) MUST be
covered by a default-run (no `#[ignore]`) integration test in
`opencv-rs-ffi/tests/equivalence/<primitive>.rs` that asserts
byte-equivalence — or a documented equivalence class, for lossy ops —
with the output of the real OpenCV adapter on identical input.

This rule prevents silent semantic drift between the pure-Rust path
and the OpenCV path.

## Development

```
make test           # cargo test --workspace
make lint           # fmt + clippy + deny + shear + taplo + typos
make mutants        # full workspace mutation bench
```

## Contributing

See [CHANGELOG.md](./CHANGELOG.md) for the release history.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))
- MIT license ([LICENSE-MIT](./LICENSE-MIT))

at your option (`MIT OR Apache-2.0`).
