#include <stdio.h>
#include <stdint.h>

void print_flags(uint16_t flags) {
  if ((flags & 1) != 0)
    printf(" CF");
  if ((flags & (1<<6)) != 0)
    printf(" ZF");
  if ((flags & (1<<7)) != 0)
    printf(" SF");
  if ((flags & (1<<10)) != 0)
    printf(" DF");
  if ((flags & (1<<11)) != 0)
    printf(" OF");
}

#define asm_start(desc) { \
  printf(desc); \
  uint32_t result; \
  uint16_t flags = 0; \
  __asm { \
    __asm push flags \
    __asm popf \

#define asm_end() \
    __asm mov result,eax \
    __asm pushf \
    __asm pop flags \
  } \
  printf(" => %x", result); \
  print_flags(flags); \
  printf("\n"); \
}

void add() {
#define add(x,y) \
  asm_start("add " #x "," #y) \
    __asm mov eax,x \
    __asm add eax,y \
  asm_end();
  add(3, 5);
  add(3, -3);
  add(3, -5);
#undef add
}

void shr() {
#define shr(x,y) \
  asm_start("shr " #x "," #y) \
    __asm mov eax,x \
    __asm shr eax,y \
  asm_end();
  shr(3, 0);
  shr(3, 1);
  shr(3, 2);
  shr(0x80000000, 1);
  shr(0x80000000, 2);
  shr(0x80000001, 1);
  shr(0x80000001, 2);
#undef shr
}

void sar() {
#define sar(x,y) \
  asm_start("sar " #x "," #y) \
    __asm mov eax,x \
    __asm sar eax,y \
  asm_end();
  sar(3, 0);
  sar(3, 1);
  sar(3, 2);
  sar(0x80000000, 1);
  sar(0x80000000, 2);
  sar(0x80000001, 1);
  sar(0x80000001, 2);
  sar(0x80000002, 1);
  sar(0x80000002, 2);
#undef sar
}

void shl() {
#define shl(x,y) \
  asm_start("shl " #x "," #y) \
    __asm mov eax,x \
    __asm shl eax,y \
  asm_end();
  shl(3, 0);
  shl(3, 1);
  shl(3, 2);
  shl(0x80000000, 1);
  shl(0x80000000, 2);
  shl(0xD0000001, 1);
  shl(0xD0000001, 2);
  shl(0xE0000002, 1);
  shl(0xE0000002, 2);
#undef shl
}

int main(void) {
  add();
  shr();
  sar();
  shl();
  return 0;
}
