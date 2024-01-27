#!/bin/sh

set -e

make deploy opt=1
mkdir -p deploy
cp web/{bundle.js,index.html,debugger.html,wasm.wasm} deploy
