//! Hexagonal Rust wrapper over OpenCV primitives.
//!
//! Convenience facade re-exporting the ports defined in
//! `opencv-rs-core`. When the `opencv` feature is enabled, the production
//! adapters from `opencv-rs-ffi` are also re-exported.
//!
//! ```text
//! opencv-rs-core   ← traits + DTOs, zero FFI
//!     ↑
//!     ├── opencv-rs-fake  ← deterministic in-memory impls for tests
//!     └── opencv-rs-ffi   ← production impls wrapping the `opencv` crate
//!             ↑
//!             opencv-rs  ← this facade
//! ```
//!
//! End users should depend on this crate. Downstream tests depend on
//! `opencv-rs-fake` as a dev-dependency.

pub use opencv_rs_core as core;

#[cfg(feature = "opencv")]
pub use opencv_rs_ffi as ffi;
