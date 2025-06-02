//! Code to ensure we reserve the low 4gb of memory for use in the exe.
//! See "Executable layout" in doc/x86-64.md.

const PAGEZERO_END: libc::size_t = 0x1000;
const RESV32_SIZE: libc::size_t = 0x7f000000 - PAGEZERO_END;

// Reserved area: pagezero is 0x1000, we want to reserve 4gb-0x1000,
// but experimentally if I use constants larger than the below the resulting macho file
// has a section sized zero, even though wine uses a larger constant in similar code (?!).
// Possibly related to
// https://github.com/llvm/llvm-project/commit/b822063669641570ab5edae72956d18a5bcde8c4
// somehow?
// objdump seems to report 32-bit-sized sections in the .o file, even though it's 64-bit output...
// PS: keeping this section from getting dead code eliminated seems to require us putting it in
// the main executable, not in another crate.
std::arch::global_asm!(
    ".zerofill RESV32,RESV32,_retrowin32_reserve,0x7f000000-0x1000",
    ".no_dead_strip _retrowin32_reserve",
);

/// Remap the lower 4gb that we reserved into being +rwx memory.
pub unsafe fn init_resv32() {
    unsafe {
        let ptr = libc::munmap(PAGEZERO_END as *mut libc::c_void, RESV32_SIZE);
        if ptr < 0 {
            panic!("munmap: {:?}", std::io::Error::last_os_error());
        }

        let ptr = libc::mmap(
            PAGEZERO_END as *mut libc::c_void,
            RESV32_SIZE,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if (ptr as i64) < 0 {
            panic!("mmap: {:?}", std::io::Error::last_os_error());
        }
        if ptr as usize != PAGEZERO_END {
            panic!("unable to mmap at {:x?}", ptr as usize);
        }
    }
}
