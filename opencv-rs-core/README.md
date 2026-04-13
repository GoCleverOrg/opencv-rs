# opencv-rs-core

Runtime-agnostic domain types and ports for the `opencv-rs` workspace.
This crate defines the `VideoCapturePort`, `ImageEncoderPort`,
`ImageOpsPort`, and `MatView` traits plus their associated DTOs
(`CapturedFrame`, `EncodedImage`, `PixelFormat`, errors).

It has **no** dependency on the `opencv` crate or any FFI code and
contains no `unsafe` blocks.

All real OpenCV integration lives in the sibling `opencv-rs-ffi` crate,
which provides production implementations. Tests use `opencv-rs-fake`, an
in-memory implementation of each port.

End users typically want the `opencv-rs` facade crate, not this one
directly.
