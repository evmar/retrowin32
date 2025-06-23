#!/bin/sh

set -e

XWIN="${XWIN:-$HOME/.xwin-cache/splat}"
clang_flags="-fuse-ld=lld -target i686-pc-windows-msvc -mno-sse"
warning_flags="/W4"
# reproducible builds, optimize for size, no security cookies
# note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
cflags="/Brepro /Os /GS- /std:c++17 $warning_flags"
sdk_flags="/winsysroot $XWIN"
link_flags="/nodefaultlib kernel32.lib /subsystem:console"

exec clang-cl $clang_flags $cflags $sdk_flags ops.cc util.cc math.cc fpu.cc /link $link_flags
