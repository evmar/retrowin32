#!/bin/sh

set -e

wine_out=wine.log
rw32_out=rw32.log

./exe/rust/build.sh
cargo run -p retrowin32 --no-default-features -F x86-emu -- ./target/i586-pc-windows-msvc/release/ops.exe > $rw32_out
wine ./target/i586-pc-windows-msvc/release/ops.exe > $wine_out

exec code -d $rw32_out $wine_out
