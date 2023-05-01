#!/bin/sh

exec zig build-exe hello.zig -O ReleaseSmall -target x86-windows-msvc -fsingle-threaded
