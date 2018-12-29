#!/bin/sh

set -e

cargo check
cargo check --no-default-features --features "mysql"
cargo check --no-default-features --features "sqlite"
cargo build
cargo doc
cargo build --release
