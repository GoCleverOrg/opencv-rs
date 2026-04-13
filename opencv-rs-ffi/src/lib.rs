//! OpenCV-backed production adapters for the `opencv-rs-core` ports.
//!
//! This is the **only** crate in the workspace that depends on the
//! `opencv` crate. Every other crate sees OpenCV exclusively through
//! the `opencv-rs-core` trait surface.
//!
//! Gated by the `opencv` feature so the workspace can build without the
//! OpenCV native libraries present on the host (matching the vmb-rs
//! `sdk`-feature pattern for `vmb-ffi`).
//!
//! Production types (`OpenCvVideoCapture`, `OpenCvImageEncoder`,
//! `OpenCvImageOps`, `OpenCvMatView`) are filled in by issue #1.

// Populated by issue #1.
