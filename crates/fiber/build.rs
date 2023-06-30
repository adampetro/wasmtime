use std::{env, fs, path::PathBuf};
use wasmtime_versioned_export_macros::versioned_suffix;

fn main() {
    let mut build = cc::Build::new();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if os == "windows" {
        println!("cargo:rerun-if-changed=src/windows.c");
        build.file("src/windows.c");
    } else if arch == "s390x" {
        println!("cargo:rerun-if-changed=src/unix/s390x.S");

        // cc does not preprocess macros passed with -D for `.s` files so need to do
        // it manually
        let asm = fs::read_to_string("src/unix/s390x.S").unwrap();
        let asm = asm.replace("VERSIONED_SUFFIX", versioned_suffix!());
        let out_dir = env::var("OUT_DIR").unwrap();
        let file_path = PathBuf::from(out_dir).join("s390x_preprocessed.S");
        fs::write(&file_path, asm).unwrap();
        build.file(file_path);
    } else {
        // assume that this is included via inline assembly in the crate itself,
        // and the crate will otherwise have a `compile_error!` for unsupported
        // platforms.
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }
    build.define(&format!("CFG_TARGET_OS_{}", os), None);
    build.define(&format!("CFG_TARGET_ARCH_{}", arch), None);
    build.compile("wasmtime-fiber");
}
