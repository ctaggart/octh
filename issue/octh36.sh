#!/bin/bash
cp /Users/cameron/github/octh/Cargo.toml .
cp /Users/cameron/github/octh/build.rs .
mkdir src
cp /Users/cameron/github/octh/src/lib.rs src
cp /Users/cameron/github/octh/src/octhelp.cc src
cp /Users/cameron/github/octh/src/octhelp.h src
BINDGEN_HEADER="__bindgen.ii" cargo build \
2>&1 \
| grep 'error\[E0428\]: the name `__node_pointer` is defined multiple times'

# https://github.com/rust-lang-nursery/rust-bindgen/issues
# export LLVM_CONFIG_PATH=/usr/local/opt/llvm/bin/llvm-config
# time creduce ./issue/octh36.sh __bindgen.ii
# time ~/gitshub/creduce/creduce/creduce ./issue/octh36.sh __bindgen.ii
