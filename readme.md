# Rust bindings for GNU Octave oct.h

[GNU Octave](https://www.gnu.org/software/octave/) has an [external code interface](https://www.gnu.org/software/octave/doc/interpreter/External-Code-Interface.html), allowing you to add functionality. Octave functions can be built using the C++ `oct.h`. They get packaged into a library with the name of the new function and an `.oct` extension. See [Getting Started with Oct-Files](https://octave.org/doc/interpreter/Getting-Started-with-Oct_002dFiles.html#Getting-Started-with-Oct_002dFiles). This Rust crate exposes binding to `oct.h`, so that you may build the Octave functions using Rust instead of C++.

## Examples
- [helloworld](example-helloworld) - A port of the C++ helloworld

## Building
- Run `cargo build`. Octave must be installed. For development, I'm using [Octave from Homebrew](https://formulae.brew.sh/formula/octave) on both Mac and Linux. I installed it by running `brew install octave`. [build.rs](octh/build.rs) will probably need some adjustments if you are using other locations.

## Issues
- [#10](https://github.com/ctaggart/octh/issues/10) build.rs is too platform specific
- [#36](https://github.com/ctaggart/octh/issues/36) macOS Catalina failing with __node_pointer defined multiple times