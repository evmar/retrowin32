.PHONY: all opt
all: web/web.js

web/glue/pkg/stamp: web/glue/src/lib.rs
	cd web/glue && ./build.sh

opt:
	cd web/glue && ./build.sh --profiling

web/web.js: web/glue/pkg/stamp
	cd web && npx tsc

fmt-rust:
	cargo fmt
fmt-dprint:
	web/node_modules/.bin/dprint fmt

fmt: fmt-rust fmt-dprint
