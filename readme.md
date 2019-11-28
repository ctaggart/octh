# Rust bindings for GNU Octave oct.h

[GNU Octave](https://www.gnu.org/software/octave/) has an [external code interface](https://www.gnu.org/software/octave/doc/interpreter/External-Code-Interface.html), allowing you to add functionality. Octave functions can be built using the C++ `oct.h`. They get packaged into a library with the name of the new function and an `.oct` extension. See [Getting Started with Oct-Files](https://octave.org/doc/interpreter/Getting-Started-with-Oct_002dFiles.html#Getting-Started-with-Oct_002dFiles). This Rust crate exposes bindings to `oct.h`, so that you may build the Octave functions using Rust instead of C++.

## Examples
- [helloworld](example-helloworld) - A port of the C++ helloworld

``` rust
use octh::*;

#[no_mangle]
pub extern "C" fn Ghelloworld (library: *const dynamic_library, relative: bool) -> *mut dld_function {
    let help = "Hello World Help String";
    dld_function_create(helloworld, library, "helloworld".into(), help.into(), relative)
}

extern "C" fn helloworld (argin: *const value_list, nargout: i32) -> value_list {
    let nargin = value_list_length(argin);
    println!("Hello World has {} input arguments and {} output arguments.", nargin, nargout);
    let mut argout = OctaveValueList::new(nargout);
    for i in 0..nargout {
        argout.set_value(i, Matrix::new().to_value());
    }
    argout.unwrap()
}
```

## Building
- Run `cargo build`. Octave must be installed. For development, I'm using [Octave from Homebrew](https://formulae.brew.sh/formula/octave) on both Mac and Linux. I installed it by running `brew install octave`. You will need to add these two environment variables, adjusted to your environment:

``` sh
export OCTAVE_INCLUDE=/home/linuxbrew/.linuxbrew/Cellar/octave/5.1.0_8/include/octave-5.1.0
export OCTAVE_LIB=/home/linuxbrew/.linuxbrew/Cellar/octave/5.1.0_8/lib/octave/5.1.0
```

## Issues
- [#10](https://github.com/ctaggart/octh/issues/10) build.rs is too platform specific
- [#36](https://github.com/ctaggart/octh/issues/36) macOS Catalina failing with __node_pointer defined multiple times