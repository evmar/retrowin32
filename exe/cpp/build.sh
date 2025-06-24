#!/bin/sh

set -e

XWIN="${XWIN:-$HOME/.xwin-cache/splat}"
clang_flags="-fuse-ld=lld -flto -target i686-pc-windows-msvc -mno-sse -mno-sse2"
# reproducible builds, optimize for size, no security cookies
# note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
cflags="/std:c++20 /Brepro /Os /GS- /MT"
sdk_flags="/winsysroot $XWIN"
link_flags="/subsystem:console ddraw.lib gdi32.lib kernel32.lib user32.lib"
# https://devblogs.microsoft.com/cppblog/introducing-the-universal-crt/
link_flags="$link_flags libcmt.lib libvcruntime.lib libucrt.lib"

for src in cmdline.cc ddraw.cc gdi.cc metrics.cc thread.cc; do
    echo $src
    clang-cl $clang_flags $cflags $sdk_flags $src /link $link_flags
done
