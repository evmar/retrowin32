#!/bin/sh

set -e

rm -f glue/pkg/glue_bg.wasm
if [ -f web/Makefile ]; then
    (make -C web profile=lto) 1>&2
else
    (make wasm opt=1) 1>&2
fi

wc -c < web/wasm.wasm
