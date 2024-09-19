#!/bin/sh

# Builds and prints the size of the wasm bundle to stdout.
# For use in https://github.com/evmar/git-metrics

set -e

rm -f glue/pkg/glue_bg.wasm
if [ -f web/Makefile ]; then
    (make -C web profile=lto) 1>&2
else
    # In older versions, Makefile was at the root of the project.
    (make wasm opt=1) 1>&2
fi

wc -c < web/wasm.wasm
