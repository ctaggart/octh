#!/bin/bash
# time creduce ./issue/octh38.sh __bindgen.ii

cp $HOME/github/octh/Cargo.toml .
cp $HOME/github/octh/build.rs .
mkdir src
cp $HOME/github/octh/src/lib.rs src
cp $HOME/github/octh/src/octhelp.cc src
cp $HOME/github/octh/src/octhelp.h src
BINDGEN_HEADER="__bindgen.ii" cargo test \
2>&1 \
| grep "thread 'root::octave::bindgen_test_layout_output_system' panicked at 'assertion failed:"
