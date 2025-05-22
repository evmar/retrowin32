use exe::println;

fn bs_test() {
    let values = [0u32, 0b1, 0b110, 0b100100];
    for value in values {
        let mut bsf: u32;
        let mut bsr: u32;
        unsafe {
            core::arch::asm!(
                "bsf {bsf}, {value}",
                "bsr {bsr}, {value}",
                value = in(reg) value,
                bsf = out(reg) bsf,
                bsr = out(reg) bsr,
            );
        }
        println!("bsf {value:x}: {bsf:x}");
        println!("bsr {value:x}: {bsr:x}");
    }
}

fn tzcnt_test() {
    let values = [0u32, 0b1, 0b11000, 0b1000_0000_0000_0000];
    for value in values {
        let mut tzcnt: u32;
        unsafe {
            core::arch::asm!(
                "tzcnt {tzcnt}, {value}",
                value = in(reg) value,
                tzcnt = out(reg) tzcnt,
            );
        }
        println!("tzcnt {value:x}: {tzcnt:x}");
    }
}

pub fn test() {
    bs_test();
    tzcnt_test();
}
