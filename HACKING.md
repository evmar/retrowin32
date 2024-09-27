# Hacking on retrowin32

## Setup

See [build setup docs](doc/build_setup.md) for initial build requirements.

## CLI build

Build/run the CLI app:

```
$ cargo build -p retrowin32 -F x86-emu,sdl --profile=lto
$ ./target/lto/retrowin32 exe/zig_hello/hello.exe
```

The command line as seen by the program is an optional third argument:

```
$ ./target/lto/retrowin32 path/to/some.exe "c:\\some.exe arg1 arg2"
```

The `--win32-trace` flag controls tracing of win32 API calls. Passing `*` (which
must be quoted from the shell) makes retrowin32 trace all win32 calls.

### Rosetta

On Apple Silicon (ARM) Macs there is tentative support for running via the
Rosetta x86 emulator. See [doc/x86-64.md](doc/x86-64.md) for instructions.

## Web

Build/run the web app:

```
$ cd web
$ make
$ npm run serve
```

Optionally, `make profile=lto` for faster binary.

## Compile-time features matrix

The above commands cover the main things you'd build, but they wrap some more
subtle configuration.

To choose the x86 emulation strategy, you must pick a Rust "feature":

- `x86-emu`: retrowin32's own x86 emulator
- `x86-64`: generate x86-64 code, requires x86 CPU or Rosetta
- `x86-unicorn`: use [Unicorn](https://www.unicorn-engine.org/) (effectively
  QEMU) for x86 emulation (note: probably broken at this point)

To choose the rendering strategy, there is one further toggle:

- `sdl`: use sdl2 for graphics
- otherwise
  - non-web: headless mode, crash on any graphics calls
  - web: render to DOM

Web builds require `x86-emu` and no `sdl`.

### rust-analyzer

Rust's IDE support doesn't know about which features you're using. In VSCode I
configure it via the workspace config in `.vscode/settings.json`. You can add a
similar block to your user settings to override which emulator backend you're
working on.

## Building while developing

There are three build profiles:

- `lto`: slowest build, highest performance
- `debug`: fastest build, lowest performance
- `release`: compromise between the above two

To iterate while developing I often use a command like this, which builds and
runs in one invocation.

```
$ cargo run -p retrowin32 -F x86-emu -- --win32-trace '*' path/to/my/exe
```

And sometimes I add:

- `--release` when the debug build runs too slowly, and
- `-F x86-emu,sdl` for GUI support

If you make a change to functions exported in the `win32/src/winapi/` layer, you
must re-run the code generator as documented in [`win32/`](win32/).

## Code layout

The entries in this list that are links are links to READMEs with more
information.

- `x86/` -- the x86 emulator
- [`win32/`](win32/) -- the win32 emulator
- `cli/` -- a command-line emulator runner
- [`web/`](web/) -- a webapp that runs the emulator in a browser
- `exe/` -- some sample Windows executables
- `memory/` -- a memory abstraction shared by `x86` and `win32`
- [`appdb/`](appdb/) -- metadata about particular binaries
- `misc/` -- minor helper tools

## The website

The retrowin32 website is documented as part of the [web README](web/README.md).
