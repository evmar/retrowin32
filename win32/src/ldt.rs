//! Code to create LDT entries.
//!
//! See https://en.wikipedia.org/wiki/Global_Descriptor_Table
//! and doc/x86-64.md section "LDT".

use std::ffi::c_int;

// https://en.wikipedia.org/wiki/Segment_descriptor
// little endian, low bits first
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct LDT_ENTRY {
    LimitLow: u16,
    BaseLow: u16,
    BaseMid: u8,

    /*
    unsigned    Type : 5;
    unsigned    Dpl : 2;
    unsigned    Pres : 1;
    */
    Flags1: u8,
    /*
    unsigned    LimitHi : 4;
    unsigned    Sys : 1;
    unsigned    Reserved_0 : 1;
    unsigned    Default_Big : 1;
    unsigned    Granularity : 1;
    */
    Flags2: u8,

    BaseHi: u8,
}

impl std::fmt::Debug for LDT_ENTRY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LDT_ENTRY")
            .field("LimitLow", &self.LimitLow)
            .field("BaseLow", &self.BaseLow)
            .field("BaseMid", &self.BaseMid)
            .field("Type", &(self.Flags1 & 0x1F))
            .field("Dpl", &((self.Flags1 >> 5) & 0x3))
            .field("Pres", &((self.Flags1 >> 7) & 0x1))
            .field("LimitHi", &(self.Flags2 & 0xF))
            .field("Sys", &((self.Flags2 >> 4) & 0x1))
            .field("Default_Big", &((self.Flags2 >> 6) & 0x1))
            .field("Granularity", &((self.Flags2 >> 7) & 0x1))
            .field("BaseHi", &self.BaseHi)
            .finish()
    }
}

extern "C" {
    fn i386_get_ldt(start_sel: c_int, descs: *mut LDT_ENTRY, num_sels: c_int) -> c_int;
    fn i386_set_ldt(start_sel: c_int, descs: *const LDT_ENTRY, num_sels: c_int) -> c_int;
}

pub struct LDT {
    next_index: u16,
}

impl LDT {
    pub fn new() -> Self {
        LDT { next_index: 32 }
    }

    pub fn add_entry(&mut self, base: u32, size: u32, code: bool) -> u16 {
        let (limit, granularity) = if size >= 0x10000 {
            (size >> 12, 1u8)
        } else {
            (size, 0u8)
        };

        // type bits:
        //  S: 0=system, 1=code/data
        //   : 0=data, 1=code
        //  E: expand-down
        //  W: writeable
        //  A: accessed
        let type_ = if code {
            // code, execute/read, accessed
            0b11011u8
        } else {
            // data, read/write, accessed
            0b10011u8
        };
        let dpl = 3u8;
        let pres = 1u8;
        let flags1 = (pres << 7) | (dpl << 5) | type_;

        let limit_hi = ((limit >> 16) & 0xF) as u8;
        let sys = 0u8;
        let default_big = 1u8; // 32bit, not 16
        let flags2: u8 = (granularity << 7) | (default_big << 6) | (sys << 4) | limit_hi;

        let entry = LDT_ENTRY {
            BaseLow: base as u16,
            BaseMid: (base >> 16) as u8,
            BaseHi: (base >> 24) as u8,
            LimitLow: limit as u16,
            Flags1: flags1,
            Flags2: flags2,
        };
        let index = self.next_index;
        // println!("adding ldt {:x?}", entry);
        let ret = unsafe { i386_set_ldt(self.next_index as c_int, &entry, 1) };
        if ret < 0 {
            panic!("i386_set_ldt: {}", std::io::Error::last_os_error());
        }
        self.next_index += 1;

        // index => selector
        (index << 3) | 0b111
    }

    #[allow(dead_code)]
    unsafe fn dump() {
        let mut entries: [LDT_ENTRY; 256] = std::mem::zeroed();
        let ret = i386_get_ldt(0, &mut entries as *mut LDT_ENTRY, 256);
        println!("existing: {ret}");
        for (i, e) in entries.iter().enumerate() {
            if e.BaseLow == 0 && e.Flags1 == 0 && e.Flags2 == 0 {
                continue;
            }
            println!("{} {:x?}", i, e);
        }
    }
}

/// Set up two LDT entries, for code and the FS register.
/// Set the FS selector to point at the teb address,
/// and return the selector used for 32-bit code.
pub unsafe fn setup_ldt(teb: u32) -> u16 {
    let mut ldt = LDT::new();

    // Wine marks all of memory as code.
    let cs = ldt.add_entry(0, 0xFFFF_FFFF, true);

    // NOTE: OSX seems extremely sensitive to the values used here, where like
    // using a span size that is not exactly 0xFFF causes the entry to be rejected.
    let fs_sel = ldt.add_entry(teb, 0xFFF, false);
    std::arch::asm!(
        "mov fs,{fs_sel:x}",
        fs_sel = in(reg) fs_sel
    );

    cs
}
