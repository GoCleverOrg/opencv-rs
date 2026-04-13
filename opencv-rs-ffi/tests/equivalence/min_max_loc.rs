//! Equivalence: `min_max_loc` between `PureRustImageOps` and `OpenCvImageOps`.

use opencv_rs_core::{ImageOpsPort, OwnedMatView, PixelFormat, PureRustImageOps};
use opencv_rs_ffi::OpenCvImageOps;

#[test]
fn min_max_loc_mono_exact() {
    // 3x2 with a clear single min and a single max so both
    // implementations must agree on the first-occurrence coordinates.
    let data = vec![5u8, 7, 1, 3, 6, 2];
    let src = OwnedMatView::new(3, 2, PixelFormat::Mono8, data).unwrap();
    let pr = PureRustImageOps.min_max_loc(&src).unwrap();
    let cv = OpenCvImageOps::new().min_max_loc(&src).unwrap();
    assert_eq!(pr.min, cv.min);
    assert_eq!(pr.max, cv.max);
    assert_eq!(pr.min_loc, cv.min_loc);
    assert_eq!(pr.max_loc, cv.max_loc);
}
