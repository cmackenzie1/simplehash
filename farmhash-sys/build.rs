use std::env;
use std::path::PathBuf;

fn main() {
    // Build the simplified FarmHash implementation
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .file("src/farmhash/farmhash.cc")
        .file("wrapper.cc")
        .include(".")
        .flag_if_supported("-std=c++11")
        .warnings(false)
        .compile("farmhash");

    // Generate bindings with C++ config
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-std=c++11")
        .clang_arg("-x")
        .clang_arg("c++")
        .allowlist_function("farmhash_hash32")
        .allowlist_function("farmhash_hash32_with_seed")
        .allowlist_function("farmhash_hash64")
        .allowlist_function("farmhash_hash64_with_seed")
        .allowlist_function("farmhash_hash64_with_seeds")
        .allowlist_function("farmhash_fingerprint128")
        // Options for bindgen
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(false)
        .layout_tests(false)
        .blocklist_item(".*pair") // Avoid std::pair issues
        .merge_extern_blocks(false)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Unsigned)
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.cc");
    println!("cargo:rerun-if-changed=src/farmhash/farmhash.h");
    println!("cargo:rerun-if-changed=src/farmhash/farmhash.cc");
}
