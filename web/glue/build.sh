#!/bin/bash

# cargo install wasm-bindgen-cli
# cargo install wasm-opt
# target_dir="$(cargo metadata --format-version 1 --no-deps | jq -r .target_directory)"

set -e

cd "$(dirname "$0")"

profile="${profile:-release}"

case $profile in
debug)
  cargo build --target wasm32-unknown-unknown --profile dev
  wasm-bindgen --out-dir pkg --typescript --target web --reference-types \
    "../../target/wasm32-unknown-unknown/debug/glue.wasm"
  sed -i. -e '/import.meta/d' pkg/glue.js
  ;;
release|lto)
  cargo build --target wasm32-unknown-unknown --profile $profile
  wasm-bindgen --out-dir pkg --typescript --target web --reference-types \
    "../../target/wasm32-unknown-unknown/$profile/glue.wasm"
  #wasm-opt -O pkg/glue_bg.wasm 
  ;;
*)
  echo "error: profile=debug or release"
  exit 1
esac
