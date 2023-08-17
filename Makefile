# Targets:
# - deploy: prepare web bundle for deploying
# - rosetta: rosetta-based native x86 build
# - cli: cpu emulator-based native build
# - fmt: run all code formatting
# Flags:
# $ make deploy opt=1

opt?=0
cargoflags=
wasmpackflags=
ifeq ($(opt), 1)
	cargoflags=--release
	wasmpackflags=--profiling
endif

all: deploy rosetta cli


wasm web/glue/pkg/glue.d.ts:
	cd web/glue && ./build.sh $(wasmpackflags)
web/bundle.js: web/glue/pkg/glue.d.ts
	cd web && npm run build
deploy: wasm web/bundle.js
.PHONY: wasm deploy


rosetta:
	./build-rosetta.sh $(cargoflags)
.PHONY: rosetta


cli:
	cargo build -p retrowin32 -F cpuemu,sdl $(cargoflags)
.PHONY: cli


fmt-rust:
	cargo fmt
fmt-dprint:
	web/node_modules/.bin/dprint fmt
fmt: fmt-rust fmt-dprint
.PHONY: fmt-rust fmt-dprint fmt
