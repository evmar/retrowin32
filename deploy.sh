#!/bin/sh

set -e

make deploy
mkdir -p deploy
cp web/{bundle.js,index.html,debugger.html,wasm.wasm} deploy
