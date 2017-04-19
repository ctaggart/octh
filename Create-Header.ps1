$env:PATH = "C:\Octave\Octave-4.2.1\bin;$env:PATH"
& gcc -E -v `
-x c++ `
-std=gnu++11 `
-IC:\Octave\Octave-4.2.1\include\octave-4.2.1\octave `
-IC:\Octave\Octave-4.2.1\include `
-IC:\Octave\Octave-4.2.1\lib\gcc\x86_64-w64-mingw32\4.9.4\include\c++ `
-IC:\Octave\Octave-4.2.1\lib\gcc\x86_64-w64-mingw32\4.9.4\include\c++\x86_64-w64-mingw32 `
-o bindgen2.h `
bindgen.h