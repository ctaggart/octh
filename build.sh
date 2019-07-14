#!/bin/sh -e
# export PATH=~/.local/share/flatpak/runtime/org.kde.Sdk/x86_64/5.12/active/files/bin:$PATH
export PATH=/usr/lib/sdk/rust-stable/bin/:$PATH
# export LIBRARY_PATH=/home/ctaggart/.local/share/flatpak/runtime/org.kde.Sdk/x86_64/5.12/active/files/lib/x86_64-linux-gnu
# cargo clean
cargo build --target x86_64-unknown-linux-gnu --release