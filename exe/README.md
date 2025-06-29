This directory contains some win32 executables used to test retrowin32.

- callback: exe that calls a testing retrowin32 API that calls back to exe
- cpp: various Windows test programs in C++
- ops: dump results of x86 operations
- trace: Windows exe tracer using Windows debug API
- winapi: MSVC Windows API (no C runtime)
- zig_hello: Zig hello world

## Symlinks

The various symlinks into a `deploy/` dir are intended to resolve to files found
in the `pages` branch of this repo. If developing locally, set it up via
`git workdir add deploy pages`.

### Windows SDK, aka XWIN

To build the C/Rust code in this directory you need some Windows headers and
libraries. The [xwin tool](https://github.com/Jake-Shadle/xwin) automates
downloading these.

To install it, follow their instructions. Then run it like:

```
$ cd  # to unpack into ~
$ xwin --accept-license --arch x86 splat --use-winsysroot-style --preserve-ms-arch-notation
```

This will unpack the files into `~/.xwin-cache/splat`, which is where the build
scripts assume it can be found. See the `$XWIN` environment variable in the
source tree otherwise.

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

## Cross compilation

- https://neugierig.org/software/blog/2024/02/cross-compile.html
- https://neugierig.org/software/blog/2024/04/cross-compile-2.html

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
