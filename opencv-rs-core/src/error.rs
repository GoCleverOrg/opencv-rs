//! Error types for the `opencv-rs` domain layer.
//!
//! Each port has its own typed error enum; [`OpenCvError`] is the umbrella
//! type returned by high-level facades that may fail across any of them.

use thiserror::Error;

/// Convenience [`Result`] alias using [`OpenCvError`] as the error type.
pub type Result<T> = std::result::Result<T, OpenCvError>;

/// Errors produced by [`crate::VideoCapturePort`] and [`crate::VideoStream`].
#[derive(Debug, Error)]
pub enum VideoCaptureError {
    /// The backend could not open the provided source path.
    #[error("failed to open video source: {path}")]
    OpenFailed {
        /// The path (or URL) that the backend refused to open.
        path: String,
    },
    /// An operation was invoked on a stream that is not currently open.
    #[error("video stream is not open")]
    NotOpen,
    /// A frame read failed for reasons other than end-of-stream.
    #[error("failed to read frame")]
    ReadFailed,
    /// The stream has been exhausted.
    #[error("end of stream")]
    EndOfStream,
    /// The requested [`crate::Backend`] is not available on this platform.
    #[error("unsupported backend")]
    UnsupportedBackend,
    /// Seeking to the requested position failed.
    #[error("seek failed")]
    SeekFailed,
    /// The backend reported a non-positive or otherwise invalid FPS.
    #[error("invalid fps property")]
    InvalidFps,
    /// A backend-specific error surfaced as a string.
    #[error("backend error: {0}")]
    Backend(String),
}

/// Errors produced by [`crate::ImageEncoderPort`].
#[derive(Debug, Error)]
pub enum ImageEncodingError {
    /// The encoder failed for the given encoding kind.
    #[error("encoding failed for kind {kind}")]
    EncodeFailed {
        /// Human-readable name of the encoding kind (e.g. `"jpeg"`).
        kind: &'static str,
    },
    /// The encoder does not support the input's pixel format.
    #[error("unsupported pixel format: {0:?}")]
    UnsupportedPixelFormat(crate::PixelFormat),
    /// A backend-specific error surfaced as a string.
    #[error("backend error: {0}")]
    Backend(String),
}

/// Errors produced by [`crate::ImageOpsPort`].
#[derive(Debug, Error)]
pub enum ImageOpsError {
    /// Two inputs did not agree on `(width, height, channels)`.
    #[error("dimension mismatch: lhs {lhs:?}, rhs {rhs:?}")]
    DimensionMismatch {
        /// Dimensions of the left-hand input as `(width, height, channels)`.
        lhs: (u32, u32, u32),
        /// Dimensions of the right-hand input as `(width, height, channels)`.
        rhs: (u32, u32, u32),
    },
    /// The operation does not support the input's pixel format.
    #[error("unsupported pixel format: {0:?}")]
    UnsupportedPixelFormat(crate::PixelFormat),
    /// The operation does not support the requested source/destination conversion.
    #[error("unsupported conversion: {src:?} -> {dst:?}")]
    UnsupportedConversion {
        /// Source pixel format.
        src: crate::PixelFormat,
        /// Destination pixel format.
        dst: crate::PixelFormat,
    },
    /// A parameter was out of range or otherwise invalid.
    #[error("invalid parameter: {0}")]
    InvalidParameter(&'static str),
    /// The input buffer was empty where a non-empty one was required.
    #[error("empty input")]
    EmptyInput,
    /// A backend-specific error surfaced as a string.
    #[error("backend error: {0}")]
    Backend(String),
}

/// Umbrella error type spanning every port in the domain.
#[derive(Debug, Error)]
pub enum OpenCvError {
    /// A [`VideoCaptureError`] occurred.
    #[error(transparent)]
    VideoCapture(#[from] VideoCaptureError),
    /// An [`ImageEncodingError`] occurred.
    #[error(transparent)]
    ImageEncoding(#[from] ImageEncodingError),
    /// An [`ImageOpsError`] occurred.
    #[error(transparent)]
    ImageOps(#[from] ImageOpsError),
}
