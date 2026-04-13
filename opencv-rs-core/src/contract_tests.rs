//! Public contract-test helpers that any [`crate`] port implementation
//! can use to prove it satisfies the port's behavioral contract.
//!
//! Every helper panics via `assert!` / `assert_eq!` on contract violation.

use std::path::Path;
use std::sync::Arc;

use crate::{
    Backend, CapturedFrame, ColorConversion, EncodingKind, ImageEncoderPort, ImageOpsPort, MatView,
    OwnedMatView, PixelFormat, ThresholdKind, VideoCaptureError, VideoCapturePort, VideoStream,
};

// --- MatView ------------------------------------------------------------

/// Verify that `view.width() / height() / channels()` equals `expected`.
pub fn verify_mat_view_dimensions<M: MatView>(view: &M, expected: (u32, u32, u32)) {
    assert_eq!(view.width(), expected.0, "width mismatch");
    assert_eq!(view.height(), expected.1, "height mismatch");
    assert_eq!(view.channels(), expected.2, "channels mismatch");
}

/// Verify that `view.data().len() == width * height * channels`.
pub fn verify_mat_view_data_length<M: MatView>(view: &M) {
    let expected = (view.width() as usize) * (view.height() as usize) * (view.channels() as usize);
    assert_eq!(
        view.data().len(),
        expected,
        "data length does not match width*height*channels"
    );
}

/// Verify that `view.pixel_format()` matches `expected`.
pub fn verify_mat_view_pixel_format<M: MatView>(view: &M, expected: PixelFormat) {
    assert_eq!(view.pixel_format(), expected, "pixel format mismatch");
}

// --- VideoCapturePort ---------------------------------------------------

/// Verify that opening a nonexistent path returns a `VideoCaptureError`.
pub fn verify_video_capture_open_rejects_missing_file<P: VideoCapturePort>(port: &P) {
    let path = Path::new("/definitely/not/a/real/path.does-not-exist.mp4");
    let result = port.open(path, Backend::Auto);
    assert!(
        result.is_err(),
        "expected error opening missing file, got Ok"
    );
}

// --- VideoStream --------------------------------------------------------

/// Verify that `read_frame` followed by `seek_to_start` + `read_frame` both succeed.
pub fn verify_video_stream_read_and_seek<F>(mut make_stream: F)
where
    F: FnMut() -> Box<dyn VideoStream>,
{
    let mut s = make_stream();
    let first = s.read_frame().expect("first read_frame should succeed");
    assert!(first.width > 0 && first.height > 0, "empty frame");
    s.seek_to_start().expect("seek_to_start should succeed");
    let _again = s
        .read_frame()
        .expect("read_frame after seek_to_start should succeed");
}

/// Verify that `fps` returns a positive value.
pub fn verify_video_stream_fps<F>(mut make_stream: F)
where
    F: FnMut() -> Box<dyn VideoStream>,
{
    let s = make_stream();
    let fps = s.fps().expect("fps should succeed");
    assert!(fps > 0.0, "fps must be positive, got {fps}");
}

/// Verify that reading past the end of the stream yields [`VideoCaptureError::EndOfStream`].
pub fn verify_video_stream_end_of_stream<F>(mut make_stream: F)
where
    F: FnMut() -> Box<dyn VideoStream>,
{
    let mut s = make_stream();
    loop {
        match s.read_frame() {
            Ok(_) => continue,
            Err(VideoCaptureError::EndOfStream) => return,
            Err(e) => panic!("expected EndOfStream, got {e:?}"),
        }
    }
}

// --- ImageEncoderPort ---------------------------------------------------

fn sample_bgr_frame() -> CapturedFrame {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    CapturedFrame {
        width: 2,
        height: 2,
        pixel_format: PixelFormat::Bgr8,
        data: Arc::from(data.into_boxed_slice()),
    }
}

/// Verify that `EncodingKind::None` returns the raw pixel bytes unchanged.
pub fn verify_image_encoder_none_passthrough<E: ImageEncoderPort>(encoder: &E) {
    let frame = sample_bgr_frame();
    let out = encoder
        .encode(&frame, EncodingKind::None)
        .expect("None encoding should succeed");
    assert_eq!(out.as_slice(), frame.data(), "None must be passthrough");
}

/// Verify that `EncodingKind::Jpeg` produces a non-empty byte buffer.
pub fn verify_image_encoder_jpeg_nonempty<E: ImageEncoderPort>(encoder: &E) {
    let frame = sample_bgr_frame();
    let out = encoder
        .encode(&frame, EncodingKind::Jpeg)
        .expect("JPEG encoding should succeed");
    assert!(!out.is_empty(), "JPEG output must not be empty");
}

/// Verify that `EncodingKind::Webp` produces a non-empty byte buffer.
pub fn verify_image_encoder_webp_nonempty<E: ImageEncoderPort>(encoder: &E) {
    let frame = sample_bgr_frame();
    let out = encoder
        .encode(&frame, EncodingKind::Webp)
        .expect("WebP encoding should succeed");
    assert!(!out.is_empty(), "WebP output must not be empty");
}

// --- ImageOpsPort -------------------------------------------------------

/// Verify that `cvt_color(BgrToRgb)` swaps channels 0 and 2.
pub fn verify_image_ops_cvt_color_bgr_to_rgb<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(1, 1, PixelFormat::Bgr8, vec![10, 20, 30]).unwrap();
    let out = ops.cvt_color(&src, ColorConversion::BgrToRgb).unwrap();
    assert_eq!(out.pixel_format(), PixelFormat::Rgb8);
    assert_eq!(out.data(), &[30, 20, 10]);
}

/// Verify that `cvt_color(GrayToRgb)` broadcasts the single channel to three.
pub fn verify_image_ops_cvt_color_gray_to_rgb<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(1, 1, PixelFormat::Mono8, vec![42]).unwrap();
    let out = ops.cvt_color(&src, ColorConversion::GrayToRgb).unwrap();
    assert_eq!(out.pixel_format(), PixelFormat::Rgb8);
    assert_eq!(out.data(), &[42, 42, 42]);
}

/// Verify that `cvt_color(BgrToGray)` produces a Mono8 image the same size as the input.
pub fn verify_image_ops_cvt_color_bgr_to_gray<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(1, 1, PixelFormat::Bgr8, vec![0, 0, 255]).unwrap();
    let out = ops.cvt_color(&src, ColorConversion::BgrToGray).unwrap();
    assert_eq!(out.pixel_format(), PixelFormat::Mono8);
    assert_eq!(out.width(), 1);
    assert_eq!(out.height(), 1);
    // Weight for red in BGR is 0.299: 0.299*255 ~= 76.2 -> 76. Allow 1 of rounding slack.
    let v = out.data()[0];
    assert!(v >= 75 && v <= 77, "unexpected gray value {v}");
}

/// Verify that `gaussian_blur` returns an output with the same dimensions as the input.
pub fn verify_image_ops_gaussian_blur_smoke<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::zeros(4, 4, PixelFormat::Mono8);
    let out = ops
        .gaussian_blur(&src, (3, 3), 1.0, 1.0)
        .expect("gaussian_blur should succeed for this smoke input");
    assert_eq!(out.width(), 4);
    assert_eq!(out.height(), 4);
}

/// Verify binary thresholding on a 3-pixel Mono8 buffer.
pub fn verify_image_ops_threshold_binary<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(3, 1, PixelFormat::Mono8, vec![10, 50, 200]).unwrap();
    let out = ops
        .threshold(&src, 30.0, 255.0, ThresholdKind::Binary)
        .unwrap();
    assert_eq!(out.data(), &[0, 255, 255]);
}

/// Verify element-wise absolute difference of two equally shaped Mono8 images.
pub fn verify_image_ops_absdiff<O: ImageOpsPort>(ops: &O) {
    let a = OwnedMatView::new(2, 2, PixelFormat::Mono8, vec![10, 20, 30, 40]).unwrap();
    let b = OwnedMatView::new(2, 2, PixelFormat::Mono8, vec![5, 25, 30, 100]).unwrap();
    let out = ops.absdiff(&a, &b).unwrap();
    assert_eq!(out.data(), &[5, 5, 0, 60]);
}

/// Verify `convert_scale_abs` with `scale=0.5, offset=0` on a Mono8 buffer.
pub fn verify_image_ops_convert_scale_abs<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(3, 1, PixelFormat::Mono8, vec![10, 20, 240]).unwrap();
    let out = ops.convert_scale_abs(&src, 0.5, 0.0).unwrap();
    assert_eq!(out.data(), &[5, 10, 120]);
}

/// Verify `min_max_loc` returns correct min/max and first-occurrence coordinates.
pub fn verify_image_ops_min_max_loc<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(3, 2, PixelFormat::Mono8, vec![5, 7, 1, 3, 7, 1]).unwrap();
    let r = ops.min_max_loc(&src).unwrap();
    assert_eq!(r.min, 1.0);
    assert_eq!(r.max, 7.0);
    assert_eq!(r.min_loc, (2, 0));
    assert_eq!(r.max_loc, (1, 0));
}

/// Verify `count_non_zero` counts non-zero bytes in a Mono8 image.
pub fn verify_image_ops_count_non_zero<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::new(2, 2, PixelFormat::Mono8, vec![0, 1, 2, 0]).unwrap();
    assert_eq!(ops.count_non_zero(&src).unwrap(), 2);
}

/// Verify that `resize` produces an output with exactly the requested dimensions.
pub fn verify_image_ops_resize_smoke<O: ImageOpsPort>(ops: &O) {
    let src = OwnedMatView::zeros(4, 4, PixelFormat::Mono8);
    let out = ops
        .resize(&src, 2, 2)
        .expect("resize should succeed for this smoke input");
    assert_eq!(out.width(), 2);
    assert_eq!(out.height(), 2);
}
