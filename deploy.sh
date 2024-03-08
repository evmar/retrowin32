#!/bin/sh

set -e

mkdir -p deploy
make deploy opt=1
cp web/{*.css,*.html,*.wasm,*.png} deploy
