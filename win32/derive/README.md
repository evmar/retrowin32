# win32 derive macros

This crate implements some macros used by the win32 library as well as a code
generator that works over the same macros.

The main attribute is `dllexport`, which is used to mark a function as available
in the Windows API. The code generator gathers all of these to generate a
`builtins` module that plumbs arguments to/from the x86 stack and registers into
the Rust versions. The same attribute also triggers the tracing infrastructure,
which logs winapi functions as they're called.

The `TryFromEnum` derive macro adds a `try_from()` method to enums, mapping
integers back to enum values.

The `shims_from_x86` macro is used in generating vtables and I hope to remove it
soon so I won't discuss it further.
