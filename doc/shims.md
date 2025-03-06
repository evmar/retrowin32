"Shims" are the word used within retrowin32 for the mechanism for x86 ->
retrowin32 (and back) calls.

Executables can call system functions provided in a DLL implemented by
retrowin32. The mechanism to connect the call to retrowin32 code was rewritten
in 2024 and described in
[this blog post](https://neugierig.org/software/blog/2024/09/retrowin32-syscalls.html)
which eventually I ought to fold into here.

In the simple case, Rust functions like `kernel32.dll!Foo` call into our custom
`kernel32.dll` stub, which forwards to `retrowin32_syscall`, which then has
emulator-specific implementations.

When a shim is invoked, the code that invoked it looks like this:

```
foo.exe:
  ...
  push 1234  ; argument to Foo
  call [kernel32!Foo]

kernel32!Foo:
  call [retrowin32_syscall]
  ret 4  ; clean up argument

retrowin32.dll!retrowin32_syscall:
  ...specific to emulator backend...
```

So at the time `retrowin32_syscall` is invoked, the stack looks like:

```
return address within kernel32.dll!Foo, after the call
return address within foo.exe, after the call
arg1 to Foo
```

## Async calls

The complex case is when a Rust function needs to call back into x86 code. x86
emulation executes one basic block of instructions at a time, while our Rust
shim functions execute to completion synchronously, so the latter cannot
directly call the former. (In [Rosetta mode](x86-64.md) all functions just
complete synchronously, none of the following applies.)

To handle these, we make functions that call back into x86 into "async" Rust
functions that return a Future.

1. A given shim winapi function like IDirectDraw7::EnumDisplayModes needs to
   call back to x86 with each available display mode.
2. To do so, we change it to an async function:

```
#[win32_derive::dllexport]
async fn EnumDisplayModes(...) -> ... {
  ...setup code... 
  // Call into x86 function and await its result.
  shims::call_x86(some_ptr, vec![args]).await;
  ...return to x86 caller, as before...
}
```

3. The dllexport transform notices that async type and forwards to push_async,
   which redirects the x86 to call async_executor next.
4. async_executor picks up the Future returned in step #2 and runs it.

Concretely when EnumDisplayModes is called, the "simple case" shim logic as
described above runs as before, but rather than returning to the caller we
instead also push a call to async_executor, which adds itself to the call stack
and runs the async state machine. In the case of call_x86 that means the x86
code eventually invoked there will return control back to async_executor.
