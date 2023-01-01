@echo off

rem /std           c++ version
rem /Fa            asm output
rem /Os            optimize for size
rem /GS-           disable stack guard code
rem /link          linker options follow
rem /subsystem     console binary
rem /nologo        don't print copyright goop
rem /nodefaultlib  disable C runtime

cl winapi.cc /std:c++20 /Fademo.asm /Os /GS- /link /subsystem:console /nologo /nodefaultlib kernel32.lib
