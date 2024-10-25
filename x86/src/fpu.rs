//! FPU registers.

use bitflags::bitflags;

bitflags! {
    pub struct Status: u16 {
        const C3 = 1 << 14;
        const C2 = 1 << 10;
        const C1 = 1 << 9;
        const C0 = 1 << 8;
    }
}

pub struct FPU {
    /// FPU ST0 through ST7 registers.
    pub st: [f64; 8],
    /// Index of top of FPU stack; 8 when stack empty.
    pub st_top: usize,
    /// FPU status word, without st_top included.
    pub status: Status,
}

impl Default for FPU {
    fn default() -> Self {
        Self {
            st: [0.; 8],
            st_top: 8,
            status: Status::empty(),
        }
    }
}

impl FPU {
    fn exception(_msg: &str) {
        // TODO: modify state bits etc.
        // At least ignoring these may allow programs to make some progress.
        // See note in https://github.com/joncampbell123/dosbox-x/issues/94 ,
        // "I've seen DOSBox SVN bail out on perfectly good demoscene programs because
        // of [not allowing underflow]."
        // Don't log because anatyda underflows thousands of times, eek.
        // log::warn!("{}", msg);
    }

    /// Get st(0), the current top of the FPU stack.
    pub fn st0(&mut self) -> &mut f64 {
        &mut self.st[self.st_top]
    }

    pub fn push(&mut self, val: f64) {
        if self.st_top == 0 {
            Self::exception("fpu stack overflow");
            return;
        }
        self.st_top -= 1;
        self.st[self.st_top] = val;
    }

    pub fn pop(&mut self) {
        if self.st_top == 8 {
            Self::exception("fpu stack underflow");
            return;
        }
        self.st_top += 1;
    }

    /// Index in self.st for a given ST0, ST1 etc reg.
    fn st_offset(&self, reg: iced_x86::Register) -> usize {
        let ofs = match reg {
            iced_x86::Register::ST0 => 0,
            iced_x86::Register::ST1 => 1,
            iced_x86::Register::ST2 => 2,
            iced_x86::Register::ST3 => 3,
            iced_x86::Register::ST4 => 4,
            iced_x86::Register::ST5 => 5,
            iced_x86::Register::ST6 => 6,
            iced_x86::Register::ST7 => 7,
            _ => unreachable!("invalid reg {reg:?}"),
        };
        let new = self.st_top + ofs;
        if new >= 8 {
            Self::exception("fpu stack underflow");
            return 7;
        }
        new
    }

    pub fn swap(&mut self, r1: iced_x86::Register, r2: iced_x86::Register) {
        let o1 = self.st_offset(r1);
        let o2 = self.st_offset(r2);
        self.st.swap(o1, o2);
    }

    pub fn get(&mut self, reg: iced_x86::Register) -> &mut f64 {
        &mut self.st[self.st_offset(reg)]
    }

    pub fn status(&self) -> u16 {
        // Our status register impl doesn't include st_top so include it here.
        let mut status = self.status.bits();
        status |= (self.st_top as u16 & 0b111) << 11;
        status
    }
}
