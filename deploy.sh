#!/bin/sh

(cd web && npm run build)
cp web/bundle.js web/index.html deploy
cp web/wasm/wasm_bg.wasm deploy/wasm
