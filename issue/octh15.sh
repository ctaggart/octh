#!/usr/bin/env bash
set -eu
export RUST_BACKTRACE=1
clang-8 -c __bindgen.ii

~/rust-bindgen/target/release/bindgen \
    --output "__bindgen.rs" \
    --enable-cxx-namespaces \
    --opaque-type "octave.refcount" \
    --use-core \
    --raw-line "#![allow(warnings)]" \
    --raw-line "extern crate core;" \
    --opaque-type "std::.*" \
    __bindgen.ii \
    -- -v -x c++ -nobuiltininc -std=gnu++11 \
    2>/dev/null

rustc __bindgen.rs \
    2>&1 \
    | grep 'error\[E0432\]: unresolved import `self::super::super::u32`'

# https://github.com/rust-lang-nursery/rust-bindgen/issues
# time creduce ./issue/octh15.sh __bindgen.ii 