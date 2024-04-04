use bitflags::bitflags;

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
            iced_x86::Register::AX => self.eax as u16,
            iced_x86::Register::CX => self.ecx as u16,
            iced_x86::Register::DX => self.edx as u16,
            iced_x86::Register::BX => self.ebx as u16,
            iced_x86::Register::SP => self.esp as u16,
            iced_x86::Register::BP => self.ebp as u16,
            iced_x86::Register::SI => self.esi as u16,
            iced_x86::Register::DI => self.edi as u16,
            iced_x86::Register::ES => self.es,
            iced_x86::Register::CS => self.cs,
            iced_x86::Register::SS => self.ss,
            iced_x86::Register::DS => self.ds,
            iced_x86::Register::FS => self.fs,
            iced_x86::Register::GS => self.gs,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get16_mut(&mut self, reg: iced_x86::Register) -> &mut u16 {
        unsafe {
            match reg {
                iced_x86::Register::AX => &mut *(&mut self.eax as *mut u32 as *mut u16),
                iced_x86::Register::CX => &mut *(&mut self.ecx as *mut u32 as *mut u16),
                iced_x86::Register::DX => &mut *(&mut self.edx as *mut u32 as *mut u16),
                iced_x86::Register::BX => &mut *(&mut self.ebx as *mut u32 as *mut u16),
                iced_x86::Register::SP => &mut *(&mut self.esp as *mut u32 as *mut u16),
                iced_x86::Register::BP => &mut *(&mut self.ebp as *mut u32 as *mut u16),
                iced_x86::Register::SI => &mut *(&mut self.esi as *mut u32 as *mut u16),
                iced_x86::Register::DI => &mut *(&mut self.edi as *mut u32 as *mut u16),
                iced_x86::Register::ES => &mut self.es,
                iced_x86::Register::CS => &mut self.cs,
                iced_x86::Register::SS => &mut self.ss,
                iced_x86::Register::DS => &mut self.ds,
                iced_x86::Register::FS => &mut self.fs,
                iced_x86::Register::GS => &mut self.gs,
                _ => unreachable!("{reg:?}"),
            }
        }
    }

    pub fn get8(&self, reg: iced_x86::Register) -> u8 {
        match reg {
            iced_x86::Register::AL => self.eax as u8,
            iced_x86::Register::CL => self.ecx as u8,
            iced_x86::Register::DL => self.edx as u8,
            iced_x86::Register::BL => self.ebx as u8,
            iced_x86::Register::AH => (self.eax >> 8) as u8,
            iced_x86::Register::CH => (self.ecx >> 8) as u8,
            iced_x86::Register::DH => (self.edx >> 8) as u8,
            iced_x86::Register::BH => (self.ebx >> 8) as u8,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get8_mut(&mut self, reg: iced_x86::Register) -> &mut u8 {
        unsafe {
            match reg {
                iced_x86::Register::AL => &mut *(&mut self.eax as *mut u32 as *mut u8),
                iced_x86::Register::CL => &mut *(&mut self.ecx as *mut u32 as *mut u8),
                iced_x86::Register::DL => &mut *(&mut self.edx as *mut u32 as *mut u8),
                iced_x86::Register::BL => &mut *(&mut self.ebx as *mut u32 as *mut u8),
                iced_x86::Register::AH => &mut *(&mut self.eax as *mut u32 as *mut u8).add(1),
                iced_x86::Register::CH => &mut *(&mut self.ecx as *mut u32 as *mut u8).add(1),
                iced_x86::Register::DH => &mut *(&mut self.edx as *mut u32 as *mut u8).add(1),
                iced_x86::Register::BH => &mut *(&mut self.ebx as *mut u32 as *mut u8).add(1),
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
            iced_x86::Register::AX => self.eax = (self.eax & 0xFFFF_0000) | value as u32,
            iced_x86::Register::CX => self.ecx = (self.ecx & 0xFFFF_0000) | value as u32,
            iced_x86::Register::DX => self.edx = (self.edx & 0xFFFF_0000) | value as u32,
            iced_x86::Register::BX => self.ebx = (self.ebx & 0xFFFF_0000) | value as u32,
            iced_x86::Register::SI => self.esi = (self.esi & 0xFFFF_0000) | value as u32,
            iced_x86::Register::DI => self.edi = (self.edi & 0xFFFF_0000) | value as u32,
            iced_x86::Register::BP => self.ebp = (self.ebp & 0xFFFF_0000) | value as u32,
            iced_x86::Register::ES => self.es = value,
            iced_x86::Register::CS => self.cs = value,
            iced_x86::Register::SS => self.ss = value,
            iced_x86::Register::DS => self.ds = value,
            iced_x86::Register::FS => self.fs = value,
            iced_x86::Register::GS => self.gs = value,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn set8(&mut self, reg: iced_x86::Register, value: u8) {
        match reg {
            iced_x86::Register::AL => self.eax = (self.eax & 0xFFFF_FF00) | value as u32,
            iced_x86::Register::CL => self.ecx = (self.ecx & 0xFFFF_FF00) | value as u32,
            iced_x86::Register::DL => self.edx = (self.edx & 0xFFFF_FF00) | value as u32,
            iced_x86::Register::BL => self.ebx = (self.ebx & 0xFFFF_FF00) | value as u32,

            iced_x86::Register::AH => self.eax = (self.eax & 0xFFFF_00FF) | ((value as u32) << 8),
            iced_x86::Register::CH => self.ecx = (self.ecx & 0xFFFF_00FF) | ((value as u32) << 8),
            iced_x86::Register::DH => self.edx = (self.edx & 0xFFFF_00FF) | ((value as u32) << 8),
            iced_x86::Register::BH => self.ebx = (self.ebx & 0xFFFF_00FF) | ((value as u32) << 8),
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get64(&self, reg: iced_x86::Register) -> u64 {
        match reg {
            iced_x86::Register::MM0 => self.mm[0],
            iced_x86::Register::MM1 => self.mm[1],
            iced_x86::Register::MM2 => self.mm[2],
            iced_x86::Register::MM3 => self.mm[3],
            iced_x86::Register::MM4 => self.mm[4],
            iced_x86::Register::MM5 => self.mm[5],
            iced_x86::Register::MM6 => self.mm[6],
            iced_x86::Register::MM7 => self.mm[7],
            _ => unimplemented!("{:?}", reg),
        }
    }
    pub fn set64(&mut self, reg: iced_x86::Register, value: u64) {
        match reg {
            iced_x86::Register::MM0 => self.mm[0] = value,
            iced_x86::Register::MM1 => self.mm[1] = value,
            iced_x86::Register::MM2 => self.mm[2] = value,
            iced_x86::Register::MM3 => self.mm[3] = value,
            iced_x86::Register::MM4 => self.mm[4] = value,
            iced_x86::Register::MM5 => self.mm[5] = value,
            iced_x86::Register::MM6 => self.mm[6] = value,
            iced_x86::Register::MM7 => self.mm[7] = value,
            _ => unimplemented!("{:?}", reg),
        }
    }
}
