#!/bin/sh

set -e

(cd wasm && wasm-pack build -t web --release)
(cd web && npm run build)
mkdir -p deploy/wasm
cp web/bundle.js web/index.html deploy
cp web/wasm/wasm_bg.wasm deploy/wasm
