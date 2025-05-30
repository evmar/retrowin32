//! GDT setup, needed for running under Unicorn.
//! This module is currently unused and bitrotting.

use crate::segments::SegmentDescriptor;
use win32_system::memory::Memory;

/// Result of setting up the GDT, with initial values for all the relevant segment registers.
pub struct GDTEntries {
    /// Address of GDT itself.
    pub addr: u32,
    pub cs: u16,
    pub ds: u16,
    pub fs: u16,
    pub ss: u16,
}

pub fn create_gdt(memory: &Memory, fs_addr: u32) -> GDTEntries {
    use memory::ExtensionsMut;

    const COUNT: usize = 5;

    let addr = memory.alloc(8 * COUNT as u32);
    let gdt: &mut [u64; COUNT] = unsafe { &mut *(memory.mem().get_ptr_mut::<u64>(addr) as *mut _) };

    gdt[0] = 0;

    let cs = (1 << 3) | 0b011;
    gdt[1] = SegmentDescriptor {
        base: 0x0000_0000,
        limit: 0xFFFF_FFFF,
        granularity: true,
        db: true, // 32 bit
        long: false,
        available: false,
        present: true,
        dpl: 3,
        system: true,  // code/data
        type_: 0b1011, // code, execute/read, accessed
    }
    .encode();

    let ds = (2 << 3) | 0b011;
    gdt[2] = SegmentDescriptor {
        base: 0x0000_0000,
        limit: 0xFFFF_FFFF,
        granularity: true,
        db: true, // 32 bit
        long: false,
        available: false,
        present: true,
        dpl: 3,
        system: true,  // code/data
        type_: 0b0011, // data, read/write, accessed
    }
    .encode();

    let fs = (3 << 3) | 0b011;
    gdt[3] = SegmentDescriptor {
        base: fs_addr,
        limit: 0x1000,
        granularity: false,
        db: true, // 32 bit
        long: false,
        available: false,
        present: true,
        dpl: 3,
        system: true,  // code/data
        type_: 0b0011, // data, read/write, accessed
    }
    .encode();

    // unicorn test says: "when setting SS, need rpl == cpl && dpl == cpl",
    // which is to say because the system is level 0 (cpl) we need the descriptor
    // to also be zero (dpl) and the selector to also be zero (rpl, the 0b000 here).
    let ss = (4 << 3) | 0b000;
    gdt[4] = SegmentDescriptor {
        base: 0x0000_0000,
        limit: 0xFFFF_FFFF,
        granularity: true,
        db: true, // 32 bit
        long: false,
        available: false,
        present: true,
        dpl: 0,        // NOTE: this is different from others
        system: true,  // code/data
        type_: 0b0011, // data, read/write, accessed
    }
    .encode();

    GDTEntries {
        addr,
        cs,
        ds,
        fs,
        ss,
    }
}
