//! Image encoder port.

use crate::{ImageEncodingError, MatView};

/// The output container an [`ImageEncoderPort`] should produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingKind {
    /// JPEG-encoded bytes.
    Jpeg,
    /// WebP-encoded bytes.
    Webp,
    /// No encoding; the encoder should return the raw pixel bytes verbatim.
    None,
}

impl EncodingKind {
    /// Human-readable short name for the encoding.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
            Self::None => "none",
        }
    }
}

/// Encodes a [`MatView`] into a byte buffer in one of the [`EncodingKind`]s.
pub trait ImageEncoderPort: Send + Sync {
    /// Encode `frame` using `kind` and return the resulting bytes.
    fn encode(
        &self,
        frame: &dyn MatView,
        kind: EncodingKind,
    ) -> Result<Vec<u8>, ImageEncodingError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_kind_names() {
        assert_eq!(EncodingKind::Jpeg.name(), "jpeg");
        assert_eq!(EncodingKind::Webp.name(), "webp");
        assert_eq!(EncodingKind::None.name(), "none");
    }
}
