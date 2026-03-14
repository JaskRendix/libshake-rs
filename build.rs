use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Always write an ffi.rs file so the crate compiles
    let out_path = PathBuf::from("src/osx/ffi.rs");

    // Only attempt real bindings on macOS *if* ForceFeedback exists
    if target == "macos" {
        let header = "/System/Library/Frameworks/ForceFeedback.framework/Headers/ForceFeedback.h";

        if Path::new(header).exists() {
            // Generate real bindings
            let bindings = bindgen::Builder::default()
                .header(header)
                .clang_arg("-framework")
                .clang_arg("ForceFeedback")
                .clang_arg("-framework")
                .clang_arg("IOKit")
                .allowlist_function("FF.*")
                .allowlist_type("FF.*")
                .allowlist_var("FF.*")
                .generate()
                .expect("Failed to generate ForceFeedback bindings");

            bindings
                .write_to_file(&out_path)
                .expect("Couldn't write bindings!");
            return;
        }
    }

    // Fallback: write stub
    std::fs::write(
        &out_path,
        "// ForceFeedback.framework not available on this macOS version.\n",
    )
    .unwrap();
}
