//! Windows-style strings.

/// UTF-16 string view.
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct Str16([u16]);

impl Str16 {
    pub fn from_bytes(mem: &[u8]) -> &Self {
        Str16::from_buffer(unsafe {
            std::slice::from_raw_parts(mem.as_ptr() as *const _, mem.len() / 2)
        })
    }

    pub fn from_bytes_mut(mem: &mut [u8]) -> &mut Self {
        Str16::from_buffer_mut(unsafe {
            std::slice::from_raw_parts_mut(mem.as_ptr() as *mut _, mem.len() / 2)
        })
    }

    pub fn from_buffer(mem: &[u16]) -> &Self {
        unsafe { std::mem::transmute(mem) }
    }

    pub fn from_buffer_mut(mem: &mut [u16]) -> &mut Self {
        unsafe { std::mem::transmute(mem) }
    }

    pub fn from_nul_term(mem: &[u16]) -> &Self {
        let end = mem.iter().position(|&c| c == 0).unwrap();
        Self::from_buffer(&mem[..end])
    }

    pub unsafe fn from_nul_term_ptr(mem: memory::Mem, addr: u32) -> Option<&Self> {
        if addr == 0 {
            return None;
        }
        let mem16: &[u16] = {
            let mem = mem.slice(addr..);
            let ptr = mem.as_slice_todo().as_ptr() as *const u16;
            std::slice::from_raw_parts(ptr, mem.len() as usize / 2)
        };
        Some(Self::from_nul_term(mem16))
    }

    pub fn buf(&self) -> &[u16] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|&c| {
                if c > 0xFF {
                    // TODO
                    panic!("unhandled non-ascii {:?}", char::from_u32(c as u32));
                }
                c as u8 as char
            })
            .collect()
    }
}

impl std::fmt::Debug for Str16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.to_string()))
    }
}

impl std::ops::Deref for Str16 {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Str16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(PartialEq, Eq)]
pub struct String16(pub Vec<u16>);

impl String16 {
    pub fn as_str16(&self) -> &Str16 {
        Str16::from_buffer(&self.0)
    }

    pub fn byte_size(&self) -> usize {
        self.0.len() * 2
    }

    pub fn from(str: &str) -> Self {
        String16(
            str.chars()
                .map(|c| {
                    if c as u16 > 0x7f {
                        panic!("unhandled non-ascii {:?}", c);
                    }
                    c as u16
                })
                .collect(),
        )
    }
}

impl std::ops::Deref for String16 {
    type Target = Str16;

    fn deref(&self) -> &Self::Target {
        self.as_str16()
    }
}

pub fn expect_ascii(slice: &[u8]) -> &str {
    match std::str::from_utf8(slice) {
        Ok(str) => str,
        Err(err) => {
            // If we hit one of these, we ought to change the caller to not use to_ascii().
            panic!("failed to ascii convert {:?}: {}", slice, err);
        }
    }
}
