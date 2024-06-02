#!/bin/sh

set -e

make deploy opt=1
cp web/*.css web/*.html web/*.wasm web/*.png deploy
