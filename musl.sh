#!/bin/sh

set -e

export PKG_CONFIG_ALLOW_CROSS=1

target=x86_64-unknown-linux-musl
rustup target add $target --toolchain=nightly
cargo build --target $target --release $@
