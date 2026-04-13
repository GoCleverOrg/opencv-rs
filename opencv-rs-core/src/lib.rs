#![forbid(unsafe_code)]
//! Runtime-agnostic domain types and ports for `opencv-rs`.
//!
//! This crate defines the trait-level contract that production and test
//! implementations must satisfy: [`VideoCapturePort`], [`ImageEncoderPort`],
//! [`ImageOpsPort`], [`MatView`], plus the DTOs ([`CapturedFrame`],
//! [`PixelFormat`], errors).
//!
//! It has **no** dependency on the `opencv` crate or any FFI code and
//! contains no `unsafe` blocks. This is the layer that downstream
//! consumers and test harnesses depend on.
//!
//! All real OpenCV integration lives in the sibling `opencv-rs-ffi`
//! crate. Tests use `opencv-rs-fake`. End users typically want the
//! top-level `opencv-rs` facade.

pub mod contract_tests;
pub mod error;
pub mod image_encoder;
pub mod image_ops;
pub mod mat_view;
pub mod types;
pub mod video_capture;

pub use error::{ImageEncodingError, ImageOpsError, OpenCvError, Result, VideoCaptureError};
pub use image_encoder::{EncodingKind, ImageEncoderPort};
pub use image_ops::{ColorConversion, ImageOpsPort, MinMaxResult, PureRustImageOps, ThresholdKind};
pub use mat_view::{MatView, OwnedMatView};
pub use types::PixelFormat;
pub use video_capture::{Backend, CapturedFrame, VideoCapturePort, VideoStream};
