extern crate bindgen;
use std::env;

fn bindgen(target: &str, bindgen_header: &str, octave_include: Option<&str>) {

    let mut builder = bindgen::Builder::default()
        .header(bindgen_header)
        .clang_arg("-v") // verbose
        .clang_arg("-x") // -x c++
        .clang_arg("c++")
        .clang_arg("-std=gnu++11")
        .enable_cxx_namespaces()
        .whitelist_type("octave.*")
        .whitelist_function("octave.*")
        .whitelist_function("stdstring.*")
        .whitelist_function("Matrix.*")
        .opaque_type("octave.refcount")
        .use_core()
        .opaque_type("std::.*");

    if let Some(include) = octave_include {
        builder = builder.clang_arg(format!("-I{}", include));
    }

    match target {
        "x86_64-pc-windows-gnu" => {
            builder = builder
                .clang_arg("--target=x86_64-w64-mingw32");
        },
        "x86_64-unknown-linux-gnu" => {
        },
        "x86_64-apple-darwin" => {
            builder = builder
                // failing tests on macOS https://github.com/rust-lang/rust-bindgen/issues/1619
                .layout_tests(false)
                .clang_arg("-isysroot")
                .clang_arg("/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk");
        },
        _ => (),
    }

    // create a __bingen.ii file
    // if bindgen_header.is_err() {
    //     builder.dump_preprocessed_input()
    //         .expect("unable to dump input");
    // }

    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/bindings.rs")
    // let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let target = &env::var("TARGET").unwrap();
    let bindgen_header = env::var("BINDGEN_HEADER").unwrap_or("src/octhelp.h".to_owned());
    let octave_include = env::var("OCTAVE_INCLUDE").ok();
    let octave_lib = env::var("OCTAVE_LIB").ok();

    // bindgen(target, &bindgen_header, octave_include.as_deref()); // as_deref is Rust 1.4.0
    bindgen(target, &bindgen_header, octave_include.as_ref().map(String::as_ref));

    // compile helper https://docs.rs/cc https://github.com/alexcrichton/cc-rs
    let mut build = cc::Build::new();
    build.cpp(true)
        .flag("-std=gnu++11")
        .file("src/octhelp.cc");
    
    if let Some(include) = octave_include {
        build.include(include);
    }

    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
        },
        "x86_64-unknown-linux-gnu" => {
        },
        "x86_64-apple-darwin" => {
            build.flag("-isysroot");
            build.flag("/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk");
        },
        _ => (),
    }

    build.compile("octhelp");

    // add libraries for linking
    // needed for cargo test
    if let Some(lib) = octave_lib {
        println!("cargo:rustc-link-search={}", lib);
    }
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            println!("cargo:rustc-link-lib=octave-7");
            println!("cargo:rustc-link-lib=octinterp-7");
        },
        "x86_64-unknown-linux-gnu" => {
            println!("cargo:rustc-link-lib=octave");
            println!("cargo:rustc-link-lib=octinterp");
        },
        "x86_64-apple-darwin" => {
            // brew install llvm octave
            // export PATH=/usr/local/opt/llvm/bin:$PATH
            println!("cargo:rustc-link-lib=octave");
            println!("cargo:rustc-link-lib=octinterp");
        },
        _ => (),
    }
}