#!/bin/sh

set -e

XWIN="${XWIN:-$HOME/.xwin-cache/splat}"
clang_flags="-fuse-ld=lld -target i686-pc-windows-msvc -mno-sse -mno-sse2"
# reproducible builds, optimize for size, no security cookies
# note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
cflags="/std:c++20 /Brepro /Os /GS-"
sdk_flags="/winsysroot $XWIN"
link_flags="/nodefaultlib /subsystem:console gdi32.lib kernel32.lib user32.lib"

for src in cmdline.cc gdi.cc metrics.cc thread.cc; do
    echo $src
    clang-cl $clang_flags $cflags $sdk_flags $src /link $link_flags
done
