#!/bin/sh

set -e

cd $(dirname $0)
# config is set up in build.rs and .cargo/config.toml
exec cargo build --release "$@"
