#!/bin/bash

# Dumps assembly of a given function in a Mac binary.

set -e

bin=~/.local/opt/llvm/bin

path="$1"
if [[ ! $path ]]; then
    echo "usage: $0 path/to/binary [filter] [index]"
    exit 1
fi

nm() {
    $bin/llvm-nm "$@" | cut -w -f3 | grep -v '^$' | uniq
}

filter="$2"
if [[ ! $filter ]]; then
    nm "$path"
    exit 0
fi

index="$3"
if [[ ! $index ]]; then
    i=1
    for sym in $(nm "$path" | grep "$filter" | c++filt); do
        echo "$i $sym"
        i=$((i + 1))
        if ((i > 10)); then
            break
        fi
    done
    exit 0
fi

sym=$(nm "$path" | grep "$filter" | sed "${index}q;d")
$bin/llvm-otool -tV -p "$sym" "$path" | c++filt
