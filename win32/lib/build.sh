#!/bin/sh

set -e

exec ~/.local/opt/llvm/bin/llvm-dlltool -m i386 -d retrowin32_test.def -l retrowin32_test.lib -k
