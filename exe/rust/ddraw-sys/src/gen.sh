#!/bin/sh

cd $(dirname $0)
xwin=~/.xwin-cache/splat/
inc="-I$xwin/sdk/include/um -I$xwin/sdk/include/shared -I$xwin/crt/include -I$xwin/sdk/include/ucrt"
ddraw="$xwin/sdk/include/um/ddraw.h"
allowlist="--allowlist-function DirectDrawCreate"
allowlist="$allowlist --allowlist-type IDirectDraw"
allowlist="$allowlist --allowlist-type DDSURFACEDESC"

# disable layout tests because it makes rust-analyzer angry
exec bindgen --no-layout-tests $allowlist $ddraw -- -target i686-pc-windows-msvc $inc > bindgen.rs
