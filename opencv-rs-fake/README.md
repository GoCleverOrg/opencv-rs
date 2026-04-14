# opencv-rs-fake

Scripted in-memory implementations of every `opencv-rs-core` port, for
use as a dev-dependency in code that depends on the ports.

These fakes let tests script deterministic frame sequences, inject
errors at specific call sites, and record invocations for later
assertion — **without linking against the `opencv` crate**. This crate
depends only on `opencv-rs-core`; it must never depend on
`opencv-rs-ffi` or on `opencv`.

## Types

- `ScriptedVideoCapture` / `ScriptedVideoStream`
- `ScriptedImageEncoder`
- `PassthroughImageOps`

## Example

```rust
use opencv_rs_core::{Backend, CapturedFrame, PixelFormat, VideoCapturePort};
use opencv_rs_fake::ScriptedVideoCapture;
use std::path::Path;
use std::sync::Arc;

let frames = vec![
    CapturedFrame {
        width: 1,
        height: 1,
        pixel_format: PixelFormat::Bgr8,
        data: Arc::from([0u8, 0, 255].as_slice()),
    },
    CapturedFrame {
        width: 1,
        height: 1,
        pixel_format: PixelFormat::Bgr8,
        data: Arc::from([255u8, 0, 0].as_slice()),
    },
];

let capture = ScriptedVideoCapture::with_frames(frames);
let mut stream = capture.open(Path::new("ignored"), Backend::Auto).unwrap();

assert!(stream.read_frame().is_ok());
assert!(stream.read_frame().is_ok());
assert!(stream.read_frame().is_err()); // EndOfStream
```

## License

`MIT OR Apache-2.0`.
