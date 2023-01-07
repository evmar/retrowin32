#!/bin/bash

diff -u out.txt <(cargo run -p retrowin32 -- ops.exe | tr -d '\r')
