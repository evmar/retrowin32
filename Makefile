# Targets:
# - deploy: prepare web bundle for deploying
# - emu: cpu emulator-based native build
# - rosetta: rosetta-based native x86 build
# - unicorn: cpu emulator-based native build
# - fmt: run all code formatting
# Flags:
# $ make deploy opt=1

opt?=0
cargoflags=
profile=debug
ifeq ($(opt), 1)
	profile=lto
	cargoflags=--profile=$(profile)
endif

all: deploy emu rosetta unicorn


wasm web/glue/pkg/glue.d.ts:
	cd web/glue && profile=$(profile) ./build.sh
web-check:
	cd web && npx tsc
deploy/bundle.js: web/glue/pkg/glue.d.ts
	cd web && npm run build
deploy: wasm deploy/bundle.js
.PHONY: wasm deploy web-check


emu:
	source cli/sdl-brew.sh && cargo build -p retrowin32 -F x86-emu,sdl $(cargoflags)
.PHONY: emu


rosetta:
	source cli/sdl-manual.sh && ./build-rosetta.sh $(cargoflags)
.PHONY: rosetta


unicorn:
	source cli/sdl-brew.sh && cargo build -p retrowin32 -F x86-unicorn,sdl $(cargoflags)
.PHONY: unicorn


fmt-rust:
	cargo fmt
fmt-dprint:
	web/node_modules/.bin/dprint fmt
fmt: fmt-rust fmt-dprint
.PHONY: fmt-rust fmt-dprint fmt
