#!/usr/bin/env bash
set -e

TARGET=x86_64-pc-windows-msvc

cargo build --release --package=vndf-launcher --target=$TARGET

mkdir -p target/release-binaries
mv target/$TARGET/release/vndf-launcher \
    target/release-binaries/vndf-launcher-$TARGET.exe
