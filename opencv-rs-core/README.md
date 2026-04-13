# opencv-rs-core

Runtime-agnostic ports and DTOs for the [`opencv-rs`](../) workspace.

This crate defines the trait-level contract that every production and
test implementation must satisfy. It is the layer downstream consumers
and test harnesses depend on.

## Zero FFI

`opencv-rs-core` has **no dependency on the `opencv` crate** and
contains no `unsafe` blocks (`#![forbid(unsafe_code)]`). Consumers
that depend only on this crate compile and test without OpenCV
installed on the host.

## Ports

| Port | Role |
| --- | --- |
| `MatView` | Read-only view over a pixel buffer (`width`, `height`, `channels`, `pixel_format`, `data`). `OwnedMatView` is the pure-Rust owned variant. |
| `VideoCapturePort` | Opens a video source and returns a `VideoStream`. |
| `VideoStream` | Pull API yielding `CapturedFrame`s. |
| `ImageEncoderPort` | Encodes a `MatView` into a chosen `EncodingKind` (JPEG, WebP, raw). |
| `ImageOpsPort` | OpenCV-equivalent image primitives: `cvt_color`, `gaussian_blur`, `threshold`, `absdiff`, `convert_scale_abs`, `min_max_loc`, `count_non_zero`, `resize`. |

DTOs: `CapturedFrame`, `PixelFormat`, `Backend`, `EncodingKind`,
`ColorConversion`, `ThresholdKind`, `MinMaxResult`, plus the error
types `OpenCvError`, `VideoCaptureError`, `ImageEncodingError`,
`ImageOpsError`.

`PureRustImageOps` is a pure-Rust reimplementation of the element-wise
and reduction methods of `ImageOpsPort`. Every method it implements is
covered by a byte-equivalence test against `OpenCvImageOps` in
`opencv-rs-ffi/tests/equivalence/`.

## `contract_tests` module

The public `contract_tests` module exposes reusable test helpers — one
per trait method — that assert the behavioural contract of any
implementation. Both `opencv-rs-fake` and `opencv-rs-ffi` exercise
these helpers against their adapters, and downstream crates with their
own implementations can re-verify the contract via a plain
`cargo test`.

End users typically depend on the [`opencv-rs`](../opencv-rs) facade
rather than this crate directly.

## License

`MIT OR Apache-2.0`.
