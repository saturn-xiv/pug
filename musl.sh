#!/bin/sh

set -e

PKG_CONFIG_ALLOW_CROSS=1
PKG_CONFIG_ALL_STATIC=  1
PKG_CONFIG_DIR=
export PKG_CONFIG_ALL_STATIC PKG_CONFIG_ALLOW_CROSS PKG_CONFIG_DIR

target=x86_64-unknown-linux-musl
rustup target add $target --toolchain=nightly
cargo build --target $target --release $@
