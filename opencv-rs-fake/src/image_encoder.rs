//! Scripted in-memory implementation of [`ImageEncoderPort`] for unit
//! testing.

use std::sync::{Arc, Mutex};

use opencv_rs_core::{EncodingKind, ImageEncoderPort, ImageEncodingError, MatView, PixelFormat};

/// Internal mutable state for [`ScriptedImageEncoder`].
#[derive(Debug, Default)]
struct EncoderState {
    /// If `Some`, every `encode` call returns a clone of this buffer.
    canned: Option<Vec<u8>>,
    /// One entry per `encode` invocation: `(kind, input byte length)`.
    calls: Vec<(EncodingKind, usize)>,
}

/// Scripted [`ImageEncoderPort`] that either returns a user-supplied
/// canned byte buffer or a deterministic pseudo-encoding built from
/// the frame metadata and bytes.
///
/// Clones share the same state via `Arc<Mutex<_>>`.
#[derive(Debug, Clone, Default)]
pub struct ScriptedImageEncoder {
    inner: Arc<Mutex<EncoderState>>,
}

impl ScriptedImageEncoder {
    /// Construct an encoder with no canned buffer and no recorded
    /// calls.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure every future call to [`ImageEncoderPort::encode`] to
    /// return a clone of `bytes`, regardless of encoding kind.
    pub fn set_canned(&self, bytes: Vec<u8>) {
        self.inner.lock().expect("poisoned").canned = Some(bytes);
    }

    /// Snapshot of every recorded `encode` call: `(kind, input length)`.
    pub fn calls(&self) -> Vec<(EncodingKind, usize)> {
        self.inner.lock().expect("poisoned").calls.clone()
    }
}

/// Stable one-byte tag for a [`PixelFormat`], used to make the
/// pseudo-encoding format-aware.
fn pixel_format_tag(pf: PixelFormat) -> u8 {
    match pf {
        PixelFormat::Mono8 => 1,
        PixelFormat::Bgr8 => 2,
        PixelFormat::Rgb8 => 3,
    }
}

/// Build a deterministic pseudo-encoding for a frame under a kind
/// other than [`EncodingKind::None`].
fn pseudo_encode(frame: &dyn MatView, kind: EncodingKind) -> Vec<u8> {
    let magic: &[u8] = match kind {
        EncodingKind::Jpeg => b"FAKE_JPEG\0",
        EncodingKind::Webp => b"FAKE_WEBP\0",
        EncodingKind::None => b"FAKE_NONE\0",
    };
    let mut out = Vec::with_capacity(magic.len() + 16 + frame.data().len());
    out.extend_from_slice(magic);
    out.extend_from_slice(&frame.width().to_le_bytes());
    out.extend_from_slice(&frame.height().to_le_bytes());
    out.extend_from_slice(&frame.channels().to_le_bytes());
    out.push(pixel_format_tag(frame.pixel_format()));
    out.extend_from_slice(frame.data());
    out
}

impl ImageEncoderPort for ScriptedImageEncoder {
    fn encode(
        &self,
        frame: &dyn MatView,
        kind: EncodingKind,
    ) -> Result<Vec<u8>, ImageEncodingError> {
        let mut state = self.inner.lock().expect("poisoned");
        state.calls.push((kind, frame.data().len()));
        if let Some(canned) = &state.canned {
            return Ok(canned.clone());
        }
        drop(state);
        match kind {
            EncodingKind::None => Ok(frame.data().to_vec()),
            EncodingKind::Jpeg | EncodingKind::Webp => Ok(pseudo_encode(frame, kind)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv_rs_core::{OwnedMatView, PixelFormat};

    fn src() -> OwnedMatView {
        OwnedMatView::new(2, 1, PixelFormat::Mono8, vec![7u8, 9u8]).unwrap()
    }

    #[test]
    fn none_encoding_is_passthrough() {
        let enc = ScriptedImageEncoder::new();
        let frame = src();
        let out = enc.encode(&frame, EncodingKind::None).unwrap();
        assert_eq!(out.as_slice(), frame.data());
    }

    #[test]
    fn jpeg_and_webp_are_nonempty_and_deterministic() {
        let enc = ScriptedImageEncoder::new();
        let frame = src();
        let a = enc.encode(&frame, EncodingKind::Jpeg).unwrap();
        let b = enc.encode(&frame, EncodingKind::Jpeg).unwrap();
        assert_eq!(a, b);
        assert!(!a.is_empty());
        let w = enc.encode(&frame, EncodingKind::Webp).unwrap();
        assert!(!w.is_empty());
        assert_ne!(a, w);
    }

    #[test]
    fn canned_override_wins() {
        let enc = ScriptedImageEncoder::new();
        enc.set_canned(vec![0xAA, 0xBB, 0xCC]);
        let frame = src();
        let a = enc.encode(&frame, EncodingKind::None).unwrap();
        let b = enc.encode(&frame, EncodingKind::Jpeg).unwrap();
        assert_eq!(a, vec![0xAA, 0xBB, 0xCC]);
        assert_eq!(b, vec![0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn pseudo_encoding_tags_pixel_format_distinctly() {
        // Each PixelFormat gets a distinct non-zero tag byte. Kills the
        // `pixel_format_tag -> 0` and `-> 1` mutants.
        // Layout: magic(10) + width(4) + height(4) + channels(4) + tag(1) + data.
        const TAG_OFFSET: usize = 10 + 4 + 4 + 4;
        let enc = ScriptedImageEncoder::new();
        let mono = OwnedMatView::new(1, 1, PixelFormat::Mono8, vec![0]).unwrap();
        let bgr = OwnedMatView::new(1, 1, PixelFormat::Bgr8, vec![0, 0, 0]).unwrap();
        let rgb = OwnedMatView::new(1, 1, PixelFormat::Rgb8, vec![0, 0, 0]).unwrap();
        let m = enc.encode(&mono, EncodingKind::Jpeg).unwrap();
        let b = enc.encode(&bgr, EncodingKind::Jpeg).unwrap();
        let r = enc.encode(&rgb, EncodingKind::Jpeg).unwrap();
        assert_eq!(m[TAG_OFFSET], 1);
        assert_eq!(b[TAG_OFFSET], 2);
        assert_eq!(r[TAG_OFFSET], 3);
    }

    #[test]
    fn calls_are_recorded() {
        let enc = ScriptedImageEncoder::new();
        let frame = src();
        let _ = enc.encode(&frame, EncodingKind::None);
        let _ = enc.encode(&frame, EncodingKind::Webp);
        let calls = enc.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], (EncodingKind::None, 2));
        assert_eq!(calls[1], (EncodingKind::Webp, 2));
    }
}
