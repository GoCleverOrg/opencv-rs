# opencv-rs-ffi

OpenCV-backed production implementations of every `opencv-rs-core` port.

This is the **only** crate in the workspace that depends on the `opencv`
crate. Everything else sees OpenCV exclusively through the
`opencv-rs-core` trait surface.

Gated by the `opencv` Cargo feature so the workspace can compile without
the OpenCV native libraries present on the host — mirroring the
`vmb-rs` `sdk` pattern.

## Equivalence-tests policy

If any implementation in `opencv-rs-core` or `opencv-rs-fake` contains a
pure-Rust reimplementation rather than delegating through the trait
surface, `opencv-rs-ffi/tests/equivalence/` MUST contain a default-run
integration test feeding identical input to both implementations and
asserting byte-equivalence (or documented equivalence class) of output.
This prevents silent semantic drift between pure-Rust reimplementations
and the OpenCV baseline.
