#include <stdio.h>

int main(void) {
  int result;
  __asm {
    mov eax,4
    add eax,7
    mov result,eax
  }
  printf("asm demo: %d\n", result);
  return 0;
}
