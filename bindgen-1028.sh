#!/usr/bin/env bash
set -eu
export RUST_BACKTRACE=1
clang-3.9 --std=c++11 -c __bindgen.ii

~/rust-bindgen/target/release/bindgen \
    --output __bindgen.rs \
    --enable-cxx-namespaces \
    --whitelist-type octave.* \
    --whitelist-function octave.* \
    --use-core \
    --raw-line "extern crate core;" \
    --opaque-type std::.* \
    __bindgen.ii \
    -- -v -x c++ -std=c++11 \
    2>/dev/null

# sed -i -e 's/::std::os::raw::i32/::std::os::raw::c_int/g' __bindgen.rs
sed -i -e 's/i32/c_int/g' __bindgen.rs

rustc --test __bindgen.rs
./__bindgen \
    2>&1 \
    | grep 'test root::bindgen_test_layout_octave_function ... FAILED'

# https://github.com/rust-lang-nursery/rust-bindgen/issues
# time creduce ./bindgen-bug.sh __bindgen.ii