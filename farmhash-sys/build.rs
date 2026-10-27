use std::env;
use std::path::PathBuf;

fn main() {
    // Build the simplified FarmHash implementation
    let mut build = cc::Build::new();

    // Basic C++ configuration
    build
        .cpp(true)
        .file("src/farmhash/farmhash.cc")
        .file("wrapper.cc")
        .include(".")
        .flag_if_supported("-std=c++11")
        .define("NDEBUG", None);

    // Set endianness configuration
    if env::var("CARGO_CFG_TARGET_ENDIAN").unwrap_or_default() == "big" {
        build.define("FARMHASH_BIG_ENDIAN", "1");
    }

    // Compile with optimizations
    build.opt_level(3).warnings(false).compile("farmhash");

    // Generate bindings with C++ config
    let bindgen = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-std=c++11")
        .clang_arg("-x")
        .clang_arg("c++");

    // Configure bindgen
    let bindings = bindgen
        // 32-bit hash functions
        .allowlist_function("farmhash_hash32")
        .allowlist_function("farmhash_hash32_with_seed")
        .allowlist_function("farmhash_fingerprint32")
        // 64-bit hash functions
        .allowlist_function("farmhash_hash64")
        .allowlist_function("farmhash_hash64_with_seed")
        .allowlist_function("farmhash_hash64_with_seeds")
        .allowlist_function("farmhash_fingerprint64")
        .allowlist_function("farmhash_fingerprint64_with_seed")
        // 128-bit hash functions
        .allowlist_function("farmhash_fingerprint128")
        .allowlist_function("farmhash_fingerprint128_with_seed")
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

    // Trigger rebuild if any of these files change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.cc");
    println!("cargo:rerun-if-changed=src/farmhash/farmhash.h");
    println!("cargo:rerun-if-changed=src/farmhash/farmhash.cc");

    // Trigger rebuild if target architecture or endianness changes
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ENDIAN");
}
