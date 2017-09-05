extern crate bindgen;
extern crate gcc;

fn bindgen() {
    let mut builder = bindgen::Builder::default()
        // .no_unstable_rust()
        .header("src/octhc.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        // .clang_arg("--target=x86_64-w64-mingw32")
        // .clang_arg("-nobuiltininc")
        // .clang_arg("-IC:/Octave/Octave-4.2.1/include/octave-4.2.1/octave")
        .clang_arg("-I/home/cameron/octave/libinterp/octave-value")
        .clang_arg("-I/home/cameron/octave/libinterp/corefcn")
        .clang_arg("-I/home/cameron/octave-build")
        .clang_arg("-I/home/cameron/octave/liboctave/array")
        .clang_arg("-I/home/cameron/octave/liboctave/operators")
        .clang_arg("-I/home/cameron/octave/liboctave/cruft/misc")
        .clang_arg("-I/home/cameron/octave/liboctave/util")
        .clang_arg("-I/home/cameron/octave/liboctave/numeric")
        .clang_arg("-I/home/cameron/octave-build/liboctave/operators")
        .clang_arg("-I/home/cameron/octave-build/libinterp")
        .clang_arg("-I/home/cameron/octave/liboctave/system")
        // .clang_arg("-I/home/cameron/octave/libinterp/corefcn")
        .enable_cxx_namespaces()
        // .opaque_type(".*") // Should have a set of used template params for every item id
        .whitelisted_type("octave.*")
        // .whitelisted_type("jmp_buf")
        // .whitelisted_type("__jmp_buf_tag")
        .whitelisted_function("octave.*")
        .opaque_type("octave_jmp_buf")
        // .whitelisted_type("std::string")
        // .whitelisted_type("std::basic_string")
        // .opaque_type("std::basic_string")
        // .opaque_type("std::union")
        .whitelisted_type("std::string")
        .opaque_type("std::string")

        .whitelisted_type("Complex").opaque_type("Complex")
        .whitelisted_type("FloatComplex").opaque_type("FloatComplex")

        // .hide_type("octave::lu")

        .whitelisted_type("time_t")
        .opaque_type("time_t")
        
        // .whitelisted_type("std::stack")
        .opaque_type("std::stack")
        
        // .whitelisted_type("std::list")
        .opaque_type("std::list")
        
        .whitelisted_type("Array")
        .opaque_type("Array")
        // .whitelisted_type("string_vector").opaque_type("string_vector")

        .whitelisted_type("dim_vector")
        .opaque_type("dim_vector")

        .derive_debug(false)
        // .derive_copy(false)
        .whitelist_recursively(false)
        .layout_tests(false)
        .generate_comments(false);

    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    bindgen();
}