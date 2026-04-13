# opencv-rs-ffi

OpenCV-backed production adapters for the ports defined in
`opencv-rs-core`.

This is the **only** crate in the workspace that depends on the
`opencv` crate. Every other crate sees OpenCV exclusively through the
`opencv-rs-core` trait surface.

## `opencv` feature gate

All functionality is gated behind the `opencv` Cargo feature. Without
it, the crate compiles as an empty `rlib` — mirroring the `vmb-rs`
`sdk` pattern — so the workspace can build without the OpenCV native
libraries installed on the host.

## Host requirements

With the `opencv` feature on, OpenCV 4.x must be installed and
discoverable by `pkg-config` / the `opencv` crate's build script.

- macOS: `brew install opencv`
- Debian/Ubuntu: `apt install libopencv-dev clang libclang-dev`

## Adapters

- `OpenCvVideoCapture` → `VideoCapturePort`
- `OpenCvVideoStream` → `VideoStream`
- `OpenCvImageEncoder` → `ImageEncoderPort`
- `OpenCvImageOps` → `ImageOpsPort`

A zero-copy `slice_to_mat` helper wraps a `&[u8]` as an `opencv::Mat`
view for use inside the adapters.

## Equivalence tests

For every pure-Rust reimplementation in `opencv-rs-core` (currently in
`PureRustImageOps`), `tests/equivalence/<primitive>.rs` contains a
default-run integration test that feeds identical input to both
implementations and asserts byte-equivalence — or a documented
equivalence class for lossy ops — of the output. This prevents silent
semantic drift between the pure-Rust path and the OpenCV baseline.

## Running the tests

```
cargo test -p opencv-rs-ffi --features opencv
```

## License

`MIT OR Apache-2.0`.
