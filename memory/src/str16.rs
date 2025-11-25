//! Windows-style UTF-16 strings.
//! TODO: this probably doesn't belong in the memory crate,
//! but there is probably a real UTF-16 crate that we should use instead.

use crate::{Extensions, Mem};

/// UTF-16 string view.
/// Still uses a [u8] underneath for alignment reasons.
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct Str16([u8]);

impl Str16 {
    pub fn from_bytes(buf: &[u8]) -> &Self {
        assert_eq!(buf.len() % 2, 0);
        unsafe { std::mem::transmute(buf) }
    }

    pub fn from_bytes_mut(buf: &mut [u8]) -> &mut Self {
        assert_eq!(buf.len() % 2, 0);
        unsafe { std::mem::transmute(buf) }
    }

    pub fn from_u16(buf: &[u16]) -> &Self {
        Self::from_bytes(unsafe {
            std::slice::from_raw_parts(buf.as_ptr() as *const _, buf.len() * 2)
        })
    }

    pub fn from_nul_term_ptr(mem: Mem<'_>, addr: u32) -> Option<&Self> {
        if addr == 0 {
            return None;
        }
        let end = mem
            .iter_pod::<u16>(addr, (mem.len() - addr) / 2)
            .position(|c| c == 0)
            .unwrap();
        Some(Self::from_bytes(mem.sub32(addr, end as u32 * 2)))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0
            .into_iter_pod::<u16>()
            .map(|c| {
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
    type Target = [u8];

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
        Str16::from_u16(&self.0)
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
