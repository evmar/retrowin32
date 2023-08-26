#!/bin/sh

set -e

exec ~/.local/opt/llvm/bin/llvm-dlltool -m i386 -d retrowin32.def -l retrowin32.lib -k
