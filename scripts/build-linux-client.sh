#!/usr/bin/env bash
set -e

TARGET=x86_64-unknown-linux-gnu

cargo build --release --package=vndf-launcher --target=$TARGET

mkdir -p target/release-binaries
mv target/$TARGET/release/vndf-launcher \
    target/release-binaries/vndf-launcher-$TARGET.bin
