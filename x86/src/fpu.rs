//! FPU registers.

use bitflags::bitflags;

bitflags! {
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Status: u16 {
        const C3 = 1 << 14;
        const C2 = 1 << 10;
        const C1 = 1 << 9;
        const C0 = 1 << 8;
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FPU {
    /// FPU ST0 through ST7 registers.
    pub st: [f64; 8],
    /// Index of top of FPU stack; 8 when stack empty.
    pub st_top: usize,
    /// FPU status word (TODO fold st_top in here?)
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
    /// Get st(0), the current top of the FPU stack.
    pub fn st0(&mut self) -> &mut f64 {
        &mut self.st[self.st_top]
    }

    pub fn push(&mut self, val: f64) {
        self.st_top -= 1;
        self.st[self.st_top] = val;
    }

    pub fn pop(&mut self) {
        if self.st_top == 8 {
            panic!("fpu pop of empty stack");
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
        self.st_top + ofs
    }

    pub fn swap(&mut self, r1: iced_x86::Register, r2: iced_x86::Register) {
        let o1 = self.st_offset(r1);
        let o2 = self.st_offset(r2);
        self.st.swap(o1, o2);
    }

    pub fn get(&mut self, reg: iced_x86::Register) -> &mut f64 {
        &mut self.st[self.st_offset(reg)]
    }
}
