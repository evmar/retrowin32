#!/bin/sh

(cd web && npm run build)
mkdir -p deploy/wasm
cp web/bundle.js web/index.html deploy
cp web/wasm/wasm_bg.wasm deploy/wasm
