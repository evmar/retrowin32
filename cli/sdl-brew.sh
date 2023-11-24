#!/bin/sh
# Set up environment for building with SDL found in homebrew.

export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"

