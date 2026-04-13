//! Equivalence: `cvt_color` between `PureRustImageOps` and `OpenCvImageOps`.

use opencv_rs_core::{
    ColorConversion, ImageOpsPort, MatView, OwnedMatView, PixelFormat, PureRustImageOps,
};
use opencv_rs_ffi::OpenCvImageOps;

fn bgr_fixture() -> OwnedMatView {
    // 4x4 BGR with varied colors.
    let mut data = Vec::with_capacity(4 * 4 * 3);
    for i in 0..16u8 {
        data.push(i.wrapping_mul(7));
        data.push(i.wrapping_mul(11));
        data.push(i.wrapping_mul(13));
    }
    OwnedMatView::new(4, 4, PixelFormat::Bgr8, data).unwrap()
}

fn gray_fixture() -> OwnedMatView {
    let data: Vec<u8> = (0..64u8).collect();
    OwnedMatView::new(8, 8, PixelFormat::Mono8, data).unwrap()
}

fn max_abs_diff(a: &[u8], b: &[u8]) -> u8 {
    assert_eq!(a.len(), b.len(), "length mismatch");
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| x.abs_diff(y))
        .max()
        .unwrap_or(0)
}

#[test]
fn bgr_to_rgb_exact() {
    let src = bgr_fixture();
    let a = PureRustImageOps
        .cvt_color(&src, ColorConversion::BgrToRgb)
        .unwrap();
    let b = OpenCvImageOps::new()
        .cvt_color(&src, ColorConversion::BgrToRgb)
        .unwrap();
    assert_eq!(a.pixel_format(), b.pixel_format());
    assert_eq!(a.data(), b.data());
}

#[test]
fn rgb_to_bgr_exact() {
    let src_data = bgr_fixture().into_data();
    let src = OwnedMatView::new(4, 4, PixelFormat::Rgb8, src_data).unwrap();
    let a = PureRustImageOps
        .cvt_color(&src, ColorConversion::RgbToBgr)
        .unwrap();
    let b = OpenCvImageOps::new()
        .cvt_color(&src, ColorConversion::RgbToBgr)
        .unwrap();
    assert_eq!(a.data(), b.data());
}

#[test]
fn gray_to_rgb_exact() {
    let src = gray_fixture();
    let a = PureRustImageOps
        .cvt_color(&src, ColorConversion::GrayToRgb)
        .unwrap();
    let b = OpenCvImageOps::new()
        .cvt_color(&src, ColorConversion::GrayToRgb)
        .unwrap();
    assert_eq!(a.data(), b.data());
}

/// BGR→Gray: OpenCV uses fixed-point arithmetic with banker's rounding
/// while PureRustImageOps uses the textbook f64 weights and
/// `f64::round` (round-half-away-from-zero). These conventions can
/// differ by 1 on individual pixels; we assert max|Δ| ≤ 1.
#[test]
fn bgr_to_gray_within_one() {
    let src = bgr_fixture();
    let a = PureRustImageOps
        .cvt_color(&src, ColorConversion::BgrToGray)
        .unwrap();
    let b = OpenCvImageOps::new()
        .cvt_color(&src, ColorConversion::BgrToGray)
        .unwrap();
    assert_eq!(a.pixel_format(), b.pixel_format());
    assert!(
        max_abs_diff(a.data(), b.data()) <= 1,
        "bgr_to_gray differs by more than 1: pure={:?} opencv={:?}",
        a.data(),
        b.data()
    );
}
