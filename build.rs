extern crate cbindgen;

use std::{env, process::Command};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Generate C header file
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_no_includes()
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate C header file.")
        .write_to_file("lorecore_api.h");

    // Create/Update schema.rs
    let mut cmd = Command::new("diesel");
    cmd.arg("print-schema").arg("--database-url=example.db");
    let schema = cmd.output().expect("Failed to execute diesel migrations");
    std::fs::write("src/sql/schema.rs", schema.stdout).expect("Failed to write schema.rs");
}
