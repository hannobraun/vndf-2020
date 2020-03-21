#!/usr/bin/env bash
set -e

BINARIES=target/client-binaries
WIN_TARGET=x86_64-pc-windows-gnu

mkdir -p $BINARIES

echo "Building Windows binary..."
cross build --release --package vndf-launcher --target=$WIN_TARGET
cp target/$WIN_TARGET/release/vndf-launcher.exe $BINARIES

echo "Building Linux binary..."
cargo build --release --package vndf-launcher
cp target/release/vndf-launcher $BINARIES
