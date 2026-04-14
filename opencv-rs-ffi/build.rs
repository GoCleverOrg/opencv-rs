use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_OPENCV");

    if env::var("CARGO_FEATURE_OPENCV").is_err() {
        return;
    }

    let version = match probe("opencv4").or_else(|| probe("opencv")) {
        Some(v) => v,
        None => {
            println!(
                "cargo:warning=opencv-rs-ffi: pkg-config did not report an OpenCV version; skipping 4.10 floor check. The opencv crate's build script will surface any linkage errors."
            );
            return;
        }
    };

    let (major, minor) = match parse_version(&version) {
        Some(mm) => mm,
        None => {
            println!(
                "cargo:warning=opencv-rs-ffi: could not parse OpenCV version '{}'; skipping 4.10 floor check.",
                version
            );
            return;
        }
    };

    if (major, minor) < (4, 10) {
        println!(
            "cargo::error=opencv-rs-ffi requires OpenCV 4.10 or later (detected: {}). The cvt_color and gaussian_blur adapters use AlgorithmHint::ALGO_HINT_DEFAULT, introduced in 4.10. Please upgrade your OpenCV installation.",
            version
        );
    }
}

fn probe(pkg: &str) -> Option<String> {
    let output = Command::new("pkg-config")
        .args(["--modversion", pkg])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8(output.stdout).ok()?;
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_version(v: &str) -> Option<(u32, u32)> {
    let mut parts = v.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    Some((major, minor))
}
