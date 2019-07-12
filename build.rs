extern crate bindgen;

#[allow(dead_code)]
fn bindgen() {
    let builder = bindgen::Builder::default()
        .header("src/octhelp.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("--target=x86_64-w64-mingw32")
        .clang_arg("-nobuiltininc")
        .clang_arg(r"-IC:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0")
        .enable_cxx_namespaces()
        .whitelist_type("octave.*")
        .whitelist_function("octave.*")
        .use_core()
        // .raw_line("#![no_std]")
        .raw_line("extern crate core;")
        .opaque_type("std::.*");

    // creates a __bingen.ii file
    // builder.dump_preprocessed_input()
    //     .expect("unable to dump input");

    let bindings = builder.generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    cc::Build::new()
        .cpp(true)
        .include(r"C:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0")
        .file("src/octhelp.cc")
        .compile("octhelp");

    // bindgen();

    println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\bin");
    println!("cargo:rustc-link-lib=octave-7");
    println!("cargo:rustc-link-lib=octinterp-7");
}

// pacman -S mingw-w64-x86_64-gcc
// rustup target add x86_64-pc-windows-gnu
// rustup default x86_64-pc-windows-gnu
// rustup component add rustfmt
// $env:PATH="C:\Octave\Octave-5.1.0.0\mingw64\bin;$env:PATH"
// $env:PATH="C:\msys64\mingw64\bin;$env:PATH"
// cargo build