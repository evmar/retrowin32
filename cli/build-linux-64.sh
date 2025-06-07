#!/bin/sh

# Builds retrowin32 binary as a x86-64 Linux executable
# using the CPU's existing support for 32-bit code.
#
# The binary needs particular flags and layout for this to work.

set -e

linker_args="-Ttext-segment=0x7f000000"
# It appears the rest gets laid out immediately after.
# All we really care about is it not being earlier.
#linker_args="$linker_args -Trodata-segment=0x7f400000"
link_flag="-C link_arg=-Wl"
for arg in $linker_args; do
    link_flag="$link_flag,$arg"
done
export RUSTFLAGS="-C panic=abort -C relocation-model=static $link_flag"

# Note: explicitly passing --target here causes Rust to obey RUSTFLAGS
# only for the target, not the host (e.g. proc macros), which is the
# behavior we need.
exec cargo build --target x86_64-unknown-linux-gnu -p retrowin32 --no-default-features --features x86-64
