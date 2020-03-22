#!/usr/bin/env bash
set -e

cargo build --release --package=vndf-launcher
echo "::set-env name=LINUX_CLIENT::target/release/vndf-launcher"
