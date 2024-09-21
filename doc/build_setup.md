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

If you are building the various helper test programs or see build errors about a
missing `$XWIN` variable, you need the Windows SDK (see below for setup).

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

### Windows SDK, aka XWIN

The retrowin32 tree includes test programs that produce win32 exe files. To
build these you need some Windows headers. The
[xwin tool](https://github.com/Jake-Shadle/xwin) automates downloading these.

To install it, follow their instructions. Then run it like:

```
$ xwin --accept-license --arch x86 splat
# also pass --disable-symlinks if on a case insensitive filesystem, like Mac
```

This will unpack the files into `~/.xwin-cache/splat`.
