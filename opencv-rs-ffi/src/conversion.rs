//! Internal helpers for moving bytes between [`MatView`] / [`OwnedMatView`]
//! and `opencv::core::Mat`.

use opencv::core::Mat;
use opencv::prelude::*;
use opencv_rs_core::{ImageOpsError, MatView, OwnedMatView, PixelFormat};

use crate::mat_view::slice_to_mat;

/// Build an owned `opencv::core::Mat` from any [`MatView`].
///
/// Copies the view's backing bytes into a fresh `Mat` shaped
/// `height x width` with channel count derived from the view's pixel
/// format.
pub(crate) fn mat_from_view(src: &dyn MatView) -> Result<Mat, ImageOpsError> {
    let rows = i32::try_from(src.height())
        .map_err(|_| ImageOpsError::InvalidParameter("height exceeds i32"))?;
    let cols = i32::try_from(src.width())
        .map_err(|_| ImageOpsError::InvalidParameter("width exceeds i32"))?;
    let channels = i32::try_from(src.channels())
        .map_err(|_| ImageOpsError::InvalidParameter("channels exceeds i32"))?;
    let expected = (rows as usize) * (cols as usize) * (channels as usize);
    if src.data().len() != expected {
        return Err(ImageOpsError::InvalidParameter(
            "MatView data length does not match width*height*channels",
        ));
    }
    // Safety: length is validated above against rows*cols*channels.
    unsafe { slice_to_mat(src.data(), rows, cols, channels) }
        .map_err(|e| ImageOpsError::Backend(e.to_string()))
}

/// Copy the bytes of `mat` into a new [`OwnedMatView`] with the given
/// [`PixelFormat`].
///
/// `mat` must be a `CV_8U` continuous buffer whose channel count agrees
/// with `pixel_format`; these invariants hold for outputs of the
/// wrapped `imgproc` / `core` operations in this crate.
pub(crate) fn owned_view_from_mat(
    mat: &Mat,
    pixel_format: PixelFormat,
) -> Result<OwnedMatView, ImageOpsError> {
    let width = u32::try_from(mat.cols())
        .map_err(|_| ImageOpsError::InvalidParameter("mat cols exceed u32"))?;
    let height = u32::try_from(mat.rows())
        .map_err(|_| ImageOpsError::InvalidParameter("mat rows exceed u32"))?;
    let bytes = mat
        .data_bytes()
        .map_err(|e| ImageOpsError::Backend(e.to_string()))?;
    OwnedMatView::new(width, height, pixel_format, bytes.to_vec())
}
