This crate corresponds roughly to the Windows OS, as seen by the Windows API
implementation:

- a `System` trait that Windows APIs all take as their first argument
- cross-DLL concepts like memory mapping, heaps, etc.

This will let us build the Windows DLLs independently from one another and from
the implementation of `System`.
