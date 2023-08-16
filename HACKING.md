# Hacking on retrowin32

## Building

Build/run the CLI app:

```
$ cargo build -p retrowin32 --release --features cpuemu,sdl
$ ./target/release/retrowin32 exe/zig_hello/hello.exe
```

Rosetta mode (see doc/x86-64.md):

```
$ ./build-rosetta.sh
$ ./target/x86_64-apple-darwin/debug/retrowin32 exe/zig_hello/hello.exe
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
