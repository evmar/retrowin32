#include "util.h"

HANDLE hStdout;

void print(std::string_view sv) {
  WriteFile(hStdout, sv.data(), sv.size(), nullptr, nullptr);
}

void print(uint32_t x) {
  char buf[9];
  size_t i = sizeof(buf);
  buf[--i] = 0;
  if (x == 0) {
    buf[--i] = '0';
  } else {
    for (; x > 0; x >>= 4) {
      auto nybble = (char)(x & 0xf);
      if (nybble < 0xa) {
        buf[--i] = '0' + nybble;
      } else {
        buf[--i] = 'a' + (nybble - 0xa);
      }
    }
  }
  print(std::string_view(&buf[i], sizeof(buf) - i - 1));
}

void printv(const char *fmt...) {
  va_list args;
  va_start(args, fmt);
  size_t start = 0, end;
  for (end = start; fmt[end]; end++) {
    if (fmt[end] == '%') {
      if (end > start) {
        WriteFile(hStdout, &fmt[start], end - start, nullptr, nullptr);
      }
      auto n = static_cast<uint32_t>(va_arg(args, int));
      print(n);
      start = end + 1;
    }
  }
  if (end > start) {
    WriteFile(hStdout, &fmt[start], end - start, nullptr, nullptr);
  }
  va_end(args);
}

void print_flags(uint32_t flags) {
  if ((flags & 1) != 0)
    print(" CF");
  if ((flags & (1 << 6)) != 0)
    print(" ZF");
  if ((flags & (1 << 7)) != 0)
    print(" SF");
  if ((flags & (1 << 10)) != 0)
    print(" DF");
  if ((flags & (1 << 11)) != 0)
    print(" OF");
}
