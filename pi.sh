#!/bin/sh

set -e

export PKG_CONFIG_ALLOW_CROSS=1

target=armv7-unknown-linux-musleabihf
rustup target add $target --toolchain=nightly
cargo build --target $target --release $@
