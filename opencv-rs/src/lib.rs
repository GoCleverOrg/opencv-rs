//! Hexagonal Rust wrapper over OpenCV primitives.
//!
//! Convenience facade re-exporting the ports defined in
//! `opencv-rs-core`. When the `opencv` feature is enabled, the production
//! adapters from `opencv-rs-ffi` are also re-exported.
//!
//! ```text
//! opencv-rs-core   ← traits + DTOs, zero FFI
//!     ↑
//!     ├── opencv-rs-fake  ← deterministic in-memory impls for tests
//!     └── opencv-rs-ffi   ← production impls wrapping the `opencv` crate
//!             ↑
//!             opencv-rs  ← this facade
//! ```
//!
//! End users should depend on this crate. Downstream tests depend on
//! `opencv-rs-fake` as a dev-dependency.

pub use opencv_rs_core as core;

pub use opencv_rs_core::{
    Backend, CapturedFrame, ColorConversion, EncodingKind, ImageEncoderPort, ImageEncodingError,
    ImageOpsError, ImageOpsPort, MatView, MinMaxResult, OpenCvError, OwnedMatView, PixelFormat,
    PureRustImageOps, Result, ThresholdKind, VideoCaptureError, VideoCapturePort, VideoStream,
};

#[cfg(feature = "opencv")]
pub use opencv_rs_ffi as ffi;

/// Production [`VideoCapturePort`] implementation backed by the `opencv` crate.
#[cfg(feature = "opencv")]
pub type RealVideoCapture = opencv_rs_ffi::OpenCvVideoCapture;

/// Production [`ImageEncoderPort`] implementation backed by the `opencv` crate.
#[cfg(feature = "opencv")]
pub type RealImageEncoder = opencv_rs_ffi::OpenCvImageEncoder;

/// Production [`ImageOpsPort`] implementation backed by the `opencv` crate.
#[cfg(feature = "opencv")]
pub type RealImageOps = opencv_rs_ffi::OpenCvImageOps;

#[cfg(test)]
mod tests {
    use super::*;
    use opencv_rs_fake::ScriptedVideoCapture;

    #[test]
    fn facade_types_compose() {
        let capture = ScriptedVideoCapture::new();
        let _port: &dyn VideoCapturePort = &capture;
    }
}
