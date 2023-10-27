# Hacking on retrowin32

## Building

Build/run the CLI app:

```
$ cargo build -p retrowin32 --release --features x86-emu,sdl
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

To choose the x86 emulation strategy, you must pick a Rust "feature":

- `x86-emu`: retrowin32's own x86 emulator
- `x86-64`: generate x86-64 code
- `x86-unicorn`: use [Unicorn](https://www.unicorn-engine.org/) (effectively
  QEMU) for x86 emulation

There are one further build time toggle:

- `sdl`: use sdl2 for graphics

Web builds require `x86-emu` and no `sdl`.

Native builds can use any emulation strategy, but `x86-64` requires an x86
process (or Rosetta on Mac) to run. Native builds without `sdl` are headless and
crash if they run any exe with graphics calls.

## Code layout

- `x86/` -- the x86 emulator
- `win32/` -- the win32 emulator
  - `win32/src/winapi/` -- the Windows API implementation
- `cli/` -- a command-line emulator runner
- `web/` -- a webapp that runs the emulator in a browser
  - `web/glue/` -- wasm glue for the `win32/` API
- `exe/` -- some sample Windows executables
