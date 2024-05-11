#pragma once

#include <stdint.h>
#include <string_view>
#include <windows.h>

extern HANDLE hStdout;

void print(std::string_view sv);
void print(uint32_t x);
void printv(const char *fmt...);
void print_flags(uint32_t flags);

#define clear_flags() __asm push 0 __asm popfd
#define get_flags()                                                            \
  uint32_t flags = 0;                                                          \
  __asm pushfd __asm pop flags;
