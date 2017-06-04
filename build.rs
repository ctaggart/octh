extern crate bindgen;
extern crate gcc;

// a helper library for octh with c exports
fn libocthc() {
    // https://github.com/servo/rust-bindgen/blob/master/bindgen-integration/build.rs
    // https://github.com/alexcrichton/gcc-rs/blob/master/src/lib.rs
    gcc::Config::new()
        .compiler("C:/Octave/Octave-4.2.1/bin/gcc.exe")
        .cpp(true)
        .file("src/octhc.cc")
        .include("C:/Octave/Octave-4.2.1/include/octave-4.2.1/octave")
        // .flag("-std=c++11")
        .flag("-std=gnu++11")
        .compile("libocthc.a");
}

fn bindgen() {
    let mut builder = bindgen::Builder::default()
        .no_unstable_rust()
        .header("src/octhc.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        // .clang_arg("-std=c++11")
        .clang_arg("-std=gnu++11")
        .clang_arg("--target=x86_64-w64-mingw32")
        .clang_arg("-nobuiltininc")
        .clang_arg("-IC:/Octave/Octave-4.2.1/include/octave-4.2.1/octave")
        .enable_cxx_namespaces()
        .opaque_type(".*") // Should have a set of used template params for every item id
        // .whitelisted_type("octave_.*")
        .layout_tests(false)
        .generate_comments(false);

        // let whitelist = [
        //     "octave_value_list"
        // ];
        // for &ty in whitelist.iter() {
        //     builder = builder.whitelisted_type(ty);
        // }

        // let opaque_types = [
        //     "std::basic_streambuf___streambuf_type",
        //     "std::basic_istream_sentry_traits_type",
        //     "std::basic_streambuf" // std::basic_streambuf<_CharT>
        // ];
        // for &ty in opaque_types.iter() {
        //     builder = builder.opaque_type(ty);
        // }

        // let blacklist = [
        //     "_CType"
        // ];
        // for &ty in blacklist.iter() {
        //     builder = builder.hide_type(ty);
        // }

    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    libocthc();
    // bindgen();
}