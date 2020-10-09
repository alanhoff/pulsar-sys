extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=pulsar");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", root.join("include").display()))
        .header("wrapper.h")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
