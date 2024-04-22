#!/bin/sh

set -e

export PATH=~/.local/opt/llvm/bin:$PATH

clang_flags="-fuse-ld=lld -target i586-pc-windows-msvc"
# reproducible builds, optimize for size, no security cookies
# note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
cflags="/Brepro /std:c++20 /Os /GS-"
xwin_path=~/redist
sdk_flags="/vctoolsdir $xwin_path/crt /winsdkdir $xwin_path/sdk"
link_flags="/nodefaultlib /subsystem:console kernel32.lib"

exec clang-cl $clang_flags $cflags $sdk_flags winapi.cc /link $link_flags
