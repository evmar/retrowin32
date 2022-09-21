all: web/web.js

wasm/pkg/stamp: wasm/src/lib.rs
	cd wasm && ./build.sh

web/web.js: wasm/pkg/stamp
	cd web && npx tsc

fmt-rust:
	cargo fmt
fmt-web:
	cd web && npm run fmt

fmt: fmt-rust fmt-web
