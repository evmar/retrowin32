#!/bin/sh

set -e

export XWIN=~/redist
exec cargo build --release --target i586-pc-windows-msvc
