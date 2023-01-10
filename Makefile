all: web/web.js

web/glue/pkg/stamp: web/glue/src/lib.rs
	cd web/glue && ./build.sh

web/web.js: web/glue/pkg/stamp
	cd web && npx tsc

fmt-rust:
	cargo fmt
fmt-dprint:
	web/node_modules/.bin/dprint fmt

fmt: fmt-rust fmt-dprint
