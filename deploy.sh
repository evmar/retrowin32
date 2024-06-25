#!/bin/sh

set -e

make -C web profile=lto
(cd web && npm run build)
cp web/*.css web/*.html web/*.wasm web/*.png deploy
