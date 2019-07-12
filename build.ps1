$env:PATH="C:\Octave\Octave-5.1.0.0\mingw64\bin;$env:PATH"
cargo clean
cargo build --target x86_64-pc-windows-gnu --release