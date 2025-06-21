#!/bin/sh

set -e

cd $(dirname $0)
# See notes in exe/rust/.  TODO: merge this there.
exec cargo build --release "$@"
