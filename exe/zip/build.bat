@echo off

cl zip.cc miniz.c /Brepro /std:c++20 /EHsc /Os /GS- /link /dynamicbase:no /debug /opt:ref /subsystem:console /nologo kernel32.lib
