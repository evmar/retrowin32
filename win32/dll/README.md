This directory contains crates for the Windows system DLLs.

There are two pices to a system DLL:

1. There is a Rust implementation crate that contains the code that runs
   natively.
2. There is a generated `.dll` file that doesn't contain much code; instead each
   function immediately invokes sysenter which transfers control to the above
   Rust code. We generate these real DLL files though so the system DLLs behave
   similarly at runtime to real DLLs.

These latter DLLs are generated from the dllexport annotations the functions in
the crate. The win32/derive generator parses them and generated the
assembly/.def inputs.
