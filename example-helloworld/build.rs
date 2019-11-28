fn main() {
    if let Ok(lib) = std::env::var("OCTAVE_LIB") {
        println!("cargo:rustc-link-search={}", lib);
    }
    println!("cargo:rustc-link-lib=octave");
    println!("cargo:rustc-link-lib=octinterp");
}