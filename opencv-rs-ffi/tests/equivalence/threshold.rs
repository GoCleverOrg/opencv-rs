//! Equivalence: `threshold` between `PureRustImageOps` and `OpenCvImageOps`.

use opencv_rs_core::{
    ImageOpsPort, MatView, OwnedMatView, PixelFormat, PureRustImageOps, ThresholdKind,
};
use opencv_rs_ffi::OpenCvImageOps;

#[test]
fn binary_threshold_exact() {
    // 8x8 Mono8 ramp so both sides of the threshold are exercised.
    let data: Vec<u8> = (0..64u8).collect();
    let src = OwnedMatView::new(8, 8, PixelFormat::Mono8, data).unwrap();
    let a = PureRustImageOps
        .threshold(&src, 20.0, 255.0, ThresholdKind::Binary)
        .unwrap();
    let b = OpenCvImageOps::new()
        .threshold(&src, 20.0, 255.0, ThresholdKind::Binary)
        .unwrap();
    assert_eq!(a.data(), b.data());
}
