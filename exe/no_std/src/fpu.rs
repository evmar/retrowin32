use crate::{fmt::Fmt, print::print};
use core::arch::asm;

macro_rules! ft {
    ($name:literal, $asm:tt) => {
        ft!($name, 1, $asm)
    };
    ($name:literal, $depth:expr, $asm:tt) => {
        $asm;
        let mut fmt = Fmt::new();
        fmt.str($name).str(": ");

        for i in 0..$depth {
            let mut val = 0f64;
            asm!("fstp qword ptr [{val}]", val = in(reg) &mut val);
            if i > 0 {
                fmt.char(b' ');
            }
            fmt.f64(val);
        }

        print(fmt.char(b'\n').buf());
    };
}

#[inline(never)]
unsafe fn fld_constants() {
    ft!("fld1", (asm!("fld1")));
    ft!("fldz", (asm!("fldz")));
    ft!("fldpi", (asm!("fldpi")));
    ft!("fldl2e", (asm!("fldl2e")));
}

#[inline(never)]
unsafe fn fld() {
    ft!("fld st(i)", 3, (asm!("fldpi", "fld1", "fld st(1)")));
    ft!("fld f32", {
        let x = 123.456f32;
        asm!("fld dword ptr [{x}]", x = in(reg) &x);
    });
    ft!("fld f64", {
        let x = 123.456f64;
        asm!("fld qword ptr [{x}]", x = in(reg) &x);
    });
}

#[inline(never)]
unsafe fn fild() {
    ft!("fild m64", {
        let x = 1234i64;
        asm!("fild qword ptr [{x}]", x = in(reg) &x);
    });
    ft!("fild m32", {
        let x = 1234i32;
        asm!("fild dword ptr [{x}]", x = in(reg) &x);
    });
    ft!("fild m16", {
        let x = 1234i16;
        asm!("fild word ptr [{x}]", x = in(reg) &x);
    });
    ft!("fild m16 neg", {
        let x = -1234i16;
        asm!("fild word ptr [{x}]", x = in(reg) &x);
    });
}

#[inline(never)]
unsafe fn trig() {
    ft!("fsincos", 2, { asm!("fldl2e", "fsincos") });
    ft!("fpatan", { asm!("fldl2e", "fldpi", "fpatan") });
}

#[inline(never)]
unsafe fn exp() {
    for x in [-0.9, -0.3, 0.0, 0.1, 0.4] {
        ft!("f2xm1", {
            asm!("fld qword ptr [{x}]", "f2xm1", x = in(reg) &x);
        });
    }
    for x in [-1.2, -0.3, 0.0, 0.5, 1.4, 2.4] {
        ft!("fscale", 2, {
            asm!("fld qword ptr [{x}]", "fldpi", "fscale", x = in(reg) &x);
        });
    }
}

#[inline(never)]
unsafe fn fprem() {
    ft!("fprem", 2, {
        asm!("fldpi", "fldl2e", "fprem");
    });
    ft!("fprem", 2, {
        asm!("fldl2e", "fldpi", "fprem");
    });

    ft!("fprem", 2, {
        asm!("fldl2e", "fchs", "fldpi", "fprem");
    });

    ft!("fprem", 2, {
        asm!("fldl2e", "fldpi", "fchs", "fprem");
    });
}

pub fn test() {
    unsafe {
        fld_constants();
        fld();
        fild();
        trig();
        exp();
        fprem();
    }
}
