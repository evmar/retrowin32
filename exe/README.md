This directory contains some win32 executables used to test retrowin32.

- ops: dump results of x86 operations
- trace: Windows exe tracer using Windows debug API
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

## Generating test programs to load an arbitrary DLL

Get list of exports:

```
$ llvm-objdump -x path/to/dll
```

In theory there's a gendef tool, but you can write a `.def` by hand too:

```
LIBRARY foo
EXPORTS
  foo_bar@4 @1
```

The `@1` is the DLL ordinal (found in the objdump output), the `@4` is the
number of argument bytes (which shows up in the symbol name based on the
caller's type signature).

Generate a `.lib` with:

```
$ llvm-dlltool -d foo.def -l foo.lib -k -m i386
```

Importantly the `-k` there makes references to `foo_bar@4` map to the suffixless
symbol `foo_bar` in the DLL.

Finally, create the test program:

```zig
// std.os.windows.WINAPI is .Stdcall on i386
extern "foo" fn foo_bar(x: c_int) callconv(std.os.windows.WINAPI) void;
```

and build via

```
$ zig build-exe foo.zig -O ReleaseSmall -target x86-windows-msvc
```
