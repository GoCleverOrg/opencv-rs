//! [`MatView`] trait and a pure-Rust owned implementation.

use crate::{ImageOpsError, PixelFormat};

/// Read-only view over an image buffer with pixel-format metadata.
pub trait MatView {
    /// Width in pixels.
    fn width(&self) -> u32;
    /// Height in pixels.
    fn height(&self) -> u32;
    /// Number of channels per pixel.
    fn channels(&self) -> u32;
    /// The pixel format describing the interleaved byte layout.
    fn pixel_format(&self) -> PixelFormat;
    /// The raw pixel bytes in row-major interleaved order.
    fn data(&self) -> &[u8];
}

/// Pure-Rust owned mat-view for use in tests and pure-Rust pipelines.
#[derive(Debug, Clone)]
pub struct OwnedMatView {
    width: u32,
    height: u32,
    pixel_format: PixelFormat,
    data: Vec<u8>,
}

impl OwnedMatView {
    /// Build a new owned mat-view. Fails if `data.len()` does not match
    /// `width * height * pixel_format.channels()`.
    pub fn new(
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        data: Vec<u8>,
    ) -> Result<Self, ImageOpsError> {
        let expected = (width as usize) * (height as usize) * (pixel_format.channels() as usize);
        if data.len() != expected {
            return Err(ImageOpsError::InvalidParameter(
                "OwnedMatView: data length does not match width*height*channels",
            ));
        }
        Ok(Self {
            width,
            height,
            pixel_format,
            data,
        })
    }

    /// Allocate a zero-filled mat-view of the given shape and format.
    pub fn zeros(width: u32, height: u32, pixel_format: PixelFormat) -> Self {
        let len = (width as usize) * (height as usize) * (pixel_format.channels() as usize);
        Self {
            width,
            height,
            pixel_format,
            data: vec![0u8; len],
        }
    }

    /// Consume the view and return the backing buffer.
    pub fn into_data(self) -> Vec<u8> {
        self.data
    }

    /// Mutable access to the backing buffer.
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl MatView for OwnedMatView {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_matching_length() {
        let v = OwnedMatView::new(2, 3, PixelFormat::Mono8, vec![0u8; 6]).unwrap();
        assert_eq!(v.width(), 2);
        assert_eq!(v.height(), 3);
        assert_eq!(v.channels(), 1);
        assert_eq!(v.pixel_format(), PixelFormat::Mono8);
        assert_eq!(v.data().len(), 6);
    }

    #[test]
    fn new_rejects_mismatched_length() {
        let err = OwnedMatView::new(2, 3, PixelFormat::Bgr8, vec![0u8; 6]).unwrap_err();
        assert!(matches!(err, ImageOpsError::InvalidParameter(_)));
    }

    #[test]
    fn zeros_allocates_zeroed_buffer() {
        let v = OwnedMatView::zeros(4, 2, PixelFormat::Bgr8);
        assert_eq!(v.data().len(), 4 * 2 * 3);
        assert_eq!(v.channels(), 3);
        assert_eq!(v.pixel_format(), PixelFormat::Bgr8);
        assert!(v.data().iter().all(|&b| b == 0));
    }

    #[test]
    fn into_data_returns_buffer() {
        let v = OwnedMatView::new(1, 1, PixelFormat::Mono8, vec![7u8]).unwrap();
        assert_eq!(v.into_data(), vec![7u8]);
    }

    #[test]
    fn data_mut_allows_mutation() {
        let mut v = OwnedMatView::zeros(1, 1, PixelFormat::Mono8);
        v.data_mut()[0] = 42;
        assert_eq!(v.data()[0], 42);
    }
}
