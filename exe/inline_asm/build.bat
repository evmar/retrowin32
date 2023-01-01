@echo off

rem /std           c++ version
rem /Fa            asm output
rem /Os            optimize for size
rem /GS-           disable stack guard code
rem /link          linker options follow
rem /subsystem     console binary
rem /nologo        don't print copyright goop

cl demo.cc /std:c++20 /Fademo.asm /Os /GS- /link /subsystem:console /nologo kernel32.lib
