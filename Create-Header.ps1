$env:PATH = "C:\Octave\Octave-4.2.1\bin;$env:PATH"
& clang -E -v `
-x c++ `
-std=c++11 `
-target x86_64-w64-mingw32 `
-IC:\Octave\Octave-4.2.1\include\octave-4.2.1\octave `
-o bindgen2.h `
bindgen.h