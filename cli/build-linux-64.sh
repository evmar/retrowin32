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
# We use dynamic-no-pic for a subtle reason.  We want no-pic for the retrowin32
# binary, but the various derive crates we make require dynamic linking, so we
# can't just build static.  That is a host vs target compilation setting issue,
# but in the x86-64 case (as distinct from e.g. Rosetta) host and target are
# the same CPU arch.
export RUSTFLAGS="-C relocation-model=dynamic-no-pic $link_flag"

exec cargo build -p retrowin32 --no-default-features --features x86-64
