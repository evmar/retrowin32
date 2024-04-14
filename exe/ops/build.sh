#!/bin/sh

set -e

export PATH=~/.local/opt/llvm/bin:$PATH

clang_flags="-fuse-ld=lld -target i686-pc-windows-msvc"
warning_flags="-Wall -Wno-nonportable-system-include-path -Wno-c++98-compat
    -Wno-unsafe-buffer-usage -Wno-old-style-cast -Wno-missing-prototypes"
# reproducible builds, optimize for size, no security cookies
# note: /Zi for debug info (useful for ghidra) but it breaks build reproducibility
cflags="/Brepro /Os /GS- /std:c++17 $warning_flags"
xwin_path=~/redist
sdk_flags="/vctoolsdir $xwin_path/crt /winsdkdir $xwin_path/sdk"
link_flags="/nodefaultlib kernel32.lib /subsystem:console"

exec clang-cl $clang_flags $cflags $sdk_flags ops.cc util.cc math.cc /link $link_flags
