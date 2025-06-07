#!/bin/sh

# Builds retrowin32 as a x86_64 Darwin executable
# using the CPU's (or Rosetta's) existing support for 32-bit code.
#
# The binary needs particular flags and layout for this to work.

# Arguments passed through to the underlying linker.
linker_args=""
# - Shrink pagezero from 4gb to 4kb so we can use lower 32 bits of memory:
linker_args="$linker_args -segalign 0x1000 -pagezero_size 0x1000"
# - Put all our own content above 3gb:
linker_args="$linker_args -image_base 0xc0001000"
# - Disable PIE, required for moving image base:
linker_args="$linker_args -no_pie -no_fixup_chains"
# - Put our RESV32 section at 0x1000 to ensure nothing like malloc claims
#   the now available lower memory:
linker_args="$linker_args -segaddr RESV32 0x1000"

# To pass the linker args through all the intermediate build layers,
# we want to end up with a RUSTFLAGS like
#   -C link_arg=-Wl,-segalign,0x1000,etc
link_flag="-C link_arg=-Wl"
for arg in $linker_args; do
    link_flag="$link_flag,$arg"
done

# relocation=model=dynamic-no-pic needed for disabling PIE as well.
# --print link-args
# TODO: try the `-C relocation-model=static` + --target trick used in build-linux-64.sh instead.
export RUSTFLAGS="$RUSTFLAGS -C relocation-model=dynamic-no-pic $link_flag"

# note: faster debug cycle if you remove 'sdl'
exec cargo build --target x86_64-apple-darwin -p retrowin32 --no-default-features --features x86-64,sdl "$@"
