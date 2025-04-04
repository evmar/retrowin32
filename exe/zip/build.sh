#!/bin/sh

set -e

cd $(dirname $0)
RUSTFLAGS='-C panic=abort' exec cargo build --release --target i586-pc-windows-msvc "$@"
