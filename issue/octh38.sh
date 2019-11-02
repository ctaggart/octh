#!/bin/bash
# time creduce ./issue/octh38.sh __bindgen.ii

cp /home/cameron/github/octh/Cargo.toml .
cp /home/cameron/github/octh/build.rs .
mkdir src
cp /home/cameron/github/octh/src/lib.rs src
cp /home/cameron/github/octh/src/octhelp.cc src
cp /home/cameron/github/octh/src/octhelp.h src
BINDGEN_HEADER="__bindgen.ii" cargo test \
2>&1 \
| grep "thread 'root::octave::bindgen_test_layout_output_system' panicked at 'assertion failed:"