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

    // Create dummy database
    let mut cmd = Command::new("diesel");
    cmd.arg("setup").arg("--database-url=dummy.db");
    cmd.output().expect("Failed to execute diesel migrations");
    // Create/Update schema.rs
    let mut cmd = Command::new("diesel");
    cmd.arg("print-schema").arg("--database-url=dummy.db");
    let schema = cmd.output().expect("Failed to execute diesel migrations");
    std::fs::write("src/sql/schema.rs", schema.stdout).expect("Failed to write schema.rs");
    // Delete dummy.db
    std::fs::remove_file("dummy.db").expect("Failed to delete dummy.db");
}
