extern crate bindgen;
use std::env;

#[allow(dead_code)]
fn bindgen(target: &str) {
    let mut builder = bindgen::Builder::default()
        .header("src/octhelp.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        .enable_cxx_namespaces()
        .whitelist_type("octave.*")
        .whitelist_function("octave.*")
        .opaque_type("octave.refcount")
        .use_core()
        .raw_line("#![allow(warnings)]")
        .raw_line("extern crate core;")
        .opaque_type("std::.*");

    match target {
        "x86_64-pc-windows-gnu" => {
            builder = builder
                .clang_arg("--target=x86_64-w64-mingw32")
                .clang_arg(r"-IC:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0");
        },
        "x86_64-unknown-linux-gnu" => {
            builder = builder
                // .clang_arg("-I/app/include/octave-5.1.0")
                // .clang_arg("-I/usr/lib/gcc/x86_64-unknown-linux-gnu/8.3.0/include");
                .clang_arg("-I/usr/local/include/octave-6.0.0");
        },
        _ => (),
    }

    // creates a __bingen.ii file
    // builder.dump_preprocessed_input()
    //     .expect("unable to dump input");

    let bindings = builder.generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    let target = &env::var("TARGET").unwrap();

    // compile helper
    let mut build = cc::Build::new();
    build.cpp(true)
        .file("src/octhelp.cc");
    
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            build.include(r"C:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0");
        },
        "x86_64-unknown-linux-gnu" => {
            // build.include("/app/include/octave-5.1.0");
            build.include("/usr/local/include/octave-6.0.0");
        },
        _ => (),
    }

    build.compile("octhelp");

    // generate binding
    // bindgen(target);

    // add libraries for linking
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\bin");
            println!("cargo:rustc-link-lib=octave-7");
            println!("cargo:rustc-link-lib=octinterp-7");
        },
        "x86_64-unknown-linux-gnu" => {
            println!("cargo:rustc-link-search=/usr/local/lib/octave/6.0.0");
            println!("cargo:rustc-link-lib=octave");
            println!("cargo:rustc-link-lib=octinterp");
        },
        _ => (),
    }
}