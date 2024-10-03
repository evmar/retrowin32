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
  ;;
release)
  cargo build --target wasm32-unknown-unknown --profile release
  wasm-bindgen --out-dir pkg --typescript --target web --reference-types \
    "../../target/wasm32-unknown-unknown/$profile/glue.wasm"
  ;;
lto)
  cargo build --target wasm32-unknown-unknown --profile lto
  wasm-bindgen --out-dir pkg --typescript --target web --reference-types \
    "../../target/wasm32-unknown-unknown/$profile/glue.wasm"
  wasm-opt -O --enable-reference-types pkg/glue_bg.wasm -o pkg/glue_bg.wasm-opt
  mv pkg/glue_bg.wasm-opt pkg/glue_bg.wasm
  ;;
*)
  echo "error: profile=debug or release"
  exit 1
esac
