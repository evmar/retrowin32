#!/bin/bash

set -eo pipefail

cd "$(dirname "$0")"
out=$(cargo run -p retrowin32 -F x86-emu -- ops.exe | tr -d '\r')
difft out.txt <(echo "$out")
