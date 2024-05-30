#!/bin/sh

set -e

linker_args="$linker_args -Ttext 0x7d000000 --verbose -Map,rw32.map"
#linker_args="$linker_args --section-start=resv32=0x1000"

# To pass the linker args through all the intermediate build layers,
# we want to end up with a RUSTFLAGS like
#   -C link_arg=-Wl,-segalign,0x1000,etc
link_flag="-C link_arg=-Wl"
for arg in $linker_args; do
    link_flag="$link_flag,$arg"
done

#export RUSTFLAGS="$RUSTFLAGS -C relocation-model=dynamic-no-pic -C code-model=large $link_flag"
export RUSTFLAGS="$RUSTFLAGS -C relocation-model=dynamic-no-pic $link_flag"

exec cargo build --target x86_64-unknown-linux-gnu -p retrowin32 --features x86-64 "$@"
