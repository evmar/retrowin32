#!/bin/sh

# Builds retrowin32 as a x86_64 Darwin exe.

# Arguments passed through to the underlying linker.
# - Shrink pagezero from 4gb to 4kb so we can use lower 32 bits of memory:
#     -segalign 0x1000 -pagezero_size 0x1000
# - Put all our own content above 4gb:
#     -image_base 0x100000000
# - Disable PIE, required for moving image base:
#     -no_pie -no_fixup_chains
# - Put our RESV32 section at 0x1000 to ensure nothing like malloc claims
#   the now available lower memory:
#     -segaddr RESV32 0x1000
linker_args="-segalign 0x1000 -pagezero_size 0x1000 -image_base 0x100000000 -no_pie -no_fixup_chains -segaddr RESV32 0x1000"

# To pass the linker args through all the intermediate build layers,
# we want to end up with a RUSTFLAGS like
#   -C link_arg=-Wl,-segalign,0x1000,etc
link_flag="-C link_arg=-Wl"
for arg in $linker_args; do
    link_flag="$link_flag,$arg"
done

# relocation=model=dynamic-no-pic needed for disabling PIE as well.
# --print link-args
export RUSTFLAGS="-C relocation-model=dynamic-no-pic $link_flag"

exec cargo build --target x86_64-apple-darwin -p retrowin32 "$@"
