#!/bin/sh
# Set up environment for building with SDL found in homebrew.

if [ "$(uname)" = "Linux" ]; then
    export LIBRARY_PATH="$LIBRARY_PATH:/usr/lib"
else
    export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
fi

