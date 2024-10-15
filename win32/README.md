# win32 implementation

This directory contains retrowin32's implementation of the win32 API.

## winapi

The Windows API lives in modules found in `src/winapi`, which are broken down
roughly by the underlying DLLs that Windows uses.

Each given Windows function like `CreateWindowA()` is written in Rust. A custom
`dllexport` attribute is then picked up by a code generator to generate the
plumbing for those functions into the machinery so that they can be called from
the x86 executable.

## Code generator

The code generator traverses the `dllexport` attributes and generates a Rust
glue module as well as the code for special `.dll` files that end up inlined
into the final binaries.

To rerun the code generator after an API change, run `make` in this directory. I
typically `make -j -C win32` from the project root.

The `Makefile` also compiles these `.dll` files, which requires `clang-cl` from
LLVM in your `$PATH`. I just unpack a pre-built
[binary release of theirs](https://releases.llvm.org/download.html).

## The `Host` trait

This crate provides a [`Host` trait](src/host.rs) that captures the API required
of the hosting process, which means it (mostly) does not contain logic around
whether it's running natively or on the web.

The intent is this crate captures all the details around e.g. the flags to any
given Windows function, while the implementation of `Host` provided one layer
out hooks that up to the specific requirements to whatever the OS provides --
realistically, either native (found in `../cli`) or web (found in
`../web/glue`).

## Code layout

- [`derive/`](derive/) -- macro implementations, primarily `dllexport`
  annotations on functions, see README
- `dll/` -- currently unused work in progress; generated win32 dlls for builtin
  API
- `src/pe` -- PE file parsing/loading, in theory independent of any winapi or
  emulation concerns
- `src/winapi` -- the Windows API, see above `winapi` docs
- [`lib/`](lib/) -- support machinery to generate a DLL used in test, see README
