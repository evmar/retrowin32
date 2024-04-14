#pragma once

#include <stdint.h>
#include <string_view>
#include <windows.h>

extern HANDLE hStdout;

void print(std::string_view sv);
void print(uint32_t x);
void printv(const char *fmt...);
void print_flags(uint32_t flags);

inline void clear_flags() {
  asm inline("pushl $0\n"
             "popfd"
             :
             :
             : "cc" /* clobbers flags*/);
}

inline uint32_t get_flags() {
  uint32_t flags = 0;
  asm inline("pushfd\n"
             "popl %[flags]"
             : [flags] "=r"(flags));
  return flags;
}
