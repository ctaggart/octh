# cargo install bindgen
# http://llvm.org/releases/download.html

$env:LIBCLANG_PATH = "C:\Program Files\LLVM\lib"
$env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
$env:PATH = "$HOME\.cargo\bin;$env:PATH"
$env:RUST_BACKTRACE = 1

bindgen --verbose --output=src/lib.rs bindgen2.h -- `
-x c++ `
-ferror-limit=0