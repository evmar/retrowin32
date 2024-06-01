#!/bin/sh

set -e

# Note: keep as much as possible in the build.rs so that cargo-show-asm picks it up too.

RUSTFLAGS='-C panic=abort' exec cargo build --target i586-pc-windows-msvc "$@"
