#!/usr/bin/env bash

# Exit the script with a nonzero exit code if:
# * any individual command finishes with a nonzero exit code, or
# * we access any undefined variable.
set -eu

# Print out Rust backtraces on panic. Useful for minimizing a particular panic.
export RUST_BACKTRACE=1

# If the `libclang.so` you're using for `bindgen` isn't the system
# `libclang.so`, let the linker find it.
#export LD_LIBRARY_PATH=~/path/to/your/directory/containing/libclang

# Make sure that the reduced test case is valid C/C++ by compiling it. If it
# isn't valid C/C++, this command will exit with a nonzero exit code and cause
# the whole script to do the same.
# clang[++ --std=c++14] -c ./pre_processed_header.hpp
clang-3.9 --std=c++11 -c __bindgen.ii

# Run `bindgen` and `grep` for the thing your hunting down! Make sure to include
# `2>&1` to get at stderr if you're hunting down a panic.
# ~/src/rust-bindgen/target/debug/bindgen \
#     ./pre_processed_header.hpp \
#     [ <extra flags> ] \
#     2>&1 \
#     | grep "<pattern in generated bindings or a panic string or ...>"

# ~/rust-bindgen/target/debug/bindgen --help > bindgen-help.txt
~/rust-bindgen/target/debug/bindgen \
    --output __bindgen.rs \
    --enable-cxx-namespaces \
    --no-layout-tests \
    --no-doc-comments \
    --no-derive-debug \
    --no-derive-copy \
    --whitelist-type octave.* \
    --whitelist-function octave.* \
    --use-core \
    --raw-line "extern crate core;" \
    --opaque-type std::.* \
    __bindgen.ii \
    -- -v -x c++ -std=c++11 \
    2>/dev/null

rustc __bindgen.rs \
    2>&1 \
    | grep 'error\[E0412\]: cannot find type `i32` in module `std::os::raw`'