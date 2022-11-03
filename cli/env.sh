#!/bin/sh

# Brew puts sdl in a path where linker can't find it.

export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"

