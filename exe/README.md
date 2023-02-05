This directory contains some win32 executables used to test retrowin32.

- ops: dump results of x86 operations
- winapi: MSVC Windows API (no C runtime)
- zig_hello: Zig hello world
- zip: compress/decompres a buffer (CPU-intensive)

## Notes on MSVC flags

- `/nologo`: don't print copyright goop
- `/Brepro`: reproducible build (no timestamp)
- `/std:c20`: c++ version
- `/EHsc`: allow exceptions
- `/Os`: optimize for size
- `/GS-`: disable security checks
- `/Fa:somepath`: asm output

- `/link`: linker options follow
- `/dynamicbase:no`: disable ASLR
- `/subsystem`: console binary
- `/debug`: debug info
- `/opt:ref`: enable optimizations despite debug info
