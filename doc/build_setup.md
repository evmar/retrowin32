# Build setup

To build retrowin32 you need [the Rust toolchain](https://rustup.rs/).

If running graphical programs locally, you will need
[SDL](https://www.libsdl.org/) (see below for setup).

If you are building the web-based retrowin32, see
[the web/ docs](../web/README.md).

If you're making Windows API changes or see build errors about missing
`clang-cl`, you need
[the Clang toolchain](https://releases.llvm.org/download.html). See
[win32/README.md](../win32/README.md).

If you are building the various helper test programs found under `exe/`, see the
[README there](../exe/README.md).

## SDL

Mac:

```
$ brew install sdl2
$ export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

(If targeting Rosetta you need an x86 SDL, see [x86-64 docs](x86-64.md).)

Debian/Ubuntu:

```
$ sudo apt install libsdl2-dev
```
