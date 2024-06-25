profile?=debug

all: wasm index.html

index.html: ../appdb/appdb.go index.tmpl
	cd ../appdb && ./run.sh

wasm glue/pkg/glue.d.ts:
	cd glue && profile=$(profile) ./build.sh

web-check:
	npx tsc

.PHONY: wasm web-check
