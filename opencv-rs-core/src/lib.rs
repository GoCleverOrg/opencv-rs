//! Runtime-agnostic domain types and ports for `opencv-rs`.
//!
//! This crate defines the trait-level contract that production and test
//! implementations must satisfy: `VideoCapturePort`, `ImageEncoderPort`,
//! `ImageOpsPort`, `MatView`, plus the DTOs (`CapturedFrame`,
//! `EncodedImage`, `PixelFormat`, errors).
//!
//! It has **no** dependency on the `opencv` crate or any FFI code and
//! contains no `unsafe` blocks. This is the layer that downstream
//! consumers and test harnesses depend on.
//!
//! All real OpenCV integration lives in the sibling `opencv-rs-ffi`
//! crate. Tests use `opencv-rs-fake`. End users typically want the
//! top-level `opencv-rs` facade.
//!
//! The trait surface is filled in by issue #1
//! (see the repo's first issue). This initial scaffold declares the
//! module layout only.

// Populated by issue #1.
