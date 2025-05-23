use crate::print_flags;
use exe::{print, println};

fn add_test() {
    for carry in [false, true] {
        for x in [0u8, 1, 2, 0x7e, 0x7f, 0x80, 0x81, 0xfe, 0xff] {
            for y in [0u8, 1, 2, 0x7e, 0x7f, 0x80, 0x81, 0xfe, 0xff] {
                let mut out = x;
                let flags_in = if carry { 1u32 } else { 0 };
                let mut flags = 0u32;
                unsafe {
                    core::arch::asm!(
                        "push {flags_in}",
                        "popfd",
                        "adc {out}, {y}",
                        "pushfd",
                        "pop dword ptr [{flags}]",
                        flags_in = in(reg) flags_in,
                        out = inout(reg_byte) out,
                        y = in(reg_byte) y,
                        flags = in(reg) &mut flags,
                    );
                }
                print!("adc {x:#02x} {y:#02x} {carry}: {out:#02x}");
                print_flags(flags);
                println!();
            }
        }
    }
}

fn sub_test() {
    for carry in [false, true] {
        for x in [0u8, 1, 2, 0x7e, 0x7f, 0x80, 0x81, 0xfe, 0xff] {
            for y in [0u8, 1, 2, 0x7e, 0x7f, 0x80, 0x81, 0xfe, 0xff] {
                let mut out = x;
                let flags_in = if carry { 1u32 } else { 0 };
                let mut flags = 0u32;
                unsafe {
                    core::arch::asm!(
                        "push {flags_in}",
                        "popfd",
                        "sbb {out}, {y}",
                        "pushfd",
                        "pop dword ptr [{flags}]",
                        flags_in = in(reg) flags_in,
                        out = inout(reg_byte) out,
                        y = in(reg_byte) y,
                        flags = in(reg) &mut flags,
                    );
                }
                print!("sbb {x:#02x} {y:#02x} {carry}: {out:#02x}");
                print_flags(flags);
                println!();
            }
        }
    }
}

pub fn test() {
    add_test();
    // TODO: sub_test();
}
