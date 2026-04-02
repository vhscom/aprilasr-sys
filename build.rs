use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

/// Use p! to debug as println! is reserved in build files.
#[allow(unused_macros)]
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

/// Builds April ASR speech-to-text library in C using vendored
/// CMake file specifically designed for the task. With a known
/// working CMake build of libaprilasr the script will generate
/// low-level bindings for April ASR following Rust *-sys crate
/// conventions in order to decouple higher-level wrappers.
fn main() {
    // Allow users to set include path as a convenience.
    if let Ok(include_dir) = env::var("APRIL_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    // Initialize vendored submodule if needed.
    // Check for a file inside the submodule since the directory itself
    // exists (but empty) in a fresh clone before submodule init.
    if !Path::new("vendor/april-asr/CMakeLists.txt").exists() {
        let status = Command::new("git")
            .args(["submodule", "update", "--init", "vendor/april-asr"])
            .status();
        if !matches!(status, Ok(s) if s.success()) {
            panic!(
                "Failed to initialize vendor/april-asr submodule. \
                 Run: git submodule update --init vendor/april-asr"
            );
        }
    }

    // Re-build when the API header, wrapper, or cmake config changes.
    println!("cargo:rerun-if-changed=vendor/april-asr/april_api.h");
    println!("cargo:rerun-if-changed=vendor/april-asr/CMakeLists.txt");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=APRIL_INCLUDE_DIR");

    // Build April ASR from source using CMake.
    // cmake::Config::build() returns the install prefix; libraries
    // are installed to <prefix>/lib/ by the upstream cmake rules.
    let dst = cmake::Config::new("vendor/april-asr").build();

    // Update linker search paths for local builds.
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=aprilasr");

    // Configure and generate April ASR bindgen bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Couldn't generate bindings!");

    // Write bindings to out dir.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}
