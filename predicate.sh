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
    --enable-cxx-namespaces \
    --no-recursive-whitelist \
    --no-layout-tests \
    --no-doc-comments \
    --no-derive-debug \
    --whitelist-type octave.* \
    --whitelist-function octave.* \
    --opaque-type octave_jmp_buf \
    --opaque-type std::string \
    --opaque-type Complex \
    --opaque-type FloatComplex \
    --opaque-type time_t \
    --opaque-type std::stack \
    --opaque-type std::list \
    --opaque-type Array \
    --opaque-type dim_vector \
    --opaque-type string_vector \
    --opaque-type std::map \
    __bindgen.ii \
    -- -v -x c++ -std=c++11 \
    2>/dev/null \
    | grep "pub m_stack_trace: root::std::string"