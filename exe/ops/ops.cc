#include <stdint.h>
#include <string_view>
#include <windows.h>

namespace {

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

void printf(const char *fmt...) {
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

void add(uint8_t x, uint8_t y) {
  printf("add % % => ", x, y);
  clear_flags();
  asm("addb %[y],%[x]"
      ""
      : [x] "+g"(x)
      : [y] "g"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_add() {
  add(3, 5);
  add(3, (uint8_t)(-3));
  add(3, (uint8_t)(-5));
}

void adc(uint8_t x, uint8_t y) {
  printf("adc (CF=1) % % => ", x, y);
  clear_flags();
  asm("stc\n"
      "adcb %[y],%[x]"
      : [x] "+g"(x)
      : [y] "g"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_adc() {
  adc(0xFF, 0);
  adc(0xFF, 1);
  adc(0xFF, 0xFE);
  adc(0xFF, 0xFF);
}

void sbb(uint8_t x, uint8_t y) {
  printf("sbb (CF=1) % % => ", x, y);
  clear_flags();
  asm("stc\n"
      "sbbb %[y],%[x]"
      : [x] "+g"(x)
      : [y] "g"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_sbb() {
  sbb(0, 0);
  sbb(0, 1);
  sbb(0, 0xFE);
  sbb(0, 0xFF);
}

void shr(uint8_t x, uint8_t y) {
  printf("shr % % => ", x, y);
  clear_flags();
  asm("shrb %[y],%[x]"
      ""
      : [x] "+g"(x)
      : [y] "c"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_shr() {
  shr(3, 0);
  shr(3, 1);
  shr(3, 2);
  shr(0x80, 1);
  shr(0x80, 2);
  shr(0x81, 1);
  shr(0x81, 2);
}

void sar(uint8_t x, uint8_t y) {
  printf("sar % % => ", x, y);
  clear_flags();
  asm("sarb %[y],%[x]"
      ""
      : [x] "+g"(x)
      : [y] "c"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_sar() {
  sar(3, 1);
  sar(3, 2);
  sar(0x80, 1);
  sar(0x80, 2);
  sar(0x81, 1);
  sar(0x81, 2);
  sar(0x82, 1);
  sar(0x82, 2);
}

void shl(uint8_t x, uint8_t y) {
  printf("sar % % => ", x, y);
  clear_flags();
  asm("shlb %[y],%[x]"
      ""
      : [x] "+g"(x)
      : [y] "c"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_shl() {
  shl(3, 0);
  shl(3, 1);
  shl(3, 2);
  shl(0x80, 1);
  shl(0x80, 2);
  shl(0xD1, 1);
  shl(0xD1, 2);
  shl(0xE2, 1);
  shl(0xE2, 2);
}

void rol(uint8_t x, uint8_t y) {
  printf("rol % % => ", x, y);
  clear_flags();
  asm("rolb %[y],%[x]"
      ""
      : [x] "+g"(x)
      : [y] "c"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_rol() {
  rol(0x80, 0);
  rol(0x80, 1);
  rol(0x80, 2);
  rol(0xC0, 1);
  rol(0xC0, 2);
  rol(0xA0, 1);
  rol(0xA0, 2);
  rol(0x6, 1);
  rol(0x60, 2);
}

void ror(uint8_t x, uint8_t y) {
  printf("ror % % => ", x, y);
  clear_flags();
  asm("rorb %[y],%[x]"
      ""
      : [x] "+g"(x)
      : [y] "c"(y)
      : "cc");
  auto flags = get_flags();
  print(x);
  print_flags(flags);
  print("\n");
}

void test_ror() {
  ror(0x01, 0);
  ror(0x01, 1);
  ror(0x01, 2);
  ror(0x03, 1);
  ror(0x03, 2);
  ror(0x02, 1);
  ror(0x02, 2);
  ror(0x06, 1);
  ror(0x06, 2);
}
} // namespace

extern "C" int mainCRTStartup() {
  hStdout = GetStdHandle(STD_OUTPUT_HANDLE);
  test_add();
  test_adc();
  test_sbb();
  test_shr();
  test_sar();
  test_shl();
  test_rol();
  test_ror();
  return 0;
}
