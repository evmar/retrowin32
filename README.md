# retrowin32

retrowin32 is a still-early Windows emulator for the web (and other non-Windows
platforms).

Take a win32 `.exe` file and run it in a web browser or a Mac.

See [some demos](https://evmar.github.io/retrowin32/).

## Status

Can run a few programs, including console, win32 GUI, and DirectX. Likely will
fail if given a program I haven't worked on yet.

- 2024/03: runs some of Minesweeper and more demoscene programs.
- 2023/10: preliminary support for using
  [Unicorn](https://www.unicorn-engine.org/) (basically QEMU) for x86 emulation.
- 2023/08:
  [new blog post](https://neugierig.org/software/blog/2023/08/x86-x64-aarch64.html)
  about new Rosetta-based emulation support. retrowin32 now can run on web,
  native with CPU emulation, or Rosetta x86-64. (Native x86-64 ought to be
  relatively easy to add too.)
- 2023/04: "monolife" demo now can sorta render first scene.
- 2023/02: can now run natively via SDL.
  [See blog post](https://neugierig.org/software/blog/2023/02/retrowin32-progress.html).
- 2022/10: released. Barely runs a few console programs and one DirectDraw
  program.
  [See blog post](https://neugierig.org/software/blog/2022/10/retrowin32.html).

See
[a list of blog posts](https://neugierig.org/software/blog/2023/09/retrowin32.html)
for more detailed updates.

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
but it still needs x86 hardware (or a CPU emulator).

So my idea is this: what if you treated a win32 executable in the same way you
treat a NES ROM -- as machine code for a CPU you no longer have, expecting
hardware and an OS that doesn't exist? retrowin32 is an emulator that interprets
the x86 instructions and implements the Windows OS API such that a win32
executable can run directly, without the Windows OS or an x86 beneath it.

See a
[comparison against other Windows emulator approaches (WINE, qemu, v86)](doc/comparison.md)
for how this is similar but different.

## Running it

See [HACKING.md](HACKING.md) to get started on the code.

## Support this project

If you like this project please consider to give a GitHub star, it help this project to be recommended and get more contributions.
