@echo off

cl winapi.cc /Brepro /std:c++20 /Fademo.asm /Os /GS- /link /subsystem:console /nologo /nodefaultlib kernel32.lib
