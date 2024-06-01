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
fn test() {
    let mut val = 0u8;
    let mut flags: u32 = 0;
    unsafe {
        core::arch::asm!(
            "push 0",
            "popfd",
            //"cmp {x}, {y}",
            "pushfd",
            "pop dword ptr [{flags}]",
            "setle [{out}]",
            //x = in(reg) 1u32,
            //y = in(reg) 1u32,
            flags = in(reg) &mut flags,
            out = in(reg) &mut val,
        );
    }

    print(
        Fmt::new()
            .bin_u8((flags >> 8) as u8)
            .char(b' ')
            .bin_u8(flags as u8)
            .char(b'\n')
            .buf(),
    );

    print(Fmt::new().hex_u32(flags).char(b'\n').buf());

    let mut fmt = Fmt::new();
    fmt.str("flags:");
    fmt_flags(&mut fmt, flags);
    fmt.char(b'\n');
    print(fmt.buf());

    if val != 0 {
        print(b"true\n");
    } else {
        print(b"false\n");
    }
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    print(b"Hxello, world!\n");
    test();
}

#[cfg(not(test))]
#[panic_handler]
unsafe fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    print(b"panicked");
    windows_sys::Win32::System::Threading::ExitProcess(1);
}
