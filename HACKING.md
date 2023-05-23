# Hacking on retrowin32

## Building

Build/run the CLI app:

```
$ cargo build -p retrowin32 --release
$ ./target/release/retrowin32
```

Build/run the web app:

```
$ make
$ cd web
$ npm run serve
```

## Code layout

- `x86/` -- the x86 emulator
- `win32/` -- the win32 emulator
  - `win32/src/winapi/` -- the Windows API implementation
- `cli/` -- a command-line emulator runner
- `web/` -- a webapp that runs the emulator in a browser
  - `web/glue/` -- wasm glue for the `win32/` API
- `exe/` -- some sample Windows executables
