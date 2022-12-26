# Design notes

## Endianness

x86 is observably little-endian, so we must emulate memory in a little-endian manner.

In principle the outer emulator code could be written to work on a big-endian system.
Initially I had abstractions in place to keep the x86 numbers and outer number separate
when appropriate -- see the commit that added this documentation to find it -- but
realistically I'm not going to run this on a non-little-endian system and it's just
more work.
