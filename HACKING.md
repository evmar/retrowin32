# Hacking on retrowin32

## Building

Build/run the CLI app:

```
$ make emu opt=1
$ ./target/release/retrowin32 exe/zig_hello/hello.exe
```

Rosetta mode (see doc/x86-64.md):

```
$ make rosetta
$ ./target/x86_64-apple-darwin/debug/retrowin32 exe/zig_hello/hello.exe
```

Build/run the web app:

```
$ make deploy opt=1
$ cd web
$ npm run serve
```

## Compile-time features matrix

The above `make` commands cover the main things you'd build, but they wrap some
more subtle configuration.

To choose the x86 emulation strategy, you must pick a Rust "feature":

- `x86-emu`: retrowin32's own x86 emulator
- `x86-64`: generate x86-64 code, requires x86 CPU or Rosetta
- `x86-unicorn`: use [Unicorn](https://www.unicorn-engine.org/) (effectively
  QEMU) for x86 emulation

To choose the rendering strategy, there is one further toggle:

- `sdl`: use sdl2 for graphics
- otherwise
  - non-web: headless mode, crash on any graphics calls
  - web: render to DOM

Web builds require `x86-emu` and no `sdl`.

## Code layout

- `x86/` -- the x86 emulator
- `win32/` -- the win32 emulator
  - `win32/src/winapi/` -- the Windows API implementation
- `cli/` -- a command-line emulator runner
- `web/` -- a webapp that runs the emulator in a browser
  - `web/glue/` -- wasm glue for the `win32/` API
- `exe/` -- some sample Windows executables
- `memory/` -- a memory abstraction shared by `x86` and `win32`
