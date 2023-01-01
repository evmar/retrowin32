#!/bin/sh

exec zig build-exe hello.zig -O ReleaseSmall --strip -target i386-windows -fsingle-threaded
