//! OpenCV-backed implementation of [`ImageEncoderPort`].

use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::prelude::VectorToVec;
use opencv_rs_core::{EncodingKind, ImageEncoderPort, ImageEncodingError, MatView};

use crate::conversion::mat_from_view;

/// Production [`ImageEncoderPort`] delegating to
/// `opencv::imgcodecs::imencode`.
///
/// [`EncodingKind::None`] short-circuits and returns the raw pixel
/// bytes verbatim (no OpenCV call is made); the JPEG and WebP encoders
/// both use a quality setting of 100.
#[derive(Debug, Default, Clone, Copy)]
pub struct OpenCvImageEncoder;

impl OpenCvImageEncoder {
    /// Construct a new [`OpenCvImageEncoder`].
    pub fn new() -> Self {
        Self
    }
}

impl ImageEncoderPort for OpenCvImageEncoder {
    fn encode(
        &self,
        frame: &dyn MatView,
        kind: EncodingKind,
    ) -> Result<Vec<u8>, ImageEncodingError> {
        if matches!(kind, EncodingKind::None) {
            return Ok(frame.data().to_vec());
        }
        let mat = mat_from_view(frame).map_err(|e| ImageEncodingError::Backend(e.to_string()))?;
        let (ext, quality_param) = match kind {
            EncodingKind::Jpeg => (".jpg", imgcodecs::IMWRITE_JPEG_QUALITY),
            EncodingKind::Webp => (".webp", imgcodecs::IMWRITE_WEBP_QUALITY),
            EncodingKind::None => unreachable!("None is short-circuited above"),
        };
        let mut params = Vector::<i32>::new();
        params.push(quality_param);
        params.push(100);
        let mut buf = Vector::<u8>::new();
        let ok = imgcodecs::imencode(ext, &mat, &mut buf, &params)
            .map_err(|e| ImageEncodingError::Backend(e.to_string()))?;
        if !ok {
            return Err(ImageEncodingError::EncodeFailed { kind: kind.name() });
        }
        Ok(buf.to_vec())
    }
}
