extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=pulsar");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", env::var("CARGO_MANIFEST_DIR").unwrap()))
        .header("wrapper.h")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
