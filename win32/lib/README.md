# .lib files

Some code in retrowin32 is built as win32 binaries. This code sometimes needs to
link against DLLs. This directory defines the .lib files passed to the linker
for doing this.

- `retrowin32.dll`: builtin "syscall" function for calls from x86->retrowin32.
- `retrowin32_test.dll`: builtin for testing retrowin32.
