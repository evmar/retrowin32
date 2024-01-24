#!/bin/bash

set -eo pipefail

out=$(cargo run -p retrowin32 -F x86-emu -- ops.exe | tr -d '\r')
diff -u out.txt <(echo "$out")
