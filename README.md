# opencv-rs

Hexagonal Rust wrapper over OpenCV I/O and image-processing primitives.
Follows the same ports-and-adapters layout as
[`vmb-rs`](https://github.com/GoCleverOrg/vmb-rs): domain and test
layers do not depend on the FFI, and a production adapter wraps the
real backend behind a feature flag.

## Architecture

```text
opencv-rs-core        ports + DTOs, zero FFI, zero unsafe
    ↑
    ├── opencv-rs-fake   in-memory scripted impls for unit tests
    └── opencv-rs-ffi    production impls wrapping the `opencv` crate
            ↑
        opencv-rs        user-facing facade (re-exports core; opencv-rs-ffi behind the `opencv` feature)
```

| Crate | Description |
| --- | --- |
| [`opencv-rs-core`](./opencv-rs-core) | Runtime-agnostic traits + DTOs. No dependency on the `opencv` crate. |
| [`opencv-rs-fake`](./opencv-rs-fake) | Deterministic in-memory impls for unit tests. |
| [`opencv-rs-ffi`](./opencv-rs-ffi) | Production impls wrapping the `opencv` crate. Gated by the `opencv` Cargo feature. |
| [`opencv-rs`](./opencv-rs) | User-facing facade. |

## Architectural contract

1. **`opencv-rs-core` has zero FFI**. It contains only traits, DTOs, and
   (optionally) pure-Rust logic. It MUST NOT depend on `opencv`.
2. **`opencv-rs-fake` has zero FFI**. It depends only on `opencv-rs-core`.
   Tests using opencv-rs-fake link zero OpenCV symbols — this is the
   architectural win that makes downstream mutation testing fast.
3. **`opencv-rs-ffi` is the only crate that depends on `opencv`**. The
   dependency is gated by the `opencv` Cargo feature so the workspace
   can compile without the OpenCV native libraries installed.
4. **Any pure-Rust reimplementation in `-core` or `-fake`** (as opposed
   to a plain trait definition) MUST be accompanied by a default-run
   integration test in `opencv-rs-ffi/tests/equivalence/*.rs` that
   asserts byte-equivalence (or documented equivalence class) of output
   between the pure-Rust impl and the OpenCV impl on identical input.
   This prevents silent semantic drift.
5. **Production consumers** (e.g. the mira workspace) depend on
   `opencv-rs` with the `opencv` feature enabled. Their tests depend on
   `opencv-rs-fake` as a dev-dependency.

## Development

```
make test           # cargo test --workspace
make lint           # fmt + clippy + deny + shear + taplo + typos
make mutants        # full workspace mutation bench
```

## License

Dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))
- MIT license ([LICENSE-MIT](./LICENSE-MIT))
