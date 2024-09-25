profile?=debug

all: wasm index.html

appdb_entries = $(shell find ../appdb -type f -name '*.toml')
index.html: ../appdb/appdb.go index.tmpl $(appdb_entries)
	cd ../appdb && ./run.sh

wasm glue/pkg/glue.d.ts:
	cd glue && profile=$(profile) ./build.sh

web-check:
	npx tsc

.PHONY: wasm web-check
