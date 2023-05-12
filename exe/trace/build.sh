#!/bin/sh

# Use cpu=i686 here to avoid generating SSE instructions.

exec zig build-exe trace.zig -mcpu=i686 --verbose-llvm-cpu-features -O ReleaseSmall -target x86-windows-msvc -fsingle-threaded
