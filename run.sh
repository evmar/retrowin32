#!/bin/sh
# usage: run.sh path/to/the.exe

exec cargo run --release -p retrowin32 -- "$@"
