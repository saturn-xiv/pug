#!/bin/sh

set -e

cargo check
cargo check --no-default-features
cargo check --no-default-features --features "mysql,redis,rabbitmq,sodium,zeromq"
cargo check --no-default-features --features "sqlite,redis,rabbitmq,sodium,zeromq"
