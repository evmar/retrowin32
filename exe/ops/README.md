# x86 operation tests

To verify retrowin32's implementation of different x86 instructions, we can run
the same program in retrowin32 and a different x86 implementation (either native
or some other emulator).

This directory contains a program that exercises different instructions and
prints their results. The file `out.txt` in principle contains the program's
output when run "correctly" (usually via Rosetta). The script `run.sh` runs the
same program via retrowin32 and diffs the output to see whether it matches.

## Adding new tests

1. modify the c++ source as appropriate to exercise your new operation
2. update the expected output with a command like:
   ```
   $ wine ops.exe > out.txt
   ```
3. verify retrowin32 matches by `run.sh`
