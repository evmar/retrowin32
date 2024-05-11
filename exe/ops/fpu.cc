#include "util.h"

extern "C" int _fltused;
extern "C" int _fltused = 0;

namespace {

void print_fpu_stack(int depth) {
  double stack[8] = {};
  for (int i = 0; i < depth; i++) {
    __asm {
      mov eax, i
      fstp qword ptr [stack + eax*8]
    }
  }
  for (int i = depth; i > 0; i--) {
    printv(" %f", stack[i - 1]);
  }
  print("\n");
}

void test_fld_constants() {
  __asm {
      fld1
      fldz
      fldpi
      fldl2e
  }
  printv("fld 1,0,pi,l2e =>");
  print_fpu_stack(4);
}

void test_fld() {
  float f32 = 1.1;
  double f64 = 2.2;
  __asm {
    fld f32
    fld f64
    fld st(1)
  }
  printv("fld =>");
  print_fpu_stack(3);
}

void test_fild() {
  uint16_t i16 = 4321;
  uint32_t i32 = 44321;
  uint64_t i64 = 454321;
  __asm {
    fild i16
    fild i32
    fild i64
  }
  printv("fild =>");
  print_fpu_stack(3);
}

void test_fst() {
  float f32;
  double f64;
  __asm {
    fldpi
    fstp f32
    fldpi
    fstp f64
  }
  printv("fst => %f %f\n", f32, f64);
}

void test_fist() {
  uint16_t i16;
  uint32_t i32;
  uint64_t i64;
  __asm {
    fldpi
    fistp word ptr [i16]
    fldpi
    fistp dword ptr [i32]
    fldpi
    fistp qword ptr [i64]
  }
  printv("fist => %x %x %x\n", i16, i32, i64);
}

void test_fchs() {
  __asm {
    fldpi
    fchs

    fld st(0)  ; dup
    fchs
  }
  print("fchs =>");
  print_fpu_stack(2);
}

void test_fabs() {
  __asm {
    fldpi
    fchs

    fld st(0)  ; dup
    fabs
  }
  print("fabs =>");
  print_fpu_stack(2);
}

void test_trig() {
  __asm {
    fld1
    fsin

    fld1
    fcos

    fld1
    fsincos

    fldpi
    fldl2e
    fpatan
  }
  print("trig =>");
  print_fpu_stack(5);
}

void test_fadd_st() {
  __asm {
    fldpi
    fldl2e
    fadd st(0), st(1)

    fldpi
    fldl2e
    fadd st(1), st(0)

    fldpi
    fldl2e
    faddp
  }
  print("fadd st =>");
  print_fpu_stack(5);
}

void test_fadd_mem() {
  float f32 = 43.21;
  double f64 = 432.1;
  __asm {
    fldpi
    fadd f32

    fldpi
    fadd f64
  }
  print("fadd mem =>");
  print_fpu_stack(2);
}

void test_fiadd() {
  uint16_t i16 = 43;
  uint32_t i32 = 44;
  __asm {
    fldpi
    fiadd i16

    fldpi
    fiadd i32
  }
  print("fiadd =>");
  print_fpu_stack(2);
}

void test_fsub_mem() {
  float f32 = 43.21;
  double f64 = 432.1;
  __asm {
    fldpi
    fsub f32
    fsub f64
  }
  print("fsub mem =>");
  print_fpu_stack(1);
}

} // anonymous namespace

void fpu_tests() {
  test_fld_constants();
  test_fld();
  test_fild();
  test_fst();
  test_fist();
  test_fchs();
  test_fabs();
  test_trig();
  test_fadd_st();
  test_fadd_mem();
  test_fiadd();
  test_fsub_mem();
}
