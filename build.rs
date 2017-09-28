extern crate bindgen;
extern crate gcc;

fn bindgen() {
    let builder = bindgen::Builder::default()
        .header("src/octhc.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")

        // Windows
        // .clang_arg("--target=x86_64-w64-mingw32")
        // .clang_arg("-nobuiltininc")
        // .clang_arg("-IC:/Octave/Octave-4.2.1/include/octave-4.2.1/octave")

        // Linux
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

        .enable_cxx_namespaces()
        .whitelist_type("octave.*")
        .whitelist_function("octave.*")

        .use_core()
        .raw_line(r#"extern crate core;"#)
        // .whitelist_type("std::string")
        // .opaque_type("std::.*")
        // .opaque_type("std::__cow_string")
        // .opaque_type("std::sentry")
        // .opaque_type("std::basic_ostream")
        // .rustfmt_bindings(false);
        .rustfmt_bindings(true);

    // creates a __bingen.ii file
    builder.dump_preprocessed_input()
        .expect("unable to dump input");

    let bindings = builder.generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    bindgen();
}