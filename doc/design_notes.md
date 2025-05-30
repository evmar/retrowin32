# Design notes

## Endianness

x86 is observably little-endian, so we must emulate memory in a little-endian
manner.

In principle the outer emulator code could be written to work on a big-endian
system. Initially I had abstractions in place to keep the x86 numbers and outer
number separate when appropriate -- see the commit that added this documentation
to find it -- but realistically I'm not going to run this on a non-little-endian
system and it's just more work.

## Alignment

The 32-bit process heap often contains data structures. E.g. to create a window
class the code will pass a pointer to a `WNDCLASSA` structure. In Rust, the
`#[repr(C)]` attribute helps lay out a `struct` in the same format, but
importantly Rust assumes that any `&WNDCLASS` points at memory that has been
properly _aligned_. Officially on Windows things are supposed to be aligned, but
it was not enforced so per Hyrum's law real programs pass around unaligned
pointers.

There is `#[repr(packed)]` that forces a struct to become unaligned, but taking
a reference to e.g. a `u32` within such a struct
[is actually undefined behavior(!)](https://github.com/rust-lang/rust/issues/27060).

Because of this alignment issue, retrowin32 attempts to often treat things as
plain slices of bytes, which are not aligned. For example, a UTF-16 string
buffer might be a `&[u8]` internally. And similarly, rather than constructing
pointers into the 32-bit process memory, we ought to instead copy structs from
and to it. I think the only true fix is carefully applying a library like
[zerocopy](https://docs.rs/zerocopy/latest/zerocopy/) that is designed to handle
these kinds of concerns.

## String encoding

Many Windows APIs come in `FooA()` and `FooW()` versions, corresponding to two
different string encodings:

- `A()` has 8-bit characters that use the current system code page
- `W()` has 16-bit characters, UTF-16 (or possibly WTF-16?)

Internally Windows surely uses the latter. The web platform's native string
format is the same.

Meanwhile, the Rust native `String` type is UTF-8. (Even the platform-specific
`OsString` is still UTF-8 on Windows, it just provides API to convert to the
Windows format.)

This leaves retrowin32 needing to juggle a variety of string types. I have
decided that internally it will use `String`, and support conversion at the
edges.

> Warning: retrowin32 is currently inconsistent about implementing this policy.

1. when coming in from user code, we convert from either system code page or
   UTF-16
2. to pass to system APIs on native (e.g. opening files, SDL calls), we
   typically pass UTF-8 through without conversion
3. to pass to system APIs on web, we need to convert to UTF-16, but we need to
   to make a copy of the string anyway (from wasm to JS heap)

When implementing a function that has `A()` and `W()` forms, a nice pattern is
to have a single shared implementation that accepts either `String`s or trait
objects that handle conversion (see the `Encoding` trait in `winapi`), so you
end up with only a single implementation.
