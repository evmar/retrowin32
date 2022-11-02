#!/bin/bash

# run like `./build.sh --profiling` for profiling output

mode=${1:---dev}
exec wasm-pack build -t web $mode
