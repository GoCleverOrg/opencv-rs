//! Scripted in-memory implementation of [`VideoCapturePort`] and
//! [`VideoStream`] for unit testing.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use opencv_rs_core::{Backend, CapturedFrame, VideoCaptureError, VideoCapturePort, VideoStream};

/// Internal mutable state for [`ScriptedVideoCapture`].
#[derive(Debug)]
struct CaptureState {
    /// Frames the next opened stream will yield in order.
    frames: Vec<CapturedFrame>,
    /// If `Some`, the next call to [`VideoCapturePort::open`] returns
    /// this error (consumed).
    open_error: Option<VideoCaptureError>,
    /// Reported FPS for every opened stream.
    fps: f64,
    /// Every `open` call, in order of invocation.
    open_calls: Vec<(PathBuf, Backend)>,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            open_error: None,
            fps: 30.0,
            open_calls: Vec::new(),
        }
    }
}

/// Scripted [`VideoCapturePort`] that serves a preconfigured list of
/// frames from an in-memory script.
///
/// Clones share the same underlying state via `Arc<Mutex<_>>`, so
/// mutators applied to one handle are visible from every clone.
#[derive(Debug, Clone, Default)]
pub struct ScriptedVideoCapture {
    inner: Arc<Mutex<CaptureState>>,
}

impl ScriptedVideoCapture {
    /// Construct an empty scripted capture with FPS defaulted to 30.0
    /// and no frames queued.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a scripted capture preloaded with the given frames.
    pub fn with_frames(frames: Vec<CapturedFrame>) -> Self {
        let this = Self::new();
        this.set_frames(frames);
        this
    }

    /// Replace the frame script used by future [`VideoCapturePort::open`]
    /// calls.
    pub fn set_frames(&self, frames: Vec<CapturedFrame>) {
        self.inner.lock().expect("poisoned").frames = frames;
    }

    /// Set the FPS reported by future streams.
    pub fn set_fps(&self, fps: f64) {
        self.inner.lock().expect("poisoned").fps = fps;
    }

    /// Arm the next call to [`VideoCapturePort::open`] to fail with the
    /// given error. The arm is consumed on the next `open`.
    pub fn fail_open_with(&self, err: VideoCaptureError) {
        self.inner.lock().expect("poisoned").open_error = Some(err);
    }

    /// Snapshot of every `open` call recorded so far.
    pub fn open_calls(&self) -> Vec<(PathBuf, Backend)> {
        self.inner.lock().expect("poisoned").open_calls.clone()
    }
}

impl VideoCapturePort for ScriptedVideoCapture {
    fn open(
        &self,
        path: &Path,
        backend: Backend,
    ) -> Result<Box<dyn VideoStream>, VideoCaptureError> {
        let mut state = self.inner.lock().expect("poisoned");
        state.open_calls.push((path.to_path_buf(), backend));
        if let Some(err) = state.open_error.take() {
            return Err(err);
        }
        let stream = ScriptedVideoStream {
            frames: state.frames.clone(),
            cursor: 0,
            fps: state.fps,
        };
        Ok(Box::new(stream))
    }
}

/// Scripted [`VideoStream`] created by [`ScriptedVideoCapture::open`].
#[derive(Debug)]
pub struct ScriptedVideoStream {
    /// Frames that will be returned in order.
    frames: Vec<CapturedFrame>,
    /// Index of the next frame to return.
    cursor: usize,
    /// FPS reported by [`VideoStream::fps`].
    fps: f64,
}

impl ScriptedVideoStream {
    /// Build a stream directly, mainly useful in integration tests that
    /// want to bypass the capture port.
    pub fn new(frames: Vec<CapturedFrame>, fps: f64) -> Self {
        Self {
            frames,
            cursor: 0,
            fps,
        }
    }
}

impl VideoStream for ScriptedVideoStream {
    fn read_frame(&mut self) -> Result<CapturedFrame, VideoCaptureError> {
        if self.cursor < self.frames.len() {
            let frame = self.frames[self.cursor].clone();
            self.cursor += 1;
            Ok(frame)
        } else {
            Err(VideoCaptureError::EndOfStream)
        }
    }

    fn fps(&self) -> Result<f64, VideoCaptureError> {
        Ok(self.fps)
    }

    fn seek_to_start(&mut self) -> Result<(), VideoCaptureError> {
        self.cursor = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv_rs_core::PixelFormat;
    use std::sync::Arc as StdArc;

    fn sample_frame() -> CapturedFrame {
        CapturedFrame {
            width: 2,
            height: 2,
            pixel_format: PixelFormat::Mono8,
            data: StdArc::from(vec![1u8, 2, 3, 4].into_boxed_slice()),
        }
    }

    #[test]
    fn fail_open_with_returns_scripted_error() {
        let cap = ScriptedVideoCapture::new();
        cap.fail_open_with(VideoCaptureError::OpenFailed {
            path: "scripted".to_string(),
        });
        let res = cap.open(Path::new("whatever"), Backend::Auto);
        match res {
            Err(VideoCaptureError::OpenFailed { .. }) => {}
            Err(other) => panic!("unexpected error: {other:?}"),
            Ok(_) => panic!("expected OpenFailed, got Ok"),
        }
        // Arm is consumed: next open should now succeed.
        assert!(cap.open(Path::new("whatever"), Backend::Auto).is_ok());
    }

    #[test]
    fn open_records_calls() {
        let cap = ScriptedVideoCapture::new();
        let _ = cap.open(Path::new("/a.mp4"), Backend::Auto);
        let _ = cap.open(Path::new("/b.mp4"), Backend::Ffmpeg);
        let calls = cap.open_calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], (PathBuf::from("/a.mp4"), Backend::Auto));
        assert_eq!(calls[1], (PathBuf::from("/b.mp4"), Backend::Ffmpeg));
    }

    #[test]
    fn stream_reads_frames_then_end_of_stream() {
        let cap = ScriptedVideoCapture::with_frames(vec![sample_frame(), sample_frame()]);
        let mut s = cap.open(Path::new("x"), Backend::Auto).unwrap();
        assert!(s.read_frame().is_ok());
        assert!(s.read_frame().is_ok());
        assert!(matches!(
            s.read_frame().unwrap_err(),
            VideoCaptureError::EndOfStream
        ));
    }

    #[test]
    fn stream_seek_to_start_rewinds() {
        let cap = ScriptedVideoCapture::with_frames(vec![sample_frame()]);
        let mut s = cap.open(Path::new("x"), Backend::Auto).unwrap();
        assert!(s.read_frame().is_ok());
        assert!(matches!(
            s.read_frame().unwrap_err(),
            VideoCaptureError::EndOfStream
        ));
        s.seek_to_start().unwrap();
        assert!(s.read_frame().is_ok());
    }

    #[test]
    fn fps_returns_configured_value() {
        let cap = ScriptedVideoCapture::with_frames(vec![sample_frame()]);
        cap.set_fps(60.0);
        let s = cap.open(Path::new("x"), Backend::Auto).unwrap();
        assert_eq!(s.fps().unwrap(), 60.0);
    }
}
