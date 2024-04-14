#include "util.h"

void math_tests();

extern "C" int mainCRTStartup() {
  hStdout = GetStdHandle(STD_OUTPUT_HANDLE);
  math_tests();
  return 0;
}
