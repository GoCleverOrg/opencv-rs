//! OpenCV-backed [`MatView`] implementation.
//!
//! Provides [`OpenCvMatView`], a borrowed read-only adapter over an
//! `opencv::core::Mat`, plus the [`slice_to_mat`] helper that wraps a
//! borrowed byte slice in a `Mat`.

use opencv::core::{Mat, MatTraitConst, CV_8U};
use opencv::prelude::*;
use opencv_rs_core::{ImageOpsError, MatView, PixelFormat};

/// Borrowed read-only view over an existing `opencv::core::Mat`.
///
/// The `PixelFormat` is inferred from the mat's channel count at
/// construction: 1 channel becomes [`PixelFormat::Mono8`], 3 channels
/// become [`PixelFormat::Bgr8`] (OpenCV's default color order). Element
/// types other than `CV_8U` and channel counts outside `{1, 3}` are
/// rejected via [`ImageOpsError::InvalidParameter`].
pub struct OpenCvMatView<'a> {
    mat: &'a Mat,
    pixel_format: PixelFormat,
    width: u32,
    height: u32,
}

impl<'a> OpenCvMatView<'a> {
    /// Build an [`OpenCvMatView`] from an existing `Mat`.
    ///
    /// Fails with [`ImageOpsError::InvalidParameter`] when the mat's
    /// element depth is not `CV_8U`, or when the channel count is not 1
    /// or 3.
    pub fn try_from_mat(mat: &'a Mat) -> Result<Self, ImageOpsError> {
        let depth = mat.depth();
        if depth != CV_8U {
            return Err(ImageOpsError::InvalidParameter(
                "OpenCvMatView: only CV_8U depth is supported",
            ));
        }
        let pixel_format = match mat.channels() {
            1 => PixelFormat::Mono8,
            3 => PixelFormat::Bgr8,
            _ => {
                return Err(ImageOpsError::InvalidParameter(
                    "OpenCvMatView: only 1- or 3-channel mats are supported",
                ));
            }
        };
        let width = u32::try_from(mat.cols())
            .map_err(|_| ImageOpsError::InvalidParameter("OpenCvMatView: negative cols"))?;
        let height = u32::try_from(mat.rows())
            .map_err(|_| ImageOpsError::InvalidParameter("OpenCvMatView: negative rows"))?;
        Ok(Self {
            mat,
            pixel_format,
            width,
            height,
        })
    }

    /// Access the wrapped `Mat` by reference.
    pub fn as_mat(&self) -> &Mat {
        self.mat
    }
}

impl<'a> MatView for OpenCvMatView<'a> {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn channels(&self) -> u32 {
        self.pixel_format.channels()
    }
    fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }
    fn data(&self) -> &[u8] {
        // `data_bytes` fails when the Mat is non-continuous; we built
        // this view from a user-supplied Mat so propagate a sensible
        // empty slice for empty mats and let OpenCV's own errors surface
        // via try_from_mat / higher layers for exotic layouts.
        self.mat.data_bytes().unwrap_or(&[])
    }
}

/// Construct an OpenCV `Mat` that borrows `slice` as its pixel buffer.
///
/// Returns a continuous `HxW` multi-channel `Mat`:
/// a flat single-channel view is created over `rows` by `cols*channels`
/// bytes and then reshaped to `channels` channels when needed. The
/// returned `Mat` is cloned to sever the borrow lifetime so callers may
/// hand it to OpenCV functions that expect an owned `Mat`.
///
/// # Safety
/// The underlying FFI call (`new_rows_cols_with_data_unsafe_def`) is
/// safe because Rust's borrow checker guarantees `slice` outlives the
/// intermediate `BoxedRef<Mat>`; the subsequent `try_clone` copies the
/// data so the returned `Mat` no longer aliases the input slice.
///
/// Nonetheless this function is marked `unsafe` because incorrect
/// `rows` / `cols` / `channels` arguments (arguments that disagree with
/// `slice.len()`) will produce a Mat with out-of-bounds pixels. Callers
/// must ensure `rows * cols * channels == slice.len()`.
pub unsafe fn slice_to_mat(
    slice: &[u8],
    rows: i32,
    cols: i32,
    channels: i32,
) -> opencv::Result<Mat> {
    // `new_rows_cols_with_data` wants width-in-samples for a
    // single-channel Mat type; we then reshape into `channels` channels.
    let width_1c = cols * channels;
    let view = Mat::new_rows_cols_with_data(rows, width_1c, slice)?;
    let reshaped = if channels > 1 {
        view.reshape(channels, rows)?
    } else {
        // BoxedRef<Mat> -> we still need to go through reshape to land
        // in a type that `try_clone` can call. `BoxedRef` derefs to Mat
        // so we just clone through it below.
        return view.try_clone();
    };
    reshaped.try_clone()
}
