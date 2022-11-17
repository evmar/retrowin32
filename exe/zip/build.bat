@echo off

rem /std           c++ version
rem /Os            optimize for size
rem /GS-           disable stack guard code
rem /link          linker options follow
rem /subsystem     console binary
rem /nologo        don't print copyright goop

cl zip.cc miniz.c /std:c++20 /EHsc /Os /link /subsystem:console /nologo kernel32.lib
