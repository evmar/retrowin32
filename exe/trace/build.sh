#!/bin/sh

exec zig build-exe trace.zig -O ReleaseSmall -target x86-windows-msvc -fsingle-threaded
