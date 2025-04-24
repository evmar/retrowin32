//! Functions expected by the Rust compiler.
//! Typically come from libc, but we are no_std.

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: usize, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = c as u8;
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(cx: *const u8, ct: *const u8, n: usize) -> usize {
    unsafe {
        for i in 0..n {
            let a = *cx.add(i);
            let b = *ct.add(i);
            if a != b {
                return (a as isize - b as isize) as usize;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    unsafe {
        let mut len = 0;
        while *s.add(len) != 0 {
            len += 1;
        }
        len
    }
}
