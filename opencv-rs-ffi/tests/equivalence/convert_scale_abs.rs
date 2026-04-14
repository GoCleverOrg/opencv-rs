//! Equivalence: `convert_scale_abs` between `PureRustImageOps` and
//! `OpenCvImageOps`.
//!
//! OpenCV's `convertScaleAbs` uses `cvRound` internally (banker's
//! rounding to even on `.5` ties), while `PureRustImageOps` uses
//! `f64::round` (round half away from zero). We assert a per-byte
//! tolerance of at most 1.

use opencv_rs_core::{ImageOpsPort, MatView, OwnedMatView, PixelFormat, PureRustImageOps};
use opencv_rs_ffi::OpenCvImageOps;

fn max_abs_diff(a: &[u8], b: &[u8]) -> u8 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| x.abs_diff(y))
        .max()
        .unwrap_or(0)
}

#[test]
fn convert_scale_abs_within_one() {
    let data: Vec<u8> = (0..64u8).collect();
    let src = OwnedMatView::new(8, 8, PixelFormat::Mono8, data).unwrap();
    let pr = PureRustImageOps.convert_scale_abs(&src, 0.5, 0.0).unwrap();
    let cv = OpenCvImageOps::new()
        .convert_scale_abs(&src, 0.5, 0.0)
        .unwrap();
    assert!(
        max_abs_diff(pr.data(), cv.data()) <= 1,
        "convert_scale_abs differs by more than 1"
    );
}

#[test]
fn convert_scale_abs_negative_scale_within_one() {
    let data: Vec<u8> = (0..64u8).collect();
    let src = OwnedMatView::new(8, 8, PixelFormat::Mono8, data).unwrap();
    let pr = PureRustImageOps
        .convert_scale_abs(&src, -1.5, 10.0)
        .unwrap();
    let cv = OpenCvImageOps::new()
        .convert_scale_abs(&src, -1.5, 10.0)
        .unwrap();
    assert!(max_abs_diff(pr.data(), cv.data()) <= 1);
}
