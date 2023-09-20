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

## Compile-time features matrix

There are two build time toggles implemented as Rust "features":

- `cpuemu`: enable the x86 emulator
- `sdl`: use sdl2 for graphics

Web builds require `cpuemu` and no `sdl`.

Native builds can either use `cpuemu` (for non-x86) or not (which requires
native x86/Rosetta on Mac). Native builds without `sdl` are headless and crash
if they run any exe with graphics calls.

## Code layout

- `x86/` -- the x86 emulator
- `win32/` -- the win32 emulator
  - `win32/src/winapi/` -- the Windows API implementation
- `cli/` -- a command-line emulator runner
- `web/` -- a webapp that runs the emulator in a browser
  - `web/glue/` -- wasm glue for the `win32/` API
- `exe/` -- some sample Windows executables
