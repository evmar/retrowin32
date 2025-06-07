//! Code to ensure we reserve the low 4gb of memory for use by the exe,
//! when running it "natively" on x86-64.
//! See "Executable layout" in doc/x86-64.md.

// The lowest range of memory is reserved for catching null pointer dereferences,
// often with OS support and constraints, so we must ensure we don't break that.
//
// - On Mac, the linker sets up a __PAGEZERO segment that defaults to 4gb.
//   We tweak it smaller via the linker flags in `cli/build-rosetta.sh`.
// - On Linux, /proc/sys/vm/mmap_min_addr is the lowest allowed by mmap, typically 64k.
//
// It looks like Windows itself reserves the lower 64k for the same reason, so it's
// likely we don't need any address lower than that.
// https://devblogs.microsoft.com/oldnewthing/20031008-00/?p=42223

const RESV32_SIZE: libc::size_t = 0x7f00_0000 - win32::LOWEST_ADDRESS as libc::size_t;

// Reserved area: LOWEST_ADDRESS is 0x10000, we want to reserve 4gb-0x10000,
// but experimentally if I use constants larger than the below the resulting macho file
// has a section sized zero, even though wine uses a larger constant in similar code (?!).
// Possibly related to
// https://github.com/llvm/llvm-project/commit/b822063669641570ab5edae72956d18a5bcde8c4
// somehow?
// objdump seems to report 32-bit-sized sections in the .o file, even though it's 64-bit output...
// PS: keeping this section from getting dead code eliminated seems to require us putting it in
// the main executable, not in another crate.
#[cfg(target_os = "macos")]
std::arch::global_asm!(
    ".zerofill RESV32,RESV32,_retrowin32_reserve,0x7f000000-0x10000",
    ".no_dead_strip _retrowin32_reserve",
);

/// Remap the lower 4gb that we reserved into being +rwx memory.
pub unsafe fn init_resv32() {
    unsafe {
        #[cfg(target_os = "macos")]
        {
            // unmap the resv32 area created above.
            let ptr = libc::munmap(win32::LOWEST_ADDRESS as *mut libc::c_void, RESV32_SIZE);
            if ptr < 0 {
                panic!("munmap: {:?}", std::io::Error::last_os_error());
            }
        }

        let ptr = libc::mmap(
            win32::LOWEST_ADDRESS as *mut libc::c_void,
            RESV32_SIZE,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANON | libc::MAP_FIXED,
            -1,
            0,
        );
        if (ptr as i64) < 0 {
            panic!("mmap: {:?}", std::io::Error::last_os_error());
        }
        if ptr as usize != win32::LOWEST_ADDRESS as usize {
            panic!(
                "unable to mmap at {addr:x}, got {ptr:x}",
                addr = win32::LOWEST_ADDRESS,
                ptr = ptr as usize
            );
        }
    }
}
