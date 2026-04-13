# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-13

### Added

- Four-crate hexagonal workspace: `opencv-rs-core`, `opencv-rs-fake`, `opencv-rs-ffi`, `opencv-rs`.
- `MatView` trait and `OwnedMatView` pure-Rust struct.
- `VideoCapturePort` + `VideoStream` traits; `Backend` enum (`Auto`, `Ffmpeg`); `CapturedFrame` DTO; `OpenCvVideoCapture` adapter.
- `ImageEncoderPort` trait; `EncodingKind` enum (`Jpeg`, `Webp`, `None`); `OpenCvImageEncoder` adapter.
- `ImageOpsPort` trait with methods: `cvt_color`, `gaussian_blur`, `threshold`, `absdiff`, `convert_scale_abs`, `min_max_loc`, `count_non_zero`, `resize`.
- `PureRustImageOps` pure-Rust reimplementations for element-wise and reduction methods (`cvt_color` for BGR<->RGB / Gray<->RGB, `threshold`, `absdiff`, `count_non_zero`, `convert_scale_abs`, `min_max_loc`).
- `OpenCvImageOps` adapter wrapping `opencv::imgproc` + `opencv::core`.
- `contract_tests` public module in `opencv-rs-core` with helpers for every trait method, consumed by both `opencv-rs-fake` and `opencv-rs-ffi` test suites.
- Equivalence tests in `opencv-rs-ffi/tests/equivalence/` for every pure-Rust reimplementation.
- `ScriptedVideoCapture`, `ScriptedVideoStream`, `ScriptedImageEncoder`, `PassthroughImageOps` scripted fakes.
- Zero-copy `slice_to_mat` helper in `opencv-rs-ffi`.
