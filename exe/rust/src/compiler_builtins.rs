//! Functions expected by the Rust compiler.
//! Typically come from libc, but we are no_std.

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: usize, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = c as u8;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(cx: *const u8, ct: *const u8, n: usize) -> usize {
    for i in 0..n {
        let a = *cx.add(i);
        let b = *ct.add(i);
        if a != b {
            return (a as isize - b as isize) as usize;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}
