extern crate bindgen;

#[allow(dead_code)]
fn bindgen() {
    let builder = bindgen::Builder::default()
        .header("src/octhelp.h")
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        // .clang_arg("--target=x86_64-w64-mingw32")
        // .clang_arg("--target=x86_64-unknown-linux-gnu") // default
        .clang_arg("-nobuiltininc")
        // .clang_arg(r"-IC:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0")
        // .clang_arg("-I/home/ctaggart/.local/share/flatpak/app/org.octave.Octave/x86_64/stable/active/files/include/octave-5.1.0")
        .clang_arg("-I/app/include/octave-5.1.0")
        .clang_arg("-I/usr/lib/gcc/x86_64-unknown-linux-gnu/8.3.0/include")
        .enable_cxx_namespaces()
        .whitelist_type("octave.*")
        .whitelist_function("octave.*")
        .use_core()
        .raw_line("#![allow(warnings)]")
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

    // let out_dir = env::var("HOME").unwrap();
    // let dest_path = Path::new(&out_dir).join("hello.rs");

    cc::Build::new()
        .cpp(true)
        // .include(r"C:\Octave\Octave-5.1.0.0\mingw64\include\octave-5.1.0")
        // .include("/home/ctaggart/.local/share/flatpak/app/org.octave.Octave/x86_64/stable/active/files/include/octave-5.1.0")
        .include("/app/include/octave-5.1.0")
        .file("src/octhelp.cc")
        .compile("octhelp");

    // bindgen();

    // println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\bin");
    // println!("cargo:rustc-link-lib=octave-7");
    // println!("cargo:rustc-link-lib=octinterp-7");

    // println!("cargo:rustc-link-search=/home/ctaggart/.local/share/flatpak/runtime/org.kde.Sdk/x86_64/5.12/active/files/lib/x86_64-linux-gnu");
    // println!("cargo:rustc-link-search=/home/ctaggart/.local/share/flatpak/app/org.octave.Octave/x86_64/stable/active/files/lib/octave/5.1.0");
    println!("cargo:rustc-link-lib=octave");
    println!("cargo:rustc-link-lib=octinterp");
}