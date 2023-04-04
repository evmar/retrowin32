# retrowin32

A not-yet-working Windows emulator for the web (and maybe non-web?).

## Status

Doesn't really work for much yet, but I'm still tinkering!

Some sample programs:

- [Zig "hello world"](https://evmar.github.io/retrowin32/?exe=zig.exe).
- [A basic DirectDraw demo](https://evmar.github.io/retrowin32/?exe=BasicDD.exe),
  which came from
  [this intro site](https://www.codeproject.com/Articles/2370/Introduction-to-DirectDraw-and-Surface-Blitting)
- [Monolife](https://evmar.github.io/retrowin32/?exe=monolife/monolife.exe&file=monolife/monolife.dat)
  by Hatha (see it [on pouet.net](https://www.pouet.net/prod.php?which=7698))
  [Note: still buggy, but the opening sorta works!]

Click the "run" button in the corner to run these.

## The idea

Software today is written in a world where the platform continually changes --
code you release today may stop working next year as the APIs and deprecations
churn. You know what doesn't change? Dead platforms. Old video games written for
a NES will work forever because the NES won't ever change again.

win32, the Windows 32-bit API, is such a platform. "win32 is the stable Linux
userland ABI" is the observation that it might actually make sense for video
game developers aiming for Linux to target the Windows API (which doesn't
change) and rely on some translation layer to manage the Linux part (which
always changes).

Today there are all these old `.exe` files lying around that are increasingly
hard to run. Even on Windows itself, there's a
[32-bit Windows translation layer](https://en.wikipedia.org/wiki/WoW64) when
running on now standard x86-64 hardware. On non-Windows the best tool is Wine,
but it requires x86 hardware. On a recent non-x86 Mac
[their x86 emulator](https://en.wikipedia.org/wiki/Rosetta_(software)) dropped
32-bit support, so even Wine isn't sufficient there.

So my idea is this: what if you treated a win32 executable in the same way you
treat a NES ROM -- as machine code for a CPU you no longer have, expecting
hardware and an OS that doesn't exist? retrowin32 is an emulator that interprets
the x86 instructions and implements the Windows OS API such that a win32
executable can run directly, without the Windows OS or an x86 beneath it.

See a
[comparison against other Windows emulator approaches (WINE, qemu, v86)](doc/comparison.md)
for how this is similar but different.

# Code layout

- `x86/` -- the x86 emulator
- `win32/` -- the win32 emulator
  - `win32/src/winapi/` -- the Windows API implementation
- `cli/` -- a command-line emulator runner
- `web/` -- a webapp that runs the emulator in a browser
  - `web/glue/` -- wasm glue for the `win32/` API
- `exe/` -- some sample Windows executables
