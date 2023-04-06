#!/bin/sh

set -e

(cd web/glue && wasm-pack build -t web --release)
(cd web && npm run build)
mkdir -p deploy
cp web/{bundle.js,index.html,debugger.html,wasm.wasm} deploy
