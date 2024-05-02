use bitflags::bitflags;
use iced_x86::Register::*;

bitflags! {
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Flags: u32 {
        /// carry
        const CF = 1 << 0;
        /// zero
        const ZF = 1 << 6;
        /// sign
        const SF = 1 << 7;
        /// direction
        const DF = 1 << 10;
        /// overflow
        const OF = 1 << 11;
        /// cpuid
        const ID = 1 << 21;
    }
}

#[repr(C)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Registers {
    // Warning: get32 assumes the registers start with eax and are in a particular order.
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,

    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,

    pub eip: u32,

    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
    // TODO: segment registers are actually 16-bit indexes into the GDT/LDT,
    // but for our purposes all we ever care about is making FS-relative accesses point
    // at the Windows TEB.
    /// Address that FS-relative accesses point to.
    pub fs_addr: u32,

    /// MMX registers.
    // TODO: officially these should alias the FPU registers(!).
    pub mm: [u64; 8],
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            eip: 0,
            cs: 0,
            ds: 0,
            es: 0,
            fs: 0,
            gs: 0,
            ss: 0,
            fs_addr: 0,
            mm: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

fn r16_to_32(r16: iced_x86::Register) -> iced_x86::Register {
    unsafe { std::mem::transmute((r16 as u8 - AX as u8) + EAX as u8) }
}

fn r8l_to_32(r8: iced_x86::Register) -> iced_x86::Register {
    unsafe { std::mem::transmute((r8 as u8 - AL as u8) + EAX as u8) }
}

fn r8h_to_32(r8: iced_x86::Register) -> iced_x86::Register {
    unsafe { std::mem::transmute((r8 as u8 - AH as u8) + EAX as u8) }
}

impl Registers {
    pub fn get32_mut(&mut self, reg: iced_x86::Register) -> &mut u32 {
        // XXX move comments from get32 here and rename once everything moved.
        let idx = reg as usize - iced_x86::Register::EAX as usize;
        if idx >= 8 {
            unreachable!("{reg:?}");
        }
        unsafe { &mut *(self as *mut Registers as *mut u32).add(idx) }
    }

    pub fn get32(&self, reg: iced_x86::Register) -> u32 {
        // This function is hot in profiles, and even if we write
        // a match statement that maps register N to struct offset 4*N,
        // llvm doesn't seem to optimize it to the obvious math.
        // I tried the equivalent in C++ and that didn't optimize either.
        // So instead here's some unsafe hackery.
        let idx = reg as usize - iced_x86::Register::EAX as usize;
        if idx >= 8 {
            unreachable!("{reg:?}");
        }
        // XXX this assumes register order matches between our struct and iced_x86.
        unsafe { *(self as *const Registers as *const u32).add(idx) }
    }

    pub fn get16(&self, reg: iced_x86::Register) -> u16 {
        match reg {
            AX | CX | DX | BX | SP | BP | SI | DI => self.get32(r16_to_32(reg)) as u16,

            ES => self.es,
            CS => self.cs,
            SS => self.ss,
            DS => self.ds,
            FS => self.fs,
            GS => self.gs,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get16_mut(&mut self, reg: iced_x86::Register) -> &mut u16 {
        unsafe {
            match reg {
                AX | CX | DX | BX | SP | BP | SI | DI => {
                    &mut *(self.get32_mut(r16_to_32(reg)) as *mut u32 as *mut u16)
                }
                ES => &mut self.es,
                CS => &mut self.cs,
                SS => &mut self.ss,
                DS => &mut self.ds,
                FS => &mut self.fs,
                GS => &mut self.gs,
                _ => unreachable!("{reg:?}"),
            }
        }
    }

    pub fn get8(&self, reg: iced_x86::Register) -> u8 {
        match reg {
            AL | CL | DL | BL => self.get32(r8l_to_32(reg)) as u8,
            AH | CH | DH | BH => (self.get32(r8h_to_32(reg)) >> 8) as u8,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get8_mut(&mut self, reg: iced_x86::Register) -> &mut u8 {
        unsafe {
            match reg {
                AL | CL | DL | BL => &mut *(self.get32_mut(r8l_to_32(reg)) as *mut u32 as *mut u8),
                AH | CH | DH | BH => {
                    &mut *(self.get32_mut(r8h_to_32(reg)) as *mut u32 as *mut u8).add(1)
                }
                _ => unreachable!("{reg:?}"),
            }
        }
    }

    pub fn set32(&mut self, reg: iced_x86::Register, value: u32) {
        let idx = reg as usize - iced_x86::Register::EAX as usize;
        if idx >= 8 {
            unreachable!("{reg:?}");
        }
        // XXX this assumes register order matches between our struct and iced_x86.
        unsafe {
            *(self as *mut Registers as *mut u32).add(idx) = value;
        }
    }

    pub fn set16(&mut self, reg: iced_x86::Register, value: u16) {
        match reg {
            AX | CX | DX | BX | SI | DI | BP => {
                let r32 = r16_to_32(reg);
                self.set32(r32, (self.get32(r32) & 0xFFFF_0000) | value as u32);
            }

            ES => self.es = value,
            CS => self.cs = value,
            SS => self.ss = value,
            DS => self.ds = value,
            FS => self.fs = value,
            GS => self.gs = value,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn set8(&mut self, reg: iced_x86::Register, value: u8) {
        match reg {
            AL | CL | DL | BL => {
                let r32 = self.get32_mut(r8l_to_32(reg));
                *r32 = (*r32 & 0xFFFF_FF00) | value as u32;
            }

            AH | CH | DH | BH => {
                let r32 = self.get32_mut(r8h_to_32(reg));
                *r32 = (*r32 & 0xFFFF_00FF) | ((value as u32) << 8);
            }
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get64(&self, reg: iced_x86::Register) -> u64 {
        match reg {
            MM0 => self.mm[0],
            MM1 => self.mm[1],
            MM2 => self.mm[2],
            MM3 => self.mm[3],
            MM4 => self.mm[4],
            MM5 => self.mm[5],
            MM6 => self.mm[6],
            MM7 => self.mm[7],
            _ => unimplemented!("{:?}", reg),
        }
    }
    pub fn set64(&mut self, reg: iced_x86::Register, value: u64) {
        match reg {
            MM0 => self.mm[0] = value,
            MM1 => self.mm[1] = value,
            MM2 => self.mm[2] = value,
            MM3 => self.mm[3] = value,
            MM4 => self.mm[4] = value,
            MM5 => self.mm[5] = value,
            MM6 => self.mm[6] = value,
            MM7 => self.mm[7] = value,
            _ => unimplemented!("{:?}", reg),
        }
    }
}
