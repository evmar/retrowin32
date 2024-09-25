#!/bin/sh

# Given an input assembly file, print the assembled machine code as bytes.

set -e -o pipefail

cd $(dirname "$0")

clang -c -target i386-apple-darwin "$1" -o - | objdump -D -
