$env:PATH = "C:\Octave\Octave-4.2.1\bin;$env:PATH"
& gcc -E -v `
-x c++ `
-std=c++11 `
-IC:\Octave\Octave-4.2.1\include\octave-4.2.1\octave `
-IC:\Octave\Octave-4.2.1\include `
-o bindgen2.h `
bindgen.h