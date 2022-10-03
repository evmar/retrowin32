# Comparison with similar systems

Executing a Windows .exe natively requires an x86 processor and the Windows OS:

<img src='native.png' width='248' alt='Windows component stack'>

[Wine](https://www.winehq.org/) translates Windows calls into your local OS, but
still requires an x86 processor:

<img src='wine.png' width='247' alt='WINE component stack'>

x86 emulators like [qemu](https://www.qemu.org/) and [v86](https://copy.sh/v86/)
emulate an x86 processor, but at a level where it still requires you to run an
OS within the emulator:

<img src='qemu.png' width='248' alt='qemu component stack'>

This project, in contrast, aims to run a win32 executable directly, in a manner
similar to how video game emulators work: by both emulating the executable and
mapping its calls directly into local OS calls.

<img src='retrowin32.png' width='248' alt='retrowin32 component stack'>
