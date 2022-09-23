#!/bin/sh
# usage: run.sh path/to/the.exe

exec cargo run -p retrowin32 -- "$@"
