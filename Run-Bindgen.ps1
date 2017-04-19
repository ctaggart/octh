# cargo install bindgen
# http://llvm.org/releases/download.html
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\lib"
$env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
$env:PATH = "$HOME\.cargo\bin;$env:PATH"
$env:RUST_BACKTRACE = 1

# bindgen --verbose --output=src/lib.rs bindgen.h -- `
# -x c++ `
# -std=gnu++11
# -IC:\Octave\Octave-4.2.1\include\octave-4.2.1\octave `
# -IC:\Octave\Octave-4.2.1\include `
# -IC:\Octave\Octave-4.2.1\lib\gcc\x86_64-w64-mingw32\4.9.4\include\c++ `
# -IC:\Octave\Octave-4.2.1\lib\gcc\x86_64-w64-mingw32\4.9.4\include\c++\x86_64-w64-mingw32

bindgen --verbose --output=src/lib.rs bindgen2.h