extern crate cbindgen;

use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/api/c_api.rs");
    println!("cargo:rerun-if-changed=migrations/");

    #[allow(clippy::eq_op)]
    if env!("CARGO_PKG_NAME") != "lorecore" {
        return;
    }

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Generate C header file
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_no_includes()
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate C header file.")
        .write_to_file("lorecore_api.h");

    // Call cargo fmt
    let mut cmd = Command::new("cargo");
    cmd.arg("fmt");
    cmd.output().expect("Failed to execute cargo fmt");
}
