//! Shared DTOs used across the `opencv-rs` ports.

/// Pixel layout for a [`crate::MatView`]'s buffer.
///
/// Only the variants consumed by the mira workspace are defined here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8-bit single-channel grayscale.
    Mono8,
    /// 8-bit 3-channel blue-green-red interleaved (OpenCV default).
    Bgr8,
    /// 8-bit 3-channel red-green-blue interleaved.
    Rgb8,
}

impl PixelFormat {
    /// Number of channels per pixel for this format.
    pub const fn channels(self) -> u32 {
        match self {
            Self::Mono8 => 1,
            Self::Bgr8 | Self::Rgb8 => 3,
        }
    }

    /// Bytes occupied by a single pixel (one byte per channel for 8-bit formats).
    pub const fn bytes_per_pixel(self) -> u32 {
        self.channels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_counts_match_format() {
        assert_eq!(PixelFormat::Mono8.channels(), 1);
        assert_eq!(PixelFormat::Bgr8.channels(), 3);
        assert_eq!(PixelFormat::Rgb8.channels(), 3);
    }

    #[test]
    fn bytes_per_pixel_matches_channels() {
        assert_eq!(PixelFormat::Mono8.bytes_per_pixel(), 1);
        assert_eq!(PixelFormat::Bgr8.bytes_per_pixel(), 3);
        assert_eq!(PixelFormat::Rgb8.bytes_per_pixel(), 3);
    }
}
