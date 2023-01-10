all: web/web.js

web/wasm/pkg/stamp: web/wasm/src/lib.rs
	cd web/wasm && ./build.sh

web/web.js: web/wasm/pkg/stamp
	cd web && npx tsc

fmt-rust:
	cargo fmt
fmt-dprint:
	web/node_modules/.bin/dprint fmt

fmt: fmt-rust fmt-dprint
