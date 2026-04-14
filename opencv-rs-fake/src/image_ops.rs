//! Passthrough in-memory implementation of [`ImageOpsPort`] that
//! delegates to [`PureRustImageOps`] for every operation it supports
//! and supplies deterministic fake outputs for the two methods that
//! [`PureRustImageOps`] refuses (`gaussian_blur` and `resize`).

use opencv_rs_core::{
    ColorConversion, ImageOpsError, ImageOpsPort, MatView, MinMaxResult, OwnedMatView,
    PureRustImageOps, ThresholdKind,
};

/// [`ImageOpsPort`] implementation intended for unit tests that want a
/// working port without linking OpenCV.
///
/// Element-wise and reduction methods forward to
/// [`PureRustImageOps`]. For the two methods not implemented in pure
/// Rust, [`ImageOpsPort::gaussian_blur`] returns the input unchanged
/// (valid fake identity), and [`ImageOpsPort::resize`] returns a
/// zero-filled buffer with the requested output dimensions.
#[derive(Debug, Default, Clone, Copy)]
pub struct PassthroughImageOps;

impl ImageOpsPort for PassthroughImageOps {
    fn cvt_color(
        &self,
        src: &dyn MatView,
        conv: ColorConversion,
    ) -> Result<OwnedMatView, ImageOpsError> {
        PureRustImageOps.cvt_color(src, conv)
    }

    fn gaussian_blur(
        &self,
        src: &dyn MatView,
        _ksize: (u32, u32),
        _sigma_x: f64,
        _sigma_y: f64,
    ) -> Result<OwnedMatView, ImageOpsError> {
        OwnedMatView::new(
            src.width(),
            src.height(),
            src.pixel_format(),
            src.data().to_vec(),
        )
    }

    fn threshold(
        &self,
        src: &dyn MatView,
        thresh: f64,
        max_val: f64,
        kind: ThresholdKind,
    ) -> Result<OwnedMatView, ImageOpsError> {
        PureRustImageOps.threshold(src, thresh, max_val, kind)
    }

    fn absdiff(&self, lhs: &dyn MatView, rhs: &dyn MatView) -> Result<OwnedMatView, ImageOpsError> {
        PureRustImageOps.absdiff(lhs, rhs)
    }

    fn convert_scale_abs(
        &self,
        src: &dyn MatView,
        scale: f64,
        offset: f64,
    ) -> Result<OwnedMatView, ImageOpsError> {
        PureRustImageOps.convert_scale_abs(src, scale, offset)
    }

    fn min_max_loc(&self, src: &dyn MatView) -> Result<MinMaxResult, ImageOpsError> {
        PureRustImageOps.min_max_loc(src)
    }

    fn count_non_zero(&self, src: &dyn MatView) -> Result<u64, ImageOpsError> {
        PureRustImageOps.count_non_zero(src)
    }

    fn resize(
        &self,
        src: &dyn MatView,
        new_width: u32,
        new_height: u32,
    ) -> Result<OwnedMatView, ImageOpsError> {
        Ok(OwnedMatView::zeros(
            new_width,
            new_height,
            src.pixel_format(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv_rs_core::PixelFormat;

    #[test]
    fn gaussian_blur_is_identity_fake() {
        let src = OwnedMatView::new(2, 2, PixelFormat::Mono8, vec![1, 2, 3, 4]).unwrap();
        let out = PassthroughImageOps
            .gaussian_blur(&src, (3, 3), 1.0, 1.0)
            .unwrap();
        assert_eq!(out.width(), 2);
        assert_eq!(out.height(), 2);
        assert_eq!(out.pixel_format(), PixelFormat::Mono8);
        assert_eq!(out.data(), &[1, 2, 3, 4]);
    }

    #[test]
    fn resize_returns_zeroed_buffer_with_requested_dims() {
        let src = OwnedMatView::zeros(4, 4, PixelFormat::Bgr8);
        let out = PassthroughImageOps.resize(&src, 2, 3).unwrap();
        assert_eq!(out.width(), 2);
        assert_eq!(out.height(), 3);
        assert_eq!(out.pixel_format(), PixelFormat::Bgr8);
        assert!(out.data().iter().all(|&b| b == 0));
    }

    #[test]
    fn delegates_to_pure_rust_for_cvt_color() {
        let src = OwnedMatView::new(1, 1, PixelFormat::Bgr8, vec![10, 20, 30]).unwrap();
        let out = PassthroughImageOps
            .cvt_color(&src, ColorConversion::BgrToRgb)
            .unwrap();
        assert_eq!(out.data(), &[30, 20, 10]);
    }
}
