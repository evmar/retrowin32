#include "util.h"

extern "C" int _fltused;
extern "C" int _fltused = 0;

namespace {

void test_fld_constants() {
  double out = 0;
  __asm {
      fld1
      fldz
      fldpi
      fldl2e
      faddp
      faddp
      faddp
      fstp [out]
  }
  printv("1+0+pi+l2e = %f\n", out);
}

} // anonymous namespace

void fpu_tests() { test_fld_constants(); }
