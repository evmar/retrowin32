#include "util.h"

void math_tests();
void fpu_tests();

extern "C" int mainCRTStartup() {
  hStdout = GetStdHandle(STD_OUTPUT_HANDLE);
  math_tests();
  fpu_tests();
  return 0;
}
