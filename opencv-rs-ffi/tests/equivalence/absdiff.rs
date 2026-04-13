//! Equivalence: `absdiff` between `PureRustImageOps` and `OpenCvImageOps`.

use opencv_rs_core::{ImageOpsPort, MatView, OwnedMatView, PixelFormat, PureRustImageOps};
use opencv_rs_ffi::OpenCvImageOps;

#[test]
fn absdiff_mono_exact() {
    let a = OwnedMatView::new(4, 4, PixelFormat::Mono8, (0..16u8).collect::<Vec<_>>()).unwrap();
    let b = OwnedMatView::new(
        4,
        4,
        PixelFormat::Mono8,
        (0..16u8).map(|v| v.wrapping_mul(3)).collect::<Vec<_>>(),
    )
    .unwrap();
    let pr = PureRustImageOps.absdiff(&a, &b).unwrap();
    let cv = OpenCvImageOps::new().absdiff(&a, &b).unwrap();
    assert_eq!(pr.data(), cv.data());
}

#[test]
fn absdiff_bgr_exact() {
    let a = OwnedMatView::new(4, 4, PixelFormat::Bgr8, (0..48u8).collect::<Vec<_>>()).unwrap();
    let b = OwnedMatView::new(
        4,
        4,
        PixelFormat::Bgr8,
        (0..48u8).map(|v| v.wrapping_add(5)).collect::<Vec<_>>(),
    )
    .unwrap();
    let pr = PureRustImageOps.absdiff(&a, &b).unwrap();
    let cv = OpenCvImageOps::new().absdiff(&a, &b).unwrap();
    assert_eq!(pr.data(), cv.data());
}
