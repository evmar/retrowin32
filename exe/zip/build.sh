#!/bin/sh

set -e

cd $(dirname $0)
exec cargo build --release --target i586-pc-windows-msvc "$@"
