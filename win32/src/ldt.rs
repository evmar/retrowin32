//! Code to create LDT entries.
//!
//! See https://en.wikipedia.org/wiki/Global_Descriptor_Table
//! and doc/x86-64.md section "LDT".

use crate::segments::SegmentDescriptor;
use std::ffi::c_int;

unsafe extern "C" {
    fn i386_get_ldt(start_sel: c_int, descs: *mut u64, num_sels: c_int) -> c_int;
    fn i386_set_ldt(start_sel: c_int, descs: *const u64, num_sels: c_int) -> c_int;
}

pub fn add_entry(base: u32, size: u32, code: bool) -> u16 {
    let (limit, granularity) = if size >= 0x10000 {
        (size >> 12, true)
    } else {
        (size, false)
    };

    // Intel manual volume 3 section 3.4.5.1 Code-and Data-Segment Descriptor Types
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
        system: true, // code or data segment (S=true means *not* system)
        type_,
    };

    let index = get_free_entry_index();
    let ret = unsafe { i386_set_ldt(index, &entry.encode(), 1) };
    if ret < 0 {
        panic!("i386_set_ldt: {}", std::io::Error::last_os_error());
    }

    // index => selector
    ((index as u16) << 3) | 0b111
}

/// Get an LDT index that is free for adding a new entry.
fn get_free_entry_index() -> i32 {
    // I had thought we could call i386_get_ldt and look through the results for an
    // empty entry, but in practice this approach fails with "invalid argument".
    // The MacOS docs mention LDT_AUTO_ALLOC, but under Rosetta using it just errors.
    // It looks like Wine starts at 32 (grep for `first_ldt_entry`) but I'm not sure why.
    // Looking at a Darwin test, they use an index greater than the table size.
    let mut unused_entry: u64 = 0; // must fetch one entry to get table len
    let ret = unsafe { i386_get_ldt(0, &mut unused_entry, 1) };
    if ret < 0 {
        panic!("i386_get_ldt: {}", std::io::Error::last_os_error());
    }
    ret // length of the LDT, aka first free entry
}

#[allow(dead_code)]
pub fn dump() {
    let mut entries: [u64; 256] = [0; 256];
    let ret = unsafe { i386_get_ldt(0, &mut entries as *mut _, entries.len() as c_int) };
    if ret < 0 {
        panic!("i386_get_ldt: {}", std::io::Error::last_os_error());
    }
    let len = ret as usize;
    let entries = &entries[..len];

    println!("{len} LDT entries:");
    for (i, &entry) in entries.iter().enumerate() {
        let entry = SegmentDescriptor::decode(entry);
        if entry.is_empty() {
            continue;
        }
        println!("#{}: {:#x?}", i, entry);
    }
}
