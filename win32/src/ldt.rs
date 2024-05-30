//! Code to create LDT entries.
//!
//! See https://en.wikipedia.org/wiki/Global_Descriptor_Table
//! and doc/x86-64.md section "LDT".

use crate::segments::SegmentDescriptor;
use core::slice;
use std::ffi::c_int;

#[cfg(target_os = "macos")]
extern "C" {
    fn i386_get_ldt(start_sel: c_int, descs: *mut u64, num_sels: c_int) -> c_int;
    fn i386_set_ldt(start_sel: c_int, descs: *const u64, num_sels: c_int) -> c_int;
}

pub struct LDT {
    next_index: u16,
}

impl Default for LDT {
    fn default() -> Self {
        LDT { next_index: 32 }
    }
}

impl LDT {
    pub fn add_entry(&mut self, base: u32, size: u32, code: bool) -> u16 {
        let (limit, granularity) = if size >= 0x10000 {
            (size >> 12, true)
        } else {
            (size, false)
        };

        // Intel manual 3.4.5.1 Code-and Data-Segment Descriptor Types
        let type_ = if code {
            // code, execute/read, accessed
            0b1011u8
        } else {
            // data, read/write, accessed
            0b0011u8
        };

        let entry = SegmentDescriptor {
            base,
            limit,
            granularity,
            db: true,    // 32-bit, not 16
            long: false, // 32-bit, not 64
            available: false,
            present: true,
            dpl: 3,       // lowest privilege
            system: true, // code or data segment, not system
            type_,
        };

        let index = self.next_index;
        // println!("adding ldt {:x?}", entry);
        todo!();
        //let ret = unsafe { i386_set_ldt(self.next_index as c_int, &entry.encode(), 1) };
        let ret = 0;
        if ret < 0 {
            panic!("i386_set_ldt: {}", std::io::Error::last_os_error());
        }
        self.next_index += 1;

        // index => selector
        (index << 3) | 0b111
    }

    pub fn dump() {
        #[cfg(target_os = "linux")]
        {
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            struct user_desc {
                entry_number: u32,
                base_addr: u32,
                limit: u32,
                flags: u8,
            }
            let mut buf: [user_desc; 16] = unsafe { std::mem::zeroed() };

            let entries = unsafe {
                const MODIFY_LDT: i64 = 154;
                const READ: i64 = 0;
                let ret = libc::syscall(
                    MODIFY_LDT,
                    READ,
                    &mut buf,
                    std::mem::size_of::<[user_desc; 16]>(),
                );
                if ret < 0 {
                    panic!("modify_ldt: {}", std::io::Error::last_os_error());
                }
                std::slice::from_raw_parts(
                    buf.as_ptr(),
                    ret as usize / std::mem::size_of::<user_desc>(),
                )
            };
            println!("{:#?}", entries);
        }

        #[cfg(target_os = "macos")]
        {
            let mut entries: [u64; 256] = [0; 256];
            let ret = unsafe { i386_get_ldt(0, &mut entries as *mut _, entries.len() as c_int) };
            println!("existing: {ret}");
            for (i, &e) in entries.iter().enumerate() {
                let entry = SegmentDescriptor::decode(e);
                if entry.empty() {
                    continue;
                }
                println!("{} {:x} {:#x?}", i, e, entry);
            }
        }
    }
}
