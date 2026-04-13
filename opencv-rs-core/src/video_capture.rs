//! Video capture port and associated DTOs.

use std::path::Path;
use std::sync::Arc;

use crate::{MatView, PixelFormat, VideoCaptureError};

/// Selector for the underlying capture backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Backend {
    /// Let the backend pick whichever implementation is available.
    Auto,
    /// Force an FFmpeg-backed capture pipeline.
    Ffmpeg,
}

/// A single frame returned by a [`VideoStream`].
#[derive(Debug, Clone)]
pub struct CapturedFrame {
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
    /// Pixel layout of [`Self::data`].
    pub pixel_format: PixelFormat,
    /// Shared-ownership pixel buffer.
    pub data: Arc<[u8]>,
}

impl MatView for CapturedFrame {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn channels(&self) -> u32 {
        self.pixel_format.channels()
    }
    fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Opens video sources and produces [`VideoStream`] instances.
pub trait VideoCapturePort: Send + Sync {
    /// Open the given path/URL with the chosen backend.
    fn open(
        &self,
        path: &Path,
        backend: Backend,
    ) -> Result<Box<dyn VideoStream>, VideoCaptureError>;
}

/// An open video stream producing frames on demand.
pub trait VideoStream: Send {
    /// Reads the next frame. Returns
    /// [`VideoCaptureError::EndOfStream`] when the stream is exhausted.
    fn read_frame(&mut self) -> Result<CapturedFrame, VideoCaptureError>;
    /// Frames-per-second as reported by the backend.
    fn fps(&self) -> Result<f64, VideoCaptureError>;
    /// Rewind the stream so the next [`Self::read_frame`] returns the first frame.
    fn seek_to_start(&mut self) -> Result<(), VideoCaptureError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captured_frame_exposes_mat_view() {
        let frame = CapturedFrame {
            width: 2,
            height: 1,
            pixel_format: PixelFormat::Mono8,
            data: Arc::from(vec![1u8, 2u8].into_boxed_slice()),
        };
        assert_eq!(frame.width(), 2);
        assert_eq!(frame.height(), 1);
        assert_eq!(frame.channels(), 1);
        assert_eq!(frame.pixel_format(), PixelFormat::Mono8);
        assert_eq!(frame.data(), &[1u8, 2u8]);
    }

    #[test]
    fn backend_equality_is_value_wise() {
        assert_eq!(Backend::Auto, Backend::Auto);
        assert_ne!(Backend::Auto, Backend::Ffmpeg);
    }
}
