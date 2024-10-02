#!/bin/bash

# Dumps assembly of a given function using lldb.

# -m: include source
# -n: specify function name
exec lldb "$1" -b -o "dis -m -n $2"
