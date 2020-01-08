use std::env;

fn main() {
    let target = &env::var("TARGET").unwrap();

    if let Ok(lib) = env::var("OCTAVE_LIB") {
        println!("cargo:rustc-link-search={}", lib);
    }
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            println!("cargo:rustc-link-lib=octave-7");
            println!("cargo:rustc-link-lib=octinterp-7");
        },
        "x86_64-unknown-linux-gnu" => {
        },
        "x86_64-apple-darwin" => {
            println!("cargo:rustc-link-lib=octave");
            println!("cargo:rustc-link-lib=octinterp");
        },
        _ => (),
    }
}
