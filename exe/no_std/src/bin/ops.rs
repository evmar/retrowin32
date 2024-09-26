#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use no_std::fmt::Fmt;
use no_std::print::print;

#[inline(never)]
fn fmt_flags(buf: &mut Fmt, reg: u32) {
    for (bit, name) in [
        (0, "CF"),
        (2, "PF"),
        (6, "ZF"),
        (7, "SF"),
        (10, "DF"),
        (11, "OF"),
    ] {
        if (reg >> bit) & 1 == 1 {
            buf.char(b' ');
            buf.str(name);
        }
    }
}

#[inline(never)]
#[allow(asm_sub_register)] // TODO: this is just because rust-analyzer is confused
fn flags_test() {
    let mut val = 0u8;
    let mut flags: u32 = 0;
    unsafe {
        core::arch::asm!(
            "push 0",
            "popfd",
            "cmp {x}, {y}",
            "pushfd",
            "pop dword ptr [{flags}]",
            "setle [{val}]",
            x = in(reg) 1u32,
            y = in(reg) 1u32,
            flags = in(reg) &mut flags,
            val = in(reg) &mut val,
        );
    }

    let mut fmt = Fmt::new();
    fmt.str("flags:");
    fmt_flags(&mut fmt, flags);
    fmt.char(b'\n');
    print(fmt.buf());

    if val != 0 {
        print(b"setle: true\n");
    } else {
        print(b"setle: false\n");
    }
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    flags_test();
    no_std::fpu::test();
}
