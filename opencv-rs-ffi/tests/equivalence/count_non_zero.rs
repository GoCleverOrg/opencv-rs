//! Equivalence: `count_non_zero` between `PureRustImageOps` and `OpenCvImageOps`.

use opencv_rs_core::{ImageOpsPort, OwnedMatView, PixelFormat, PureRustImageOps};
use opencv_rs_ffi::OpenCvImageOps;

#[test]
fn count_non_zero_mono_exact() {
    // Mix of zero and non-zero bytes.
    let data: Vec<u8> = (0..64u8).map(|i| if i % 3 == 0 { 0 } else { i }).collect();
    let src = OwnedMatView::new(8, 8, PixelFormat::Mono8, data).unwrap();
    let pr = PureRustImageOps.count_non_zero(&src).unwrap();
    let cv = OpenCvImageOps::new().count_non_zero(&src).unwrap();
    assert_eq!(pr, cv);
}
