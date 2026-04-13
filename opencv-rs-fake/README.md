# opencv-rs-fake

In-memory implementations of every `opencv-rs-core` port, for unit
testing code that depends on the ports.

Tests use these fakes to script programmable frame sequences, inject
errors at specific call sites, and record invocations for later
assertion — **without linking against the `opencv` crate**.

This crate depends only on `opencv-rs-core`. It must NEVER depend on
`opencv-rs-ffi` or the `opencv` crate.
