all: web/web.js

wasm/pkg/stamp: wasm/src/lib.rs
	cd wasm && ./build.sh

web/web.js: wasm/pkg/stamp
	cd web && npx tsc

win32/fmt:
	cd win32 && cargo fmt
wasm/fmt:
	cd wasm && cargo fmt
web/fmt:
	cd web && npm run fmt

fmt: wasm/fmt web/fmt win32/fmt
