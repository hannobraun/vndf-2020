#!/usr/bin/env bash
set -e

cargo test --verbose
cargo build --verbose --package vndf-server --bin vndf-server
cargo build --verbose --package vndf-tests
cargo build --verbose --package vndf-launcher --bin vndf-launcher
