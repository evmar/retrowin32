all: web/web.js

wasm/pkg/stamp: wasm/src/lib.rs
	cd wasm && ./build.sh

web/web.js: wasm/pkg/stamp
	cd web && npx tsc

fmt-rust:
	cargo fmt
fmt-dprint:
	web/node_modules/.bin/dprint fmt

fmt: fmt-rust fmt-dprint
