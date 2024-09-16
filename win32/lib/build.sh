#!/bin/sh

set -e

llvm-dlltool -m i386 -d retrowin32_test.def -l retrowin32_test.lib -k
llvm-dlltool -m i386 -d retrowin32.def -l retrowin32.lib -k
