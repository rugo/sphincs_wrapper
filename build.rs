// build.rs
use std::env;
use cc::Build;
use std::path::PathBuf;


fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate Rust bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("src_c/ref/params/params-sphincs-sha2-tests.h") // Specify the header to generate bindings for
        .clang_arg("-Isrc_c") // Include path for header files
        .clang_arg("-DPARAMS=sphincs-sha2-tests") // Define macro for parameterization
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    Build::new()
        .file("src_c/ref/params/params-sphincs-sha2-tests.h")
        .file("src_c/ref/randombytes.c")
        .file("src_c/ref/address.c")
        .file("src_c/ref/merkle.c")
        .file("src_c/ref/wots.c")
        .file("src_c/ref/wotsx1.c")
        .file("src_c/ref/utils.c")
        .file("src_c/ref/utilsx1.c")
        .file("src_c/ref/fors.c")
        .file("src_c/ref/sign.c")
        .file("src_c/ref/sha2.c")
        .file("src_c/ref/hash_sha2.c")
        .file("src_c/ref/thash_sha2_simple.c")
        .include("src_c/ref")
        .flag("-std=c99")
        .flag("-DPARAMS=sphincs-sha2-tests")
        .opt_level(3)              // Equivalent to -O3 optimization
        .flag("-O3")                // Explicitly adding the optimization flag
        .compile("sphincs_wrap_c");

    // Tell cargo to tell rustc to link the system shared library.
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=sphincs_wrap_c");

    println!("cargo:rerun-if-changed=src_c");
    println!("cargo:rerun-if-changed=src_c/ref/params/params-sphincs-sha2-tests.h");
}