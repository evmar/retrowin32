#!/bin/sh

# This script is used to deploy the website.
# It expects the `pages` branch to be checked out in the `deploy` subdir;
# set that up with:
#   git worktree add deploy pages

set -e

make -C web profile=lto
(cd web && npm run build)
cp web/*.css web/*.html web/*.wasm web/*.png deploy
