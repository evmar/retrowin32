# win32 implementation

This directory contains retrowin32's implementation of Windows and win32 API.

## Windows API

The Windows API lives in crates found in `dll/`. These crates attempt to be
independent from one another and from the outer win32's definition of the
emulator machinery; see the `System` trait described below.

Each given Windows function like `CreateWindowA()` is written in Rust. A custom
`dllexport` attribute is then picked up by a code generator to generate the
plumbing for those functions into the machinery so that they can be called from
the x86 executable.

### Code generator

The code generator traverses the `dllexport` attributes and generates a Rust
glue module as well as the code for special `.dll` files that end up inlined
into the final binaries.

To rerun the code generator after an API change, run `cargo minibuild` from the
project root.

The build also compiles these `.dll` files, which requires `clang-cl` from LLVM
in your `$PATH`. I just unpack a pre-built
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

## The `System` trait

Each DLL (e.g. user32) needs to make calls to the underlying system (e.g. to
call a window procedure). These kinds of calls are abstracted through the
`System` trait defined in the `win32-system` crate in `system/`.

This trait is then implemented by the larger `win32` crate, which brings all the
DLLs together along with the actual implementation of `System`

## Code layout

- `winapi/` -- core types needed by any Windows DLL implementation
- `system/` -- the underlying `System` interface used by DLLs
- `src` -- roughly the Windows OS, or the bits of Windows that lie outside of
  the win32 API, implements the `System` interface, ties in the x86 emulator
- [`derive/`](derive/) -- macro implementations, primarily `dllexport`
  annotations on functions, see README
- `dll/` -- implementation of the win32 API, split by underlying DLL; also,
  generated, mostly empty `.dll` files that map to the above
- [`lib/`](lib/) -- support machinery to generate a DLL used in test, see README
- [`extract/`](extract/) -- tool to generate win32 API definitions, see README
