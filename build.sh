#!/bin/sh -e
# For building within the flatpak environment.
# Octave and Rust must both be installed.
# flatpak run --command=sh --devel org.octave.Octave

# flatpak
# export PATH=/usr/lib/sdk/rust-stable/bin/:$PATH
export PATH=/snap/octave/5/usr/bin:$PATH
# cargo clean
cargo build --target x86_64-unknown-linux-gnu --release