//! Contract-level tests that verify the fakes in this crate satisfy
//! every published `opencv_rs_core::contract_tests::verify_*` helper.

use std::sync::Arc;

use opencv_rs_core::contract_tests as ct;
use opencv_rs_core::{CapturedFrame, PixelFormat, VideoCapturePort, VideoStream};
use opencv_rs_fake::{PassthroughImageOps, ScriptedImageEncoder, ScriptedVideoCapture};

fn sample_frames() -> Vec<CapturedFrame> {
    let data_a: Vec<u8> = (0..16).collect();
    let data_b: Vec<u8> = (16..32).collect();
    vec![
        CapturedFrame {
            width: 4,
            height: 4,
            pixel_format: PixelFormat::Mono8,
            data: Arc::from(data_a.into_boxed_slice()),
        },
        CapturedFrame {
            width: 4,
            height: 4,
            pixel_format: PixelFormat::Mono8,
            data: Arc::from(data_b.into_boxed_slice()),
        },
    ]
}

fn make_capture() -> ScriptedVideoCapture {
    ScriptedVideoCapture::with_frames(sample_frames())
}

fn open_stream(cap: &ScriptedVideoCapture) -> Box<dyn VideoStream> {
    cap.open(
        std::path::Path::new("scripted"),
        opencv_rs_core::Backend::Auto,
    )
    .expect("scripted open must succeed")
}

// --- VideoCapturePort / VideoStream ------------------------------------

#[test]
fn video_capture_rejects_missing_file_when_armed() {
    // Scripted capture does not naturally reject any path, so arm it
    // with an open_error so the verifier passes.
    let cap = ScriptedVideoCapture::new();
    cap.fail_open_with(opencv_rs_core::VideoCaptureError::OpenFailed {
        path: "missing".to_string(),
    });
    ct::verify_video_capture_open_rejects_missing_file(&cap);
}

#[test]
fn video_stream_read_and_seek() {
    let cap = make_capture();
    ct::verify_video_stream_read_and_seek(|| open_stream(&cap));
}

#[test]
fn video_stream_fps_positive() {
    let cap = make_capture();
    ct::verify_video_stream_fps(|| open_stream(&cap));
}

#[test]
fn video_stream_end_of_stream() {
    let cap = make_capture();
    ct::verify_video_stream_end_of_stream(|| open_stream(&cap));
}

// --- ImageEncoderPort --------------------------------------------------

#[test]
fn image_encoder_none_passthrough() {
    let enc = ScriptedImageEncoder::new();
    ct::verify_image_encoder_none_passthrough(&enc);
}

#[test]
fn image_encoder_jpeg_nonempty() {
    let enc = ScriptedImageEncoder::new();
    ct::verify_image_encoder_jpeg_nonempty(&enc);
}

#[test]
fn image_encoder_webp_nonempty() {
    let enc = ScriptedImageEncoder::new();
    ct::verify_image_encoder_webp_nonempty(&enc);
}

// --- ImageOpsPort ------------------------------------------------------

#[test]
fn image_ops_cvt_color_bgr_to_rgb() {
    ct::verify_image_ops_cvt_color_bgr_to_rgb(&PassthroughImageOps);
}

#[test]
fn image_ops_cvt_color_gray_to_rgb() {
    ct::verify_image_ops_cvt_color_gray_to_rgb(&PassthroughImageOps);
}

#[test]
fn image_ops_cvt_color_bgr_to_gray() {
    ct::verify_image_ops_cvt_color_bgr_to_gray(&PassthroughImageOps);
}

#[test]
fn image_ops_gaussian_blur_smoke() {
    ct::verify_image_ops_gaussian_blur_smoke(&PassthroughImageOps);
}

#[test]
fn image_ops_threshold_binary() {
    ct::verify_image_ops_threshold_binary(&PassthroughImageOps);
}

#[test]
fn image_ops_absdiff() {
    ct::verify_image_ops_absdiff(&PassthroughImageOps);
}

#[test]
fn image_ops_convert_scale_abs() {
    ct::verify_image_ops_convert_scale_abs(&PassthroughImageOps);
}

#[test]
fn image_ops_min_max_loc() {
    ct::verify_image_ops_min_max_loc(&PassthroughImageOps);
}

#[test]
fn image_ops_count_non_zero() {
    ct::verify_image_ops_count_non_zero(&PassthroughImageOps);
}

#[test]
fn image_ops_resize_smoke() {
    ct::verify_image_ops_resize_smoke(&PassthroughImageOps);
}

// --- MatView helpers on CapturedFrame ----------------------------------

#[test]
fn mat_view_dimensions_on_captured_frame() {
    let frame = sample_frames().remove(0);
    ct::verify_mat_view_dimensions(&frame, (4, 4, 1));
    ct::verify_mat_view_data_length(&frame);
    ct::verify_mat_view_pixel_format(&frame, PixelFormat::Mono8);
}
