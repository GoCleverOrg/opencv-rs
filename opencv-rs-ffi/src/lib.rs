#![cfg_attr(not(feature = "opencv"), allow(unused))]
//! OpenCV-backed production adapters for the `opencv-rs-core` ports.
//!
//! This is the **only** crate in the workspace that depends on the
//! `opencv` crate. Every other crate sees OpenCV exclusively through
//! the `opencv-rs-core` trait surface.
//!
//! Gated by the `opencv` feature so the workspace can build without the
//! OpenCV native libraries present on the host (matching the vmb-rs
//! `sdk`-feature pattern for `vmb-ffi`). When the feature is off this
//! crate compiles to an empty rlib — none of the `OpenCv*` adapter
//! types exist.

#[cfg(feature = "opencv")]
mod conversion;
#[cfg(feature = "opencv")]
pub mod image_encoder;
#[cfg(feature = "opencv")]
pub mod image_ops;
#[cfg(feature = "opencv")]
pub mod mat_view;
#[cfg(feature = "opencv")]
pub mod video_capture;

#[cfg(feature = "opencv")]
pub use image_encoder::OpenCvImageEncoder;
#[cfg(feature = "opencv")]
pub use image_ops::OpenCvImageOps;
#[cfg(feature = "opencv")]
pub use mat_view::{slice_to_mat, OpenCvMatView};
#[cfg(feature = "opencv")]
pub use video_capture::{OpenCvVideoCapture, OpenCvVideoStream};
