extern crate bindgen;

fn bindgen() {
    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("bindgen.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("--target=x86_64-w64-mingw32")
        .clang_arg("-nobuiltininc")
        .clang_arg("-IC:/Octave/Octave-4.2.1/include/octave-4.2.1/octave")
        .enable_cxx_namespaces()
        // .opaque_type(".*")
        .whitelisted_type("octave_.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    bindgen();
}