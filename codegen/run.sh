#!/bin/bash

set -e

cargo run ../exe/winapi/winapi.exe > t.wat
wat2wasm t.wat
node run.js
