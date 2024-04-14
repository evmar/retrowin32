#include "util.h"

extern "C" int _fltused;
extern "C" int _fltused = 0;

namespace {

void test_fld_constants() {
  double out = 0;
  asm("fld1\n"
      "fldz\n"
      "fldpi\n"
      "fldl2e\n"
      "faddp\n"
      "faddp\n"
      "faddp\n"
      "fstpl %[out]\n"
      ""
      : [out] "=m"(out));
  printv("1+0+pi+l2e = %f\n", out);
}

} // anonymous namespace

void fpu_tests() { test_fld_constants(); }
