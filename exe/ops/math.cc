#include "util.h"

namespace {

const uint32_t CPUFLAG_OF = 0x800;

void add(uint8_t x, uint8_t y) {
  printv("add %x,%x => ", x, y);
  clear_flags();
  __asm {
    mov al, y
    add x, al
  }
  get_flags();
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
  printv("adc (CF=1) %x,%x => ", x, y);
  clear_flags();
  __asm {
    stc
    mov al, y
    adc x, al
  }
  get_flags();
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
  printv("sbb (CF=1) %x,%x => ", x, y);
  clear_flags();
  __asm {
    stc
             // TODO: sbb against x directly
    mov al, x
    sbb al, y
    mov x, al
  }
  get_flags();
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
  printv("shr %x,%x => ", x, y);
  clear_flags();
  __asm {
    mov cl, y
    shr x, cl
  }
  get_flags();
  if (y != 1) {
    // Result is undefined for shift != 1.
    flags &= ~CPUFLAG_OF;
  }
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
  printv("sar %x,%x => ", x, y);
  clear_flags();
  __asm {
    mov cl, y
    sar x, cl
  }
  get_flags();
  if (y != 1) {
    // Result is undefined for shift != 1.
    flags &= ~CPUFLAG_OF;
  }
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
  printv("sar %x,%x => ", x, y);
  clear_flags();
  __asm {
    mov cl, y
    shl x, cl
  }
  get_flags();
  if (y != 1) {
    // Result is undefined for shift != 1.
    flags &= ~CPUFLAG_OF;
  }
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
  printv("rol %x,%x => ", x, y);
  clear_flags();
  __asm {
    mov cl, y
    rol x, cl
  }
  get_flags();
  if (y != 1) {
    // Result is undefined for shift != 1.
    flags &= ~CPUFLAG_OF;
  }
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
  printv("ror %x,%x => ", x, y);
  clear_flags();
  __asm {
    mov cl, y
    ror x, cl
  }
  get_flags();
  if (y != 1) {
    // Result is undefined for shift != 1.
    flags &= ~CPUFLAG_OF;
  }
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

void math_tests() {
  test_add();
  test_adc();
  test_sbb();
  test_shr();
  test_sar();
  test_shl();
  test_rol();
  test_ror();
}
