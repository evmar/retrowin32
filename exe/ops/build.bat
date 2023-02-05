@echo off

cl ops.cc /Brepro /std:c++20 /Os /GS- /link /subsystem:console /nologo kernel32.lib
