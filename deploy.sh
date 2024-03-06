#!/bin/sh

set -e

mkdir -p deploy
make wasm opt=1
cp web/{*.html,*.wasm,*.png} deploy
