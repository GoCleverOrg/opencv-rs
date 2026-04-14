//! Equivalence tests between the pure-Rust and OpenCV-backed image-ops
//! implementations.
//!
//! Each submodule targets a single `ImageOpsPort` primitive that has a
//! pure-Rust reimplementation in `opencv_rs_core::PureRustImageOps`:
//! we feed a small deterministic input into both implementations and
//! assert byte-level equivalence (with a documented ±1 tolerance for
//! operations whose rounding conventions differ between OpenCV and
//! pure Rust).

#![cfg(feature = "opencv")]

#[path = "equivalence/cvt_color.rs"]
mod cvt_color;

#[path = "equivalence/threshold.rs"]
mod threshold;

#[path = "equivalence/absdiff.rs"]
mod absdiff;

#[path = "equivalence/count_non_zero.rs"]
mod count_non_zero;

#[path = "equivalence/convert_scale_abs.rs"]
mod convert_scale_abs;

#[path = "equivalence/min_max_loc.rs"]
mod min_max_loc;
