# retrowin32

A not-yet-working Windows emulator for the web (and maybe non-web?).

## Status

Doesn't work at all yet, but I'm still tinkering!

Come back later for actual notes on how to run it etc.

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
hard to run. Even modern 64-bit Windows itself has a
[32-bit Windows translation layer](https://en.wikipedia.org/wiki/WoW64). On
non-Windows the best tool is Wine, but it requires x86 hardware. On a recent
non-x86 Mac
[even their x86 emulator dropped 32-bit support](<https://en.wikipedia.org/wiki/Rosetta_(software)>).

So my idea is this: what if you treated a win32 executable in the same way you
treat a NES ROM -- as machine code for a CPU you no longer have, expecting
hardware and an OS that doesn't exist?

See a [comparison against similar ideas (WINE, qemu, v86)](doc/comparison.md)
and why this is different.

# Code layout

- `win32/` -- the x86/win32 emulator
- `cli/` -- a command-line emulator runner
- `wasm/` -- a wrapper that exposes the `win32/` API
- `web/` -- a webapp that uses the Wasm bundle to single-step/debug the emulator
  in a browser
