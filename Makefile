# Targets:
# - deploy: prepare web bundle for deploying
# - emu: cpu emulator-based native build
# - rosetta: rosetta-based native x86 build
# - unicorn: cpu emulator-based native build
# - linux: linux based x86 build
# - fmt: run all code formatting
# Flags:
# $ make deploy opt=1

dprint?=web/node_modules/.bin/dprint
opt?=0
cargoflags=
profile=debug
ifeq ($(opt), 1)
	profile=lto
	cargoflags=--profile=$(profile)
endif

all: deploy emu rosetta unicorn

web/index.html: appdb/appdb.go web/index.tmpl
	cd appdb && go run . -tmpl ../web/index.tmpl render > ../web/index.html

wasm web/glue/pkg/glue.d.ts:
	cd web/glue && profile=$(profile) ./build.sh
web-check:
	cd web && npx tsc
deploy/bundle.js: web/glue/pkg/glue.d.ts
	cd web && npm run build
deploy-exes:
	cd appdb && go run . deploy

deploy: wasm deploy-exes deploy/bundle.js web/index.html
.PHONY: wasm deploy-exes deploy web-check


emu:
	. cli/sdl-brew.sh && cargo build -p retrowin32 -F x86-emu,sdl $(cargoflags)
.PHONY: emu

rosetta:
	. cli/sdl-manual.sh && ./build-rosetta.sh $(cargoflags)
.PHONY: rosetta

unicorn:
	. cli/sdl-brew.sh && cargo build -p retrowin32 -F x86-unicorn,sdl $(cargoflags)
.PHONY: unicorn


fmt-rust:
	cargo fmt
fmt-dprint:
	$(dprint) fmt
fmt: fmt-rust fmt-dprint
.PHONY: fmt-rust fmt-dprint fmt
