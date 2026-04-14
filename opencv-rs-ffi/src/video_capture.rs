//! OpenCV-backed implementation of [`VideoCapturePort`] / [`VideoStream`].

use std::path::Path;
use std::sync::Arc;

use opencv::prelude::*;
use opencv::videoio;
use opencv_rs_core::{
    Backend, CapturedFrame, PixelFormat, VideoCaptureError, VideoCapturePort, VideoStream,
};

/// Production [`VideoCapturePort`] delegating to
/// `opencv::videoio::VideoCapture`.
#[derive(Debug, Default, Clone, Copy)]
pub struct OpenCvVideoCapture;

impl OpenCvVideoCapture {
    /// Construct a new [`OpenCvVideoCapture`].
    pub fn new() -> Self {
        Self
    }
}

impl VideoCapturePort for OpenCvVideoCapture {
    fn open(
        &self,
        path: &Path,
        backend: Backend,
    ) -> Result<Box<dyn VideoStream>, VideoCaptureError> {
        let path_str = path.to_str().ok_or_else(|| VideoCaptureError::OpenFailed {
            path: path.display().to_string(),
        })?;
        let backend_code = match backend {
            Backend::Auto => videoio::CAP_ANY,
            Backend::Ffmpeg => videoio::CAP_FFMPEG,
        };
        let cap = videoio::VideoCapture::from_file(path_str, backend_code)
            .map_err(|e| VideoCaptureError::Backend(e.to_string()))?;
        let is_open = cap
            .is_opened()
            .map_err(|e| VideoCaptureError::Backend(e.to_string()))?;
        if !is_open {
            return Err(VideoCaptureError::OpenFailed {
                path: path_str.to_string(),
            });
        }
        Ok(Box::new(OpenCvVideoStream { cap }))
    }
}

/// OpenCV-backed [`VideoStream`] wrapping a single
/// `opencv::videoio::VideoCapture`.
pub struct OpenCvVideoStream {
    cap: videoio::VideoCapture,
}

impl OpenCvVideoStream {
    /// Access the wrapped `VideoCapture` for advanced configuration
    /// (setting capture properties, etc.).
    pub fn as_mut_capture(&mut self) -> &mut videoio::VideoCapture {
        &mut self.cap
    }
}

impl VideoStream for OpenCvVideoStream {
    fn read_frame(&mut self) -> Result<CapturedFrame, VideoCaptureError> {
        let mut mat = opencv::core::Mat::default();
        let ok = self
            .cap
            .read(&mut mat)
            .map_err(|e| VideoCaptureError::Backend(e.to_string()))?;
        if !ok || mat.empty() {
            return Err(VideoCaptureError::EndOfStream);
        }
        let width = u32::try_from(mat.cols())
            .map_err(|_| VideoCaptureError::Backend("negative cols".to_string()))?;
        let height = u32::try_from(mat.rows())
            .map_err(|_| VideoCaptureError::Backend("negative rows".to_string()))?;
        let channels = mat.channels();
        let pixel_format = match channels {
            1 => PixelFormat::Mono8,
            3 => PixelFormat::Bgr8,
            other => {
                return Err(VideoCaptureError::Backend(format!(
                    "unsupported channel count: {other}"
                )));
            }
        };
        let bytes = mat
            .data_bytes()
            .map_err(|e| VideoCaptureError::Backend(e.to_string()))?;
        let data: Arc<[u8]> = Arc::from(bytes.to_vec().into_boxed_slice());
        Ok(CapturedFrame {
            width,
            height,
            pixel_format,
            data,
        })
    }

    fn fps(&self) -> Result<f64, VideoCaptureError> {
        self.cap
            .get(videoio::CAP_PROP_FPS)
            .map_err(|e| VideoCaptureError::Backend(e.to_string()))
    }

    fn seek_to_start(&mut self) -> Result<(), VideoCaptureError> {
        let ok = self
            .cap
            .set(videoio::CAP_PROP_POS_FRAMES, 0.0)
            .map_err(|e| VideoCaptureError::Backend(e.to_string()))?;
        if !ok {
            return Err(VideoCaptureError::SeekFailed);
        }
        Ok(())
    }
}
