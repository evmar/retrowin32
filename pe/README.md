# PE parser

This crate traverses Windows Portable Executable (PE) files, aka .exe and .dll.

It doesn't interact with any of the emulation machinery, it just accepts byte
buffers and returns different views on to them.
