use std::env;
use std::path::PathBuf;

fn main() {
    // Compile cityhash
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .file("cityhash/src/city.cc")
        .include("cityhash/src")
        .flag_if_supported("-std=c++11")
        // Include the directory where config.h is located
        .include("cityhash/src")
        .warnings(false)
        .compile("cityhash");

    // Generate bindings with C++ config
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-std=c++11")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-I./cityhash/src")
        .allowlist_function("CityHash64")
        .allowlist_function("CityHash64WithSeed")
        .allowlist_function("CityHash64WithSeeds")
        .allowlist_function("CityHash32")
        // These options are needed for bindgen 0.71
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(false)
        .layout_tests(false)
        // Make sure the extern blocks are correctly generated as unsafe
        .blocklist_item(".*pair") // Avoid std::pair issues
        // Safety settings for bindgen 0.71
        .merge_extern_blocks(false)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Unsigned)
        // Map C++ types to Rust types
        .opaque_type("std::.*")
        .blocklist_type("uint8")
        .blocklist_type("uint32")
        .blocklist_type("uint64")
        .raw_line("// Type definitions")
        .raw_line("pub type uint8 = u8;")
        .raw_line("pub type uint32 = u32;")
        .raw_line("pub type uint64 = u64;")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=cityhash/src/city.h");
    println!("cargo:rerun-if-changed=cityhash/src/city.cc");
    println!("cargo:rerun-if-changed=cityhash/src/config.h");
}
