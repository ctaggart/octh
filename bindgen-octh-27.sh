#!/usr/bin/env bash
#set -u
export RUST_BACKTRACE=1
clang-3.9 --std=c++11 -c __bindgen.ii

~/rust-bindgen/target/release/bindgen \
    --output __bindgen.rs \
    --enable-cxx-namespaces \
    --whitelist-type octave.* \
    --whitelist-function octave.* \
    --use-core \
    --raw-line "extern crate core;" \
    __bindgen.ii \
    -- -v -x c++ -std=c++11 \
    2>/dev/null
rc=$?; if [[ $rc != 0 ]]; then exit $rc; fi

rustfmt __bindgen.rs 2>/dev/null
rustc --crate-type=lib __bindgen.rs 2>/dev/null

# return success if there is an error code is detected
rc=$?; if [[ $rc != 0 ]]; then exit 0; else exit 1; fi

# time creduce ./bindgen-bug.sh __bindgen.ii