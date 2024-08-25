#!/bin/sh

set -e

~/.local/opt/llvm/bin/llvm-dlltool -m i386 -d retrowin32.def -l retrowin32_test.lib -k
~/.local/opt/llvm/bin/llvm-dlltool -m i386 -d retrowin32.def -l retrowin32.lib -k
