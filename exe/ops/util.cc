#include "util.h"

HANDLE hStdout;

void print(std::string_view sv) {
  WriteFile(hStdout, sv.data(), sv.size(), nullptr, nullptr);
}

void print(uint32_t x) {
  char buf[9];
  size_t i = sizeof(buf);
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
  print(std::string_view(&buf[i], sizeof(buf) - i));
}

void print(double f) {
  // To print a float, we multiply by 1000 and print as decimal.

  // TODO: retrowin32 doesn't support the fpu op that "<" needs.
  // bool neg = f < 0;
  // if (f < 0)
  //   f = -f;
  uint32_t x = (uint32_t)(f * 1000.0);
  char buf[64];
  size_t i = sizeof(buf);
  if (x == 0) {
    buf[--i] = '0';
  } else {
    while (x > 0 && i > (sizeof(buf) - 5)) {
      buf[--i] = '0' + (x % 10);
      x /= 10;
      if (i == sizeof(buf) - 3) {
        buf[--i] = '.';
        if (x == 0) {
          buf[--i] = '0';
        }
      }
    }
  }
  // if (neg) {
  //   buf[--i] = '-';
  // }
  print(std::string_view(&buf[i], sizeof(buf) - i));
}

void printv(const char *fmt...) {
  va_list args;
  va_start(args, fmt);
  size_t start = 0, end;
  for (end = start; fmt[end]; end++) {
    if (fmt[end] == '%') {
      if (end > start) {
        print(std::string_view(&fmt[start], end - start));
      }
      end++;
      if (fmt[end] == 'x') {
        auto n = static_cast<uint32_t>(va_arg(args, int));
        print(n);
      } else if (fmt[end] == 'f') {
        auto n = va_arg(args, double);
        print(n);
      } else {
        print("invalid format specifier\n");
        __builtin_trap();
      }
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
