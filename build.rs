// build.rs
use std::env;
use cc::Build;
use std::path::PathBuf;


fn main() {
    // address.c randombytes.c merkle.c wots.c wotsx1.c utils.c utilsx1.c fors.c sign.c
    // fips202.c hash_shake.c thash_shake_robust.c
    // params.h address.h randombytes.h merkle.h wots.h wotsx1.h utils.h utilsx1.h fors.h api.h  hash.h thash.h
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate Rust bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("src_c/params/params-sphincs-sha2-tests.h") // Specify the header to generate bindings for
        .clang_arg("-Isrc_c") // Include path for header files
        .clang_arg("-DPARAMS=sphincs-sha2-tests") // Define macro for parameterization
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    Build::new()
    // .file("src_c/hash.h")
    // .file("src_c/params.h")
    // .file("src_c/address.h")
    
    // // .file("src_c/merkle.h")
    // .file("src_c/wots.h")
    // // .file("src_c/wotsx1.h")
    // .file("src_c/utils.h")
    // .file("src_c/utilsx1.h")
    // .file("src_c/fors.h")
    // .file("src_c/api.h")
    // .file("src_c/hash.h")
    // .file("src_c/thash.h")
        // .file("src_c/randombytes.h")
        .file("src_c/params/params-sphincs-sha2-tests.h")
        .file("src_c/randombytes.c")
        .file("src_c/address.c")
        .file("src_c/merkle.c")
        .file("src_c/wots.c")
        .file("src_c/wotsx1.c")
        .file("src_c/utils.c")
        .file("src_c/utilsx1.c")
        .file("src_c/fors.c")
        .file("src_c/sign.c")
        .file("src_c/sha2.c")
        .file("src_c/hash_sha2.c")
        .file("src_c/thash_sha2_simple.c")
        //.file("src_c/fips202.c")
        //.file("src_c/hash_shake.c")
        //.file("src_c/thash_shake_robust.c")
        .include("src_c")
        .flag("-std=c99")
        .flag("-DPARAMS=sphincs-sha2-tests")
        .opt_level(3)              // Equivalent to -O3 optimization
        .flag("-O3")                // Explicitly adding the optimization flag
        .compile("sphincs_wrap_c");

    // Tell cargo to tell rustc to link the system shared library.
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=sphincs_wrap_c");

    println!("cargo:rerun-if-changed=src_c");
    println!("cargo:rerun-if-changed=src_c/params/params-sphincs-sha2-tests.h");
}