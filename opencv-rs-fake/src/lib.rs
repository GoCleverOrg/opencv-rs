#![forbid(unsafe_code)]
//! Deterministic in-memory implementations of every `opencv-rs-core`
//! port for use in unit tests.
//!
//! The types in this crate let tests script programmable frame
//! sequences, inject errors, and record invocations for later
//! assertion, without linking against the `opencv` crate.
//!
//! This crate depends only on `opencv-rs-core`. It must never depend
//! on `opencv-rs-ffi` or the `opencv` crate.

pub mod image_encoder;
pub mod image_ops;
pub mod video_capture;

pub use image_encoder::ScriptedImageEncoder;
pub use image_ops::PassthroughImageOps;
pub use video_capture::{ScriptedVideoCapture, ScriptedVideoStream};
