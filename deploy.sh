#!/bin/sh

set -e

(cd wasm && wasm-pack build -t web --release)
(cd web && npm run build)
mkdir -p deploy
cp web/{bundle.js,index.html,wasm.wasm} deploy
