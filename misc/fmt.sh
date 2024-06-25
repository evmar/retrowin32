#!/bin/sh

# Run all code formatters.

set -e

cargo fmt
web/node_modules/.bin/dprint fmt
