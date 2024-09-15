#!/bin/sh

# Run all code formatters.

set -e

if [ "$1" = "--check" ]; then
    if ! (
        cargo fmt -- --check &&
        dprint check
    ); then
        echo
        echo "error: formatting check failed; run 'misc/fmt.sh' to fix"
        exit 1
    fi
else
    cargo fmt
    dprint fmt
fi
