//! OpenCV-backed implementation of [`ImageOpsPort`].

use opencv::core::{self, AlgorithmHint, Mat, Point, Size};
use opencv::imgproc;
use opencv_rs_core::{
    ColorConversion, ImageOpsError, ImageOpsPort, MatView, MinMaxResult, OwnedMatView, PixelFormat,
    ThresholdKind,
};

use crate::conversion::{mat_from_view, owned_view_from_mat};

/// Production [`ImageOpsPort`] delegating to `opencv::imgproc` and
/// `opencv::core`.
#[derive(Debug, Default, Clone, Copy)]
pub struct OpenCvImageOps;

impl OpenCvImageOps {
    /// Construct a new [`OpenCvImageOps`].
    pub fn new() -> Self {
        Self
    }
}

fn require_mono(src: &dyn MatView) -> Result<(), ImageOpsError> {
    if src.pixel_format() != PixelFormat::Mono8 {
        return Err(ImageOpsError::UnsupportedPixelFormat(src.pixel_format()));
    }
    Ok(())
}

fn backend<E: std::fmt::Display>(err: E) -> ImageOpsError {
    ImageOpsError::Backend(err.to_string())
}

impl ImageOpsPort for OpenCvImageOps {
    fn cvt_color(
        &self,
        src: &dyn MatView,
        conv: ColorConversion,
    ) -> Result<OwnedMatView, ImageOpsError> {
        let (code, expected_src, out_pf) = match conv {
            ColorConversion::BgrToRgb => {
                (imgproc::COLOR_BGR2RGB, PixelFormat::Bgr8, PixelFormat::Rgb8)
            }
            ColorConversion::RgbToBgr => {
                (imgproc::COLOR_RGB2BGR, PixelFormat::Rgb8, PixelFormat::Bgr8)
            }
            ColorConversion::BgrToGray => (
                imgproc::COLOR_BGR2GRAY,
                PixelFormat::Bgr8,
                PixelFormat::Mono8,
            ),
            ColorConversion::GrayToRgb => (
                imgproc::COLOR_GRAY2RGB,
                PixelFormat::Mono8,
                PixelFormat::Rgb8,
            ),
        };
        if src.pixel_format() != expected_src {
            return Err(ImageOpsError::UnsupportedConversion {
                src: src.pixel_format(),
                dst: out_pf,
            });
        }
        let src_mat = mat_from_view(src)?;
        let mut dst = Mat::default();
        imgproc::cvt_color(
            &src_mat,
            &mut dst,
            code,
            0,
            AlgorithmHint::ALGO_HINT_DEFAULT,
        )
        .map_err(backend)?;
        owned_view_from_mat(&dst, out_pf)
    }

    fn gaussian_blur(
        &self,
        src: &dyn MatView,
        ksize: (u32, u32),
        sigma_x: f64,
        sigma_y: f64,
    ) -> Result<OwnedMatView, ImageOpsError> {
        let src_mat = mat_from_view(src)?;
        let mut dst = Mat::default();
        let kw = i32::try_from(ksize.0).map_err(|_| {
            ImageOpsError::InvalidParameter("gaussian_blur: ksize width exceeds i32")
        })?;
        let kh = i32::try_from(ksize.1).map_err(|_| {
            ImageOpsError::InvalidParameter("gaussian_blur: ksize height exceeds i32")
        })?;
        imgproc::gaussian_blur(
            &src_mat,
            &mut dst,
            Size::new(kw, kh),
            sigma_x,
            sigma_y,
            core::BORDER_DEFAULT,
            AlgorithmHint::ALGO_HINT_DEFAULT,
        )
        .map_err(backend)?;
        owned_view_from_mat(&dst, src.pixel_format())
    }

    fn threshold(
        &self,
        src: &dyn MatView,
        thresh: f64,
        max_val: f64,
        kind: ThresholdKind,
    ) -> Result<OwnedMatView, ImageOpsError> {
        require_mono(src)?;
        let src_mat = mat_from_view(src)?;
        let mut dst = Mat::default();
        let typ = match kind {
            ThresholdKind::Binary => imgproc::THRESH_BINARY,
        };
        imgproc::threshold(&src_mat, &mut dst, thresh, max_val, typ).map_err(backend)?;
        owned_view_from_mat(&dst, PixelFormat::Mono8)
    }

    fn absdiff(&self, lhs: &dyn MatView, rhs: &dyn MatView) -> Result<OwnedMatView, ImageOpsError> {
        let ld = (lhs.width(), lhs.height(), lhs.channels());
        let rd = (rhs.width(), rhs.height(), rhs.channels());
        if ld != rd || lhs.pixel_format() != rhs.pixel_format() {
            return Err(ImageOpsError::DimensionMismatch { lhs: ld, rhs: rd });
        }
        let a = mat_from_view(lhs)?;
        let b = mat_from_view(rhs)?;
        let mut dst = Mat::default();
        core::absdiff(&a, &b, &mut dst).map_err(backend)?;
        owned_view_from_mat(&dst, lhs.pixel_format())
    }

    fn convert_scale_abs(
        &self,
        src: &dyn MatView,
        scale: f64,
        offset: f64,
    ) -> Result<OwnedMatView, ImageOpsError> {
        let src_mat = mat_from_view(src)?;
        let mut dst = Mat::default();
        core::convert_scale_abs(&src_mat, &mut dst, scale, offset).map_err(backend)?;
        owned_view_from_mat(&dst, src.pixel_format())
    }

    fn min_max_loc(&self, src: &dyn MatView) -> Result<MinMaxResult, ImageOpsError> {
        require_mono(src)?;
        if src.data().is_empty() {
            return Err(ImageOpsError::EmptyInput);
        }
        let src_mat = mat_from_view(src)?;
        let mut min_v: f64 = 0.0;
        let mut max_v: f64 = 0.0;
        let mut min_loc = Point::new(0, 0);
        let mut max_loc = Point::new(0, 0);
        let mask = Mat::default();
        core::min_max_loc(
            &src_mat,
            Some(&mut min_v),
            Some(&mut max_v),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &mask,
        )
        .map_err(backend)?;
        let to_u32 = |v: i32| -> Result<u32, ImageOpsError> {
            u32::try_from(v).map_err(|_| {
                ImageOpsError::Backend(format!("min_max_loc returned negative coord {v}"))
            })
        };
        Ok(MinMaxResult {
            min: min_v,
            max: max_v,
            min_loc: (to_u32(min_loc.x)?, to_u32(min_loc.y)?),
            max_loc: (to_u32(max_loc.x)?, to_u32(max_loc.y)?),
        })
    }

    fn count_non_zero(&self, src: &dyn MatView) -> Result<u64, ImageOpsError> {
        require_mono(src)?;
        let src_mat = mat_from_view(src)?;
        let n = core::count_non_zero(&src_mat).map_err(backend)?;
        Ok(n.max(0) as u64)
    }

    fn resize(
        &self,
        src: &dyn MatView,
        new_width: u32,
        new_height: u32,
    ) -> Result<OwnedMatView, ImageOpsError> {
        let src_mat = mat_from_view(src)?;
        let mut dst = Mat::default();
        let w = i32::try_from(new_width)
            .map_err(|_| ImageOpsError::InvalidParameter("resize: new_width exceeds i32"))?;
        let h = i32::try_from(new_height)
            .map_err(|_| ImageOpsError::InvalidParameter("resize: new_height exceeds i32"))?;
        imgproc::resize(
            &src_mat,
            &mut dst,
            Size::new(w, h),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )
        .map_err(backend)?;
        owned_view_from_mat(&dst, src.pixel_format())
    }
}
