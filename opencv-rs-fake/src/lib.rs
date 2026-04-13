//! Deterministic in-memory implementations of every `opencv-rs-core`
//! port for use in unit tests.
//!
//! Tests use these fakes to script programmable frame sequences, inject
//! errors at specific call sites, and record invocations for later
//! assertion — all without linking against the `opencv` crate.
//!
//! This crate depends only on `opencv-rs-core`. It must NEVER depend on
//! `opencv-rs-ffi` or the `opencv` crate.
//!
//! The actual fake types (`ScriptedVideoCapture`, `ScriptedImageEncoder`,
//! etc.) are filled in by issue #1 on this repo.

// Populated by issue #1.
