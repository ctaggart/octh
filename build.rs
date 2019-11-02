extern crate bindgen;
use std::env;

#[allow(dead_code)]
fn bindgen(target: &str) {

    let bindgen_header = &env::var("BINDGEN_HEADER");
    let header =
        match bindgen_header {
            Ok(x) => x,
            Err(_) => "src/octhelp.h"
        };
    // let default_header = "src/octhelp.h";
    // let header = &bindgen_header.as_ref().unwrap_or(default_header.to_owned());

    let mut builder = bindgen::Builder::default()
        .header(header)
        .clang_arg("-v") // verbose
        .clang_arg("-x") // -x c++
        .clang_arg("c++")
        .clang_arg("-std=gnu++11")
        .enable_cxx_namespaces()
        .whitelist_type("octave.*")
        .whitelist_function("octave.*")
        .opaque_type("octave.refcount")
        .use_core()
        // .raw_line("#![allow(warnings)]")
        // .raw_line("extern crate core;")
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
                // .clang_arg("-I/usr/local/include/octave-6.0.0");
                .clang_arg("-I/home/linuxbrew/.linuxbrew/Cellar/octave/5.1.0_6/include/octave-5.1.0")
        },
        "x86_64-apple-darwin" => {
            // brew install octave
            builder = builder
                // failing tests on macOS https://github.com/rust-lang/rust-bindgen/issues/1619
                .layout_tests(false)
                .clang_arg("-isysroot")
                .clang_arg("/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk")
                // .clang_arg("/Library/Developer/CommandLineTools/SDKs/MacOSX10.14.sdk")
                // .clang_arg("-I/usr/local/opt/llvm/include")
                .clang_arg("-I/usr/local/opt/octave/include/octave-5.1.0")
                // .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include");
        },
        _ => (),
    }

    if bindgen_header.is_err() {
        // creates a __bingen.ii file
        builder.dump_preprocessed_input()
            .expect("unable to dump input");
    }

    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/bindings.rs")
    // let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let target = &env::var("TARGET").unwrap();
    // println!("{}", target);

    // generate binding
    bindgen(target);

    // compile helper
    // https://github.com/alexcrichton/cc-rs
    // https://docs.rs/cc/1.0.46/cc/struct.Build.html#method.cpp_link_stdlib
    // Which stdlib is octave built with?
    let mut build = cc::Build::new();
    build.cpp(true)
        .flag("-std=gnu++11")
        .file("src/octhelp.cc");
    
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            build.include(r"C:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0");
        },
        "x86_64-unknown-linux-gnu" => {
            // build.include("/usr/local/include/octave-6.0.0");
            build.include("/home/linuxbrew/.linuxbrew/Cellar/octave/5.1.0_6/include/octave-5.1.0");
        },
        "x86_64-apple-darwin" => {
            build.flag("-isysroot");
            build.flag("/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk");
            build.include("/usr/local/opt/octave/include/octave-5.1.0");
        },
        _ => (),
    }

    build.compile("octhelp");

    // add libraries for linking
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\bin");
            println!("cargo:rustc-link-lib=octave-7");
            println!("cargo:rustc-link-lib=octinterp-7");
        },
        "x86_64-unknown-linux-gnu" => {
            // println!("cargo:rustc-link-search=/usr/local/lib/octave/6.0.0");
            println!("cargo:rustc-link-search=/home/linuxbrew/.linuxbrew/Cellar/octave/5.1.0_6/lib/octave/5.1.0");
            // println!("cargo:rustc-link-lib=octave");
            // println!("cargo:rustc-link-lib=octinterp");
        },
        "x86_64-apple-darwin" => {
            // brew install llvm octave
            // export PATH=/usr/local/opt/llvm/bin:$PATH
            println!("cargo:rustc-link-search=/usr/local/opt/octave/lib/octave/5.1.0");
            println!("cargo:rustc-link-lib=octave");
            println!("cargo:rustc-link-lib=octinterp");
        },
        _ => (),
    }
}