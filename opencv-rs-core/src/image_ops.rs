//! Image operations port and its pure-Rust implementation.

use crate::{ImageOpsError, MatView, OwnedMatView, PixelFormat};

/// Supported color-space conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorConversion {
    /// Swap channels 0 and 2 of a 3-channel BGR image to produce RGB.
    BgrToRgb,
    /// Swap channels 0 and 2 of a 3-channel RGB image to produce BGR.
    RgbToBgr,
    /// Convert a BGR image to single-channel grayscale using OpenCV weights.
    BgrToGray,
    /// Broadcast a grayscale image to 3-channel RGB.
    GrayToRgb,
}

/// Thresholding algorithm selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdKind {
    /// Binary threshold: pixels strictly greater than `thresh` become `max_val`, others become 0.
    Binary,
}

/// Result of [`ImageOpsPort::min_max_loc`].
#[derive(Debug, Clone, Copy)]
pub struct MinMaxResult {
    /// Minimum pixel value encountered.
    pub min: f64,
    /// Maximum pixel value encountered.
    pub max: f64,
    /// Location `(x, y)` of the first pixel equal to [`Self::min`].
    pub min_loc: (u32, u32),
    /// Location `(x, y)` of the first pixel equal to [`Self::max`].
    pub max_loc: (u32, u32),
}

/// Port exposing element-wise and reduction image operations.
pub trait ImageOpsPort: Send + Sync {
    /// Convert `src` between color spaces per [`ColorConversion`].
    fn cvt_color(
        &self,
        src: &dyn MatView,
        conv: ColorConversion,
    ) -> Result<OwnedMatView, ImageOpsError>;
    /// Apply a Gaussian blur with the given kernel size and sigmas.
    fn gaussian_blur(
        &self,
        src: &dyn MatView,
        ksize: (u32, u32),
        sigma_x: f64,
        sigma_y: f64,
    ) -> Result<OwnedMatView, ImageOpsError>;
    /// Threshold a grayscale image.
    fn threshold(
        &self,
        src: &dyn MatView,
        thresh: f64,
        max_val: f64,
        kind: ThresholdKind,
    ) -> Result<OwnedMatView, ImageOpsError>;
    /// Element-wise absolute difference of two identically shaped images.
    fn absdiff(&self, lhs: &dyn MatView, rhs: &dyn MatView) -> Result<OwnedMatView, ImageOpsError>;
    /// Element-wise `saturate_u8(|src * scale + offset|)`.
    fn convert_scale_abs(
        &self,
        src: &dyn MatView,
        scale: f64,
        offset: f64,
    ) -> Result<OwnedMatView, ImageOpsError>;
    /// Locate the minimum and maximum pixel values and their first positions.
    fn min_max_loc(&self, src: &dyn MatView) -> Result<MinMaxResult, ImageOpsError>;
    /// Count non-zero pixels in a grayscale image.
    fn count_non_zero(&self, src: &dyn MatView) -> Result<u64, ImageOpsError>;
    /// Resize `src` to the given dimensions.
    fn resize(
        &self,
        src: &dyn MatView,
        new_width: u32,
        new_height: u32,
    ) -> Result<OwnedMatView, ImageOpsError>;
}

/// Pure-Rust implementation of element-wise and reduction [`ImageOpsPort`] methods.
///
/// `gaussian_blur` and `resize` return
/// `ImageOpsError::Backend("unsupported in PureRustImageOps: requires OpenCV backend")`;
/// they are outside scope for pure-Rust reimplementation per issue #1.
#[derive(Debug, Default, Clone, Copy)]
pub struct PureRustImageOps;

/// Saturating rounding conversion of `v` to `u8`.
fn sat_u8(v: f64) -> u8 {
    v.round().clamp(0.0, 255.0) as u8
}

fn dims(view: &dyn MatView) -> (u32, u32, u32) {
    (view.width(), view.height(), view.channels())
}

impl ImageOpsPort for PureRustImageOps {
    fn cvt_color(
        &self,
        src: &dyn MatView,
        conv: ColorConversion,
    ) -> Result<OwnedMatView, ImageOpsError> {
        let w = src.width();
        let h = src.height();
        let pf = src.pixel_format();
        let data = src.data();
        match conv {
            ColorConversion::BgrToRgb | ColorConversion::RgbToBgr => {
                let (expected_src, out_pf) = match conv {
                    ColorConversion::BgrToRgb => (PixelFormat::Bgr8, PixelFormat::Rgb8),
                    ColorConversion::RgbToBgr => (PixelFormat::Rgb8, PixelFormat::Bgr8),
                    ColorConversion::BgrToGray | ColorConversion::GrayToRgb => unreachable!(),
                };
                if pf != expected_src {
                    return Err(ImageOpsError::UnsupportedConversion {
                        src: pf,
                        dst: out_pf,
                    });
                }
                let mut out = vec![0u8; data.len()];
                for (i, chunk) in out.chunks_exact_mut(3).enumerate() {
                    let src = i * 3;
                    chunk[0] = data[src + 2];
                    chunk[1] = data[src + 1];
                    chunk[2] = data[src];
                }
                OwnedMatView::new(w, h, out_pf, out)
            }
            ColorConversion::BgrToGray => {
                if pf != PixelFormat::Bgr8 {
                    return Err(ImageOpsError::UnsupportedConversion {
                        src: pf,
                        dst: PixelFormat::Mono8,
                    });
                }
                let out: Vec<u8> = data
                    .chunks_exact(3)
                    .map(|px| {
                        let b = px[0] as f64;
                        let g = px[1] as f64;
                        let r = px[2] as f64;
                        sat_u8(0.114 * b + 0.587 * g + 0.299 * r)
                    })
                    .collect();
                OwnedMatView::new(w, h, PixelFormat::Mono8, out)
            }
            ColorConversion::GrayToRgb => {
                if pf != PixelFormat::Mono8 {
                    return Err(ImageOpsError::UnsupportedConversion {
                        src: pf,
                        dst: PixelFormat::Rgb8,
                    });
                }
                let out: Vec<u8> = data.iter().flat_map(|&v| [v, v, v]).collect();
                OwnedMatView::new(w, h, PixelFormat::Rgb8, out)
            }
        }
    }

    fn gaussian_blur(
        &self,
        _src: &dyn MatView,
        _ksize: (u32, u32),
        _sigma_x: f64,
        _sigma_y: f64,
    ) -> Result<OwnedMatView, ImageOpsError> {
        Err(ImageOpsError::Backend(
            "unsupported in PureRustImageOps: requires OpenCV backend".to_string(),
        ))
    }

    fn threshold(
        &self,
        src: &dyn MatView,
        thresh: f64,
        max_val: f64,
        kind: ThresholdKind,
    ) -> Result<OwnedMatView, ImageOpsError> {
        if src.pixel_format() != PixelFormat::Mono8 {
            return Err(ImageOpsError::UnsupportedPixelFormat(src.pixel_format()));
        }
        let data = src.data();
        let max_u8 = sat_u8(max_val);
        let out = match kind {
            ThresholdKind::Binary => data
                .iter()
                .map(|&b| if (b as f64) > thresh { max_u8 } else { 0 })
                .collect::<Vec<u8>>(),
        };
        OwnedMatView::new(src.width(), src.height(), PixelFormat::Mono8, out)
    }

    fn absdiff(&self, lhs: &dyn MatView, rhs: &dyn MatView) -> Result<OwnedMatView, ImageOpsError> {
        let ld = dims(lhs);
        let rd = dims(rhs);
        if ld != rd || lhs.pixel_format() != rhs.pixel_format() {
            return Err(ImageOpsError::DimensionMismatch { lhs: ld, rhs: rd });
        }
        let a = lhs.data();
        let b = rhs.data();
        if a.len() != b.len() {
            return Err(ImageOpsError::DimensionMismatch { lhs: ld, rhs: rd });
        }
        let out: Vec<u8> = a
            .iter()
            .zip(b.iter())
            .map(|(&x, &y)| x.abs_diff(y))
            .collect();
        OwnedMatView::new(lhs.width(), lhs.height(), lhs.pixel_format(), out)
    }

    fn convert_scale_abs(
        &self,
        src: &dyn MatView,
        scale: f64,
        offset: f64,
    ) -> Result<OwnedMatView, ImageOpsError> {
        let out: Vec<u8> = src
            .data()
            .iter()
            .map(|&b| sat_u8((b as f64 * scale + offset).abs()))
            .collect();
        OwnedMatView::new(src.width(), src.height(), src.pixel_format(), out)
    }

    fn min_max_loc(&self, src: &dyn MatView) -> Result<MinMaxResult, ImageOpsError> {
        if src.pixel_format() != PixelFormat::Mono8 {
            return Err(ImageOpsError::UnsupportedPixelFormat(src.pixel_format()));
        }
        let data = src.data();
        if data.is_empty() {
            return Err(ImageOpsError::EmptyInput);
        }
        let w = src.width();
        let mut min_v: u8 = data[0];
        let mut max_v: u8 = data[0];
        let mut min_loc = (0u32, 0u32);
        let mut max_loc = (0u32, 0u32);
        for (idx, &b) in data.iter().enumerate() {
            let x = (idx as u32) % w;
            let y = (idx as u32) / w;
            if b < min_v {
                min_v = b;
                min_loc = (x, y);
            }
            if b > max_v {
                max_v = b;
                max_loc = (x, y);
            }
        }
        Ok(MinMaxResult {
            min: min_v as f64,
            max: max_v as f64,
            min_loc,
            max_loc,
        })
    }

    fn count_non_zero(&self, src: &dyn MatView) -> Result<u64, ImageOpsError> {
        if src.pixel_format() != PixelFormat::Mono8 {
            return Err(ImageOpsError::UnsupportedPixelFormat(src.pixel_format()));
        }
        Ok(src.data().iter().filter(|&&b| b != 0).count() as u64)
    }

    fn resize(
        &self,
        _src: &dyn MatView,
        _new_width: u32,
        _new_height: u32,
    ) -> Result<OwnedMatView, ImageOpsError> {
        Err(ImageOpsError::Backend(
            "unsupported in PureRustImageOps: requires OpenCV backend".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mono(w: u32, h: u32, data: Vec<u8>) -> OwnedMatView {
        OwnedMatView::new(w, h, PixelFormat::Mono8, data).unwrap()
    }
    fn bgr(w: u32, h: u32, data: Vec<u8>) -> OwnedMatView {
        OwnedMatView::new(w, h, PixelFormat::Bgr8, data).unwrap()
    }

    #[test]
    fn threshold_binary_basic() {
        let src = mono(3, 1, vec![10, 50, 200]);
        let out = PureRustImageOps
            .threshold(&src, 30.0, 255.0, ThresholdKind::Binary)
            .unwrap();
        assert_eq!(out.data(), &[0, 255, 255]);
    }

    #[test]
    fn threshold_binary_equal_to_threshold_is_zero() {
        // Pixel exactly at threshold must NOT exceed it (original uses `>`).
        let src = mono(3, 1, vec![29, 30, 31]);
        let out = PureRustImageOps
            .threshold(&src, 30.0, 255.0, ThresholdKind::Binary)
            .unwrap();
        assert_eq!(out.data(), &[0, 0, 255]);
    }

    #[test]
    fn threshold_rejects_non_mono() {
        let src = bgr(1, 1, vec![1, 2, 3]);
        let err = PureRustImageOps
            .threshold(&src, 0.0, 255.0, ThresholdKind::Binary)
            .unwrap_err();
        assert!(matches!(err, ImageOpsError::UnsupportedPixelFormat(_)));
    }

    #[test]
    fn absdiff_mono() {
        let a = mono(2, 2, vec![10, 20, 30, 40]);
        let b = mono(2, 2, vec![5, 25, 30, 100]);
        let out = PureRustImageOps.absdiff(&a, &b).unwrap();
        assert_eq!(out.data(), &[5, 5, 0, 60]);
    }

    #[test]
    fn absdiff_rejects_mismatched_dims() {
        let a = mono(2, 2, vec![0; 4]);
        let b = mono(2, 3, vec![0; 6]);
        let err = PureRustImageOps.absdiff(&a, &b).unwrap_err();
        assert!(matches!(err, ImageOpsError::DimensionMismatch { .. }));
    }

    #[test]
    fn absdiff_rejects_mismatched_format() {
        let a = mono(1, 1, vec![0]);
        let b = bgr(1, 1, vec![0, 0, 0]);
        let err = PureRustImageOps.absdiff(&a, &b).unwrap_err();
        assert!(matches!(err, ImageOpsError::DimensionMismatch { .. }));
    }

    #[test]
    fn absdiff_rejects_same_dims_different_format() {
        // Bgr8 and Rgb8 both have channels=3, so dims() tuples match; only
        // pixel_format differs. Kills the `||` -> `&&` mutant on the guard.
        let a = bgr(1, 1, vec![1, 2, 3]);
        let b = OwnedMatView::new(1, 1, PixelFormat::Rgb8, vec![1, 2, 3]).unwrap();
        let err = PureRustImageOps.absdiff(&a, &b).unwrap_err();
        assert!(matches!(err, ImageOpsError::DimensionMismatch { .. }));
    }

    #[test]
    fn absdiff_rejects_when_dims_differ_but_byte_count_matches() {
        // 4x1 and 2x2 Mono8 both have 4 bytes, so the length fallback check
        // would accept them. Kills the `dims -> const` mutants by forcing the
        // dims() inequality check to be the decisive gate.
        let a = mono(4, 1, vec![1, 2, 3, 4]);
        let b = mono(2, 2, vec![1, 2, 3, 4]);
        let err = PureRustImageOps.absdiff(&a, &b).unwrap_err();
        assert!(matches!(err, ImageOpsError::DimensionMismatch { .. }));
    }

    #[test]
    fn cvt_color_bgr_to_rgb_swaps_channels() {
        // Multi-pixel input so the per-chunk offset `i * 3` varies with i;
        // kills the `*` -> `/` mutant on the offset calculation.
        let src = bgr(2, 1, vec![1, 2, 3, 10, 20, 30]);
        let out = PureRustImageOps
            .cvt_color(&src, ColorConversion::BgrToRgb)
            .unwrap();
        assert_eq!(out.pixel_format(), PixelFormat::Rgb8);
        assert_eq!(out.data(), &[3, 2, 1, 30, 20, 10]);
    }

    #[test]
    fn cvt_color_rgb_to_bgr_swaps_channels() {
        let src = OwnedMatView::new(1, 1, PixelFormat::Rgb8, vec![1, 2, 3]).unwrap();
        let out = PureRustImageOps
            .cvt_color(&src, ColorConversion::RgbToBgr)
            .unwrap();
        assert_eq!(out.pixel_format(), PixelFormat::Bgr8);
        assert_eq!(out.data(), &[3, 2, 1]);
    }

    #[test]
    fn cvt_color_bgr_to_gray_uses_opencv_weights() {
        // Pure red in BGR is (0,0,255); gray = 0.299 * 255 = 76.245 -> 76.
        let src = bgr(1, 1, vec![0, 0, 255]);
        let out = PureRustImageOps
            .cvt_color(&src, ColorConversion::BgrToGray)
            .unwrap();
        assert_eq!(out.pixel_format(), PixelFormat::Mono8);
        assert_eq!(out.data(), &[76]);
    }

    #[test]
    fn cvt_color_bgr_to_gray_all_channels_nonzero() {
        // All three weights contribute distinctly; kills the arithmetic
        // mutants on the weighted sum (+ <-> -, + <-> *, * <-> +).
        // 0.114*100 + 0.587*200 + 0.299*50 = 11.4 + 117.4 + 14.95 = 143.75 -> 144.
        let src = bgr(1, 1, vec![100, 200, 50]);
        let out = PureRustImageOps
            .cvt_color(&src, ColorConversion::BgrToGray)
            .unwrap();
        assert_eq!(out.data(), &[144]);
    }

    #[test]
    fn cvt_color_gray_to_rgb_broadcasts() {
        let src = mono(2, 1, vec![10, 200]);
        let out = PureRustImageOps
            .cvt_color(&src, ColorConversion::GrayToRgb)
            .unwrap();
        assert_eq!(out.pixel_format(), PixelFormat::Rgb8);
        assert_eq!(out.data(), &[10, 10, 10, 200, 200, 200]);
    }

    #[test]
    fn cvt_color_rejects_wrong_input_format() {
        let src = mono(1, 1, vec![5]);
        let err = PureRustImageOps
            .cvt_color(&src, ColorConversion::BgrToRgb)
            .unwrap_err();
        assert!(matches!(err, ImageOpsError::UnsupportedConversion { .. }));
    }

    #[test]
    fn count_non_zero_counts_bytes() {
        // Asymmetric zero/non-zero split so the `!= 0` filter cannot be
        // mutated to `== 0` without changing the expected result.
        let src = mono(3, 1, vec![0, 1, 2]);
        assert_eq!(PureRustImageOps.count_non_zero(&src).unwrap(), 2);
    }

    #[test]
    fn count_non_zero_rejects_non_mono() {
        let src = bgr(1, 1, vec![0, 0, 0]);
        let err = PureRustImageOps.count_non_zero(&src).unwrap_err();
        assert!(matches!(err, ImageOpsError::UnsupportedPixelFormat(_)));
    }

    #[test]
    fn min_max_loc_returns_first_occurrence() {
        // 3x2 image, row-major: values 5, 7, 1, 3, 7, 1.
        // min=1 first at idx 2 -> (x=2, y=0); max=7 first at idx 1 -> (x=1, y=0).
        let src = mono(3, 2, vec![5, 7, 1, 3, 7, 1]);
        let r = PureRustImageOps.min_max_loc(&src).unwrap();
        assert_eq!(r.min, 1.0);
        assert_eq!(r.max, 7.0);
        assert_eq!(r.min_loc, (2, 0));
        assert_eq!(r.max_loc, (1, 0));
    }

    #[test]
    fn min_max_loc_first_occurrence_with_duplicates_and_multi_row() {
        // Min value 1 appears at idx 4,5,6 -> first at (0,1). Max value 8 only at idx 7 -> (3,1).
        let src = mono(4, 2, vec![5, 5, 7, 7, 1, 1, 1, 8]);
        let r = PureRustImageOps.min_max_loc(&src).unwrap();
        assert_eq!(r.min, 1.0);
        assert_eq!(r.min_loc, (0, 1));
        assert_eq!(r.max, 8.0);
        assert_eq!(r.max_loc, (3, 1));
    }

    #[test]
    fn min_max_loc_empty_input_errors() {
        let src = mono(0, 0, vec![]);
        let err = PureRustImageOps.min_max_loc(&src).unwrap_err();
        assert!(matches!(err, ImageOpsError::EmptyInput));
    }

    #[test]
    fn min_max_loc_rejects_non_mono() {
        let src = bgr(1, 1, vec![1, 2, 3]);
        let err = PureRustImageOps.min_max_loc(&src).unwrap_err();
        assert!(matches!(err, ImageOpsError::UnsupportedPixelFormat(_)));
    }

    #[test]
    fn convert_scale_abs_scales_and_saturates() {
        // 10*0.5=5; 20*0.5=10; 240*0.5=120. Abs no-op for non-negative.
        let src = mono(3, 1, vec![10, 20, 240]);
        let out = PureRustImageOps.convert_scale_abs(&src, 0.5, 0.0).unwrap();
        assert_eq!(out.data(), &[5, 10, 120]);
    }

    #[test]
    fn convert_scale_abs_takes_absolute_value() {
        // -1 is impossible for u8, so use negative scale. 10 * -2 = -20, abs=20.
        let src = mono(1, 1, vec![10]);
        let out = PureRustImageOps.convert_scale_abs(&src, -2.0, 0.0).unwrap();
        assert_eq!(out.data(), &[20]);
    }

    #[test]
    fn convert_scale_abs_requires_abs_for_negative_intermediate() {
        // 255 * -1 + -100 = -355; abs() -> 355 -> saturates to 255.
        // Without abs() the result would saturate to 0.
        let src = mono(1, 1, vec![255]);
        let out = PureRustImageOps
            .convert_scale_abs(&src, -1.0, -100.0)
            .unwrap();
        assert_eq!(out.data(), &[255]);
    }

    #[test]
    fn convert_scale_abs_saturates_high() {
        let src = mono(1, 1, vec![200]);
        let out = PureRustImageOps.convert_scale_abs(&src, 2.0, 0.0).unwrap();
        assert_eq!(out.data(), &[255]);
    }

    #[test]
    fn gaussian_blur_returns_backend_error() {
        let src = mono(1, 1, vec![0]);
        let err = PureRustImageOps
            .gaussian_blur(&src, (3, 3), 0.0, 0.0)
            .unwrap_err();
        assert!(matches!(err, ImageOpsError::Backend(_)));
    }

    #[test]
    fn resize_returns_backend_error() {
        let src = mono(1, 1, vec![0]);
        let err = PureRustImageOps.resize(&src, 2, 2).unwrap_err();
        assert!(matches!(err, ImageOpsError::Backend(_)));
    }
}
