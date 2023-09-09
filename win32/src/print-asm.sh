#!/bin/sh

set -e -o pipefail

cd $(dirname "$0")

clang -c -target i386-apple-darwin trampoline_x86.s -o - | objdump -D -

# clang -c -target x86_64-apple-darwin trampoline_x86-64.s -o - | objdump -D -
