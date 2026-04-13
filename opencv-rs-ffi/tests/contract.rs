//! Contract tests binding the `opencv-rs-core::contract_tests` helpers
//! to the OpenCV-backed production adapters.
//!
//! Video-stream helpers requiring a real open stream synthesize a tiny
//! test video at test-setup time via `opencv::videoio::VideoWriter`.

#![cfg(feature = "opencv")]

use opencv_rs_core::contract_tests as ct;
use opencv_rs_core::{Backend, CapturedFrame, PixelFormat, VideoCapturePort, VideoStream};
use opencv_rs_ffi::{
    slice_to_mat, OpenCvImageEncoder, OpenCvImageOps, OpenCvMatView, OpenCvVideoCapture,
};
use std::path::PathBuf;
use std::sync::Arc;

/// Synthesize a tiny test video (5 frames, 16x16 BGR, 10 fps) into a
/// unique temp subdirectory. Tries `.mp4` with fourcc `mp4v` first,
/// then falls back to `.avi` with fourcc `MJPG`. Panics if neither
/// works on this host.
fn build_test_video() -> PathBuf {
    use opencv::core::{Mat, Scalar, Size, CV_8UC3};
    use opencv::videoio::{VideoWriter, VideoWriterTrait, VideoWriterTraitConst};

    let dir = std::env::temp_dir().join(format!(
        "opencv_rs_test_{}_{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");

    let attempts: &[(&str, [char; 4])] = &[
        ("sample.mp4", ['m', 'p', '4', 'v']),
        ("sample.avi", ['M', 'J', 'P', 'G']),
    ];

    for (filename, cc) in attempts {
        let path = dir.join(filename);
        let fourcc = match VideoWriter::fourcc(cc[0], cc[1], cc[2], cc[3]) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mut writer = match VideoWriter::new(
            path.to_str().unwrap(),
            fourcc,
            10.0,
            Size::new(16, 16),
            true,
        ) {
            Ok(w) => w,
            Err(_) => continue,
        };
        let opened = writer.is_opened().unwrap_or(false);
        if !opened {
            let _ = writer.release();
            let _ = std::fs::remove_file(&path);
            continue;
        }
        let mut all_ok = true;
        for i in 0..5 {
            let mat = Mat::new_rows_cols_with_default(
                16,
                16,
                CV_8UC3,
                Scalar::new((i * 40) as f64, 0.0, 0.0, 0.0),
            )
            .unwrap();
            if writer.write(&mat).is_err() {
                all_ok = false;
                break;
            }
        }
        let _ = writer.release();
        if !all_ok {
            let _ = std::fs::remove_file(&path);
            continue;
        }
        // Sanity: verify the file can be re-opened and yields a frame.
        let port = OpenCvVideoCapture::new();
        match port.open(&path, Backend::Auto) {
            Ok(mut stream) => {
                if stream.read_frame().is_ok() {
                    return path;
                }
            }
            Err(_) => {}
        }
        let _ = std::fs::remove_file(&path);
    }

    panic!(
        "could not synthesize a test video; neither mp4v/.mp4 nor MJPG/.avi \
         encoders are available on this host"
    );
}

fn make_stream_factory(path: PathBuf) -> impl FnMut() -> Box<dyn VideoStream> {
    move || {
        let port = OpenCvVideoCapture::new();
        port.open(&path, Backend::Auto).expect("open test video")
    }
}

fn sample_bgr() -> CapturedFrame {
    CapturedFrame {
        width: 2,
        height: 2,
        pixel_format: PixelFormat::Bgr8,
        data: Arc::from(vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].into_boxed_slice()),
    }
}

#[test]
fn mat_view_dimensions_contract() {
    let frame = sample_bgr();
    ct::verify_mat_view_dimensions(&frame, (2, 2, 3));
}

#[test]
fn mat_view_data_length_contract() {
    let frame = sample_bgr();
    ct::verify_mat_view_data_length(&frame);
}

#[test]
fn mat_view_pixel_format_contract() {
    let frame = sample_bgr();
    ct::verify_mat_view_pixel_format(&frame, PixelFormat::Bgr8);
}

#[test]
fn opencv_mat_view_bgr_dimensions_contract() {
    let data: Vec<u8> = (0..48u8).collect();
    let mat = unsafe { slice_to_mat(&data, 4, 4, 3) }.expect("build bgr mat");
    let view = OpenCvMatView::try_from_mat(&mat).expect("wrap bgr mat");
    ct::verify_mat_view_dimensions(&view, (4, 4, 3));
}

#[test]
fn opencv_mat_view_bgr_data_length_contract() {
    let data: Vec<u8> = (0..48u8).collect();
    let mat = unsafe { slice_to_mat(&data, 4, 4, 3) }.expect("build bgr mat");
    let view = OpenCvMatView::try_from_mat(&mat).expect("wrap bgr mat");
    ct::verify_mat_view_data_length(&view);
}

#[test]
fn opencv_mat_view_bgr_pixel_format_contract() {
    let data: Vec<u8> = (0..48u8).collect();
    let mat = unsafe { slice_to_mat(&data, 4, 4, 3) }.expect("build bgr mat");
    let view = OpenCvMatView::try_from_mat(&mat).expect("wrap bgr mat");
    ct::verify_mat_view_pixel_format(&view, PixelFormat::Bgr8);
}

#[test]
fn opencv_mat_view_mono_dimensions_contract() {
    let data: Vec<u8> = (0..16u8).collect();
    let mat = unsafe { slice_to_mat(&data, 4, 4, 1) }.expect("build mono mat");
    let view = OpenCvMatView::try_from_mat(&mat).expect("wrap mono mat");
    ct::verify_mat_view_dimensions(&view, (4, 4, 1));
}

#[test]
fn opencv_mat_view_mono_data_length_contract() {
    let data: Vec<u8> = (0..16u8).collect();
    let mat = unsafe { slice_to_mat(&data, 4, 4, 1) }.expect("build mono mat");
    let view = OpenCvMatView::try_from_mat(&mat).expect("wrap mono mat");
    ct::verify_mat_view_data_length(&view);
}

#[test]
fn opencv_mat_view_mono_pixel_format_contract() {
    let data: Vec<u8> = (0..16u8).collect();
    let mat = unsafe { slice_to_mat(&data, 4, 4, 1) }.expect("build mono mat");
    let view = OpenCvMatView::try_from_mat(&mat).expect("wrap mono mat");
    ct::verify_mat_view_pixel_format(&view, PixelFormat::Mono8);
}

#[test]
fn video_capture_open_rejects_missing_file_contract() {
    let port = OpenCvVideoCapture::new();
    ct::verify_video_capture_open_rejects_missing_file(&port);
}

#[test]
fn image_encoder_none_passthrough_contract() {
    let enc = OpenCvImageEncoder::new();
    ct::verify_image_encoder_none_passthrough(&enc);
}

#[test]
fn image_encoder_jpeg_nonempty_contract() {
    let enc = OpenCvImageEncoder::new();
    ct::verify_image_encoder_jpeg_nonempty(&enc);
}

#[test]
fn image_encoder_webp_nonempty_contract() {
    let enc = OpenCvImageEncoder::new();
    ct::verify_image_encoder_webp_nonempty(&enc);
}

#[test]
fn image_ops_cvt_color_bgr_to_rgb_contract() {
    ct::verify_image_ops_cvt_color_bgr_to_rgb(&OpenCvImageOps::new());
}

#[test]
fn image_ops_cvt_color_gray_to_rgb_contract() {
    ct::verify_image_ops_cvt_color_gray_to_rgb(&OpenCvImageOps::new());
}

#[test]
fn image_ops_cvt_color_bgr_to_gray_contract() {
    ct::verify_image_ops_cvt_color_bgr_to_gray(&OpenCvImageOps::new());
}

#[test]
fn image_ops_gaussian_blur_smoke_contract() {
    ct::verify_image_ops_gaussian_blur_smoke(&OpenCvImageOps::new());
}

#[test]
fn image_ops_threshold_binary_contract() {
    ct::verify_image_ops_threshold_binary(&OpenCvImageOps::new());
}

#[test]
fn image_ops_absdiff_contract() {
    ct::verify_image_ops_absdiff(&OpenCvImageOps::new());
}

#[test]
fn image_ops_convert_scale_abs_contract() {
    ct::verify_image_ops_convert_scale_abs(&OpenCvImageOps::new());
}

#[test]
fn image_ops_min_max_loc_contract() {
    ct::verify_image_ops_min_max_loc(&OpenCvImageOps::new());
}

#[test]
fn image_ops_count_non_zero_contract() {
    ct::verify_image_ops_count_non_zero(&OpenCvImageOps::new());
}

#[test]
fn image_ops_resize_smoke_contract() {
    ct::verify_image_ops_resize_smoke(&OpenCvImageOps::new());
}

#[test]
fn video_stream_read_and_seek_contract() {
    let path = build_test_video();
    ct::verify_video_stream_read_and_seek(make_stream_factory(path));
}

#[test]
fn video_stream_fps_contract() {
    let path = build_test_video();
    ct::verify_video_stream_fps(make_stream_factory(path));
}

#[test]
fn video_stream_end_of_stream_contract() {
    let path = build_test_video();
    ct::verify_video_stream_end_of_stream(make_stream_factory(path));
}
