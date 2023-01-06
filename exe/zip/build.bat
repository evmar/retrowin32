@echo off

rem /std           c++ version
rem /EHsc          allow exceptions
rem /Os            optimize for size
rem /GS-           disable security checks
rem /link          linker options follow
rem /dynamicbase   ASLR
rem /subsystem     console binary
rem /nologo        don't print copyright goop
rem /debug         debug info
rem /opt:ref       enable optimizations despite debug info

cl zip.cc miniz.c /std:c++20 /EHsc /Os /GS- /link /dynamicbase:no /debug /opt:ref /subsystem:console /nologo kernel32.lib
