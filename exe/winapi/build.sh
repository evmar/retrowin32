#!/bin/sh

set -e

XWIN="${XWIN:-~/.xwin-cache/splat}"

clang_flags="-fuse-ld=lld -target i686-pc-windows-msvc"
# reproducible builds, optimize for size, no security cookies
# note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
cflags="/Brepro /std:c++20 /Os /GS-"
sdk_flags="/vctoolsdir $XWIN/crt /winsdkdir $XWIN/sdk"
link_flags="/nodefaultlib /subsystem:console kernel32.lib"

exec clang-cl $clang_flags $cflags $sdk_flags winapi.cc /link $link_flags
