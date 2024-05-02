use bitflags::bitflags;
use iced_x86::Register::{self, *};

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
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Registers {
    /// 32-bit registers, in order:
    ///   eax ecx edx ebx esp ebp esi edi,
    /// which matches the enums in iced_x86.
    // Note: register access is hot in profiles.
    // Previously we had named fields eax ebx etc., but to access them generally,
    // if we write a match statement that maps register N to field at offset 4*N,
    // llvm doesn't seem to optimize it to the obvious math.
    // I tried the equivalent in C++ and that didn't optimize either.
    // So instead we represent these as an array internally.
    r32: [u32; 8],

    pub eip: u32,

    /// Segment registers, in order:
    ///   es cs ss ds fs gs
    segment: [u16; 6],

    // TODO: segment registers are actually 16-bit indexes into the GDT/LDT,
    // but for our purposes all we ever care about is making FS-relative accesses point
    // at the Windows TEB.
    /// Address that FS-relative accesses point to.
    pub fs_addr: u32,

    /// MMX registers.
    // TODO: officially these should alias the FPU registers(!).
    pub mm: [u64; 8],
}

#[allow(dead_code)]
const fn assert_enums_as_expected() {
    assert!(ECX as u8 == EAX as u8 + 1);
    assert!(EDX as u8 == EAX as u8 + 2);
    assert!(EBX as u8 == EAX as u8 + 3);
    assert!(ESP as u8 == EAX as u8 + 4);
    assert!(EBP as u8 == EAX as u8 + 5);
    assert!(ESI as u8 == EAX as u8 + 6);
    assert!(EDI as u8 == EAX as u8 + 7);

    assert!(CX as u8 == AX as u8 + 1);
    assert!(DX as u8 == AX as u8 + 2);
    assert!(BX as u8 == AX as u8 + 3);

    assert!(CL as u8 == AL as u8 + 1);
    assert!(DL as u8 == AL as u8 + 2);
    assert!(BL as u8 == AL as u8 + 3);

    assert!(CH as u8 == AH as u8 + 1);
    assert!(DH as u8 == AH as u8 + 2);
    assert!(BH as u8 == AH as u8 + 3);

    assert!(CS as u8 == ES as u8 + 1);
    assert!(SS as u8 == ES as u8 + 2);
    assert!(DS as u8 == ES as u8 + 3);
    assert!(FS as u8 == ES as u8 + 4);
    assert!(GS as u8 == ES as u8 + 5);

    assert!(MM1 as u8 == MM0 as u8 + 1);
    assert!(MM2 as u8 == MM0 as u8 + 2);
    assert!(MM3 as u8 == MM0 as u8 + 3);
    assert!(MM4 as u8 == MM0 as u8 + 4);
    assert!(MM5 as u8 == MM0 as u8 + 5);
    assert!(MM6 as u8 == MM0 as u8 + 6);
    assert!(MM7 as u8 == MM0 as u8 + 7);
}
const _: () = assert_enums_as_expected();

/// Map a 16-bit register (e.g. BX) to its 32-bit underlying register (e.g. EBX).
fn r16_to_32(r16: Register) -> Register {
    // Safety: see check in assert_enums_as_expected.
    unsafe { std::mem::transmute((r16 as u8 - AX as u8) + EAX as u8) }
}

/// Map an 8-bit low register (e.g. BL) to its 32-bit underlying register (e.g. EBX).
fn r8l_to_32(r8: Register) -> Register {
    // Safety: see check in assert_enums_as_expected.
    unsafe { std::mem::transmute((r8 as u8 - AL as u8) + EAX as u8) }
}

/// Map an 8-bit high  register (e.g. BH) to its 32-bit underlying register (e.g. EBX).
fn r8h_to_32(r8: Register) -> Register {
    // Safety: see check in assert_enums_as_expected.
    unsafe { std::mem::transmute((r8 as u8 - AH as u8) + EAX as u8) }
}

impl Registers {
    pub fn get32_mut(&mut self, reg: Register) -> &mut u32 {
        let idx = reg as usize - Register::EAX as usize;
        // See check in assert_enums_as_expected() -- the registers we can fetch are always < 8.
        if idx >= 8 {
            unreachable!("{reg:?}");
        }
        &mut self.r32[idx]
    }

    pub fn get32(&self, reg: Register) -> u32 {
        // See comments in get32_mut.
        let idx = reg as usize - Register::EAX as usize;
        if idx >= 8 {
            unreachable!("{reg:?}");
        }
        self.r32[idx]
    }

    pub fn get16(&self, reg: Register) -> u16 {
        match reg {
            AX | CX | DX | BX | SP | BP | SI | DI => self.get32(r16_to_32(reg)) as u16,

            ES | CS | SS | DS | FS | GS => {
                let idx = reg as usize - ES as usize;
                self.segment[idx]
            }
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get16_mut(&mut self, reg: Register) -> &mut u16 {
        unsafe {
            match reg {
                AX | CX | DX | BX | SP | BP | SI | DI => {
                    &mut *(self.get32_mut(r16_to_32(reg)) as *mut u32 as *mut u16)
                }
                ES | CS | SS | DS | FS | GS => {
                    let idx = reg as usize - ES as usize;
                    &mut self.segment[idx]
                }
                _ => unreachable!("{reg:?}"),
            }
        }
    }

    pub fn get8(&self, reg: Register) -> u8 {
        match reg {
            AL | CL | DL | BL => self.get32(r8l_to_32(reg)) as u8,
            AH | CH | DH | BH => (self.get32(r8h_to_32(reg)) >> 8) as u8,
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn get8_mut(&mut self, reg: Register) -> &mut u8 {
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

    pub fn set32(&mut self, reg: Register, value: u32) {
        *self.get32_mut(reg) = value;
    }

    pub fn set16(&mut self, reg: Register, value: u16) {
        match reg {
            AX | CX | DX | BX | SI | DI | BP => {
                let r32 = r16_to_32(reg);
                self.set32(r32, (self.get32(r32) & 0xFFFF_0000) | value as u32);
            }
            ES | CS | SS | DS | FS | GS => {
                let idx = reg as usize - ES as usize;
                self.segment[idx] = value;
            }
            _ => unreachable!("{reg:?}"),
        }
    }

    pub fn set8(&mut self, reg: Register, value: u8) {
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

    pub fn get64(&self, reg: Register) -> u64 {
        let index = reg as usize - MM0 as usize;
        if index >= 8 {
            unreachable!("{reg:?}");
        }
        self.mm[index]
    }

    pub fn set64(&mut self, reg: Register, value: u64) {
        let index = reg as usize - MM0 as usize;
        if index >= 8 {
            unreachable!("{reg:?}");
        }
        self.mm[index] = value;
    }
}
