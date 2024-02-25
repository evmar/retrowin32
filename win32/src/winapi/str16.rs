//! Windows-style strings.

/// UTF-16 string view.
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct Str16([u16]);

impl Str16 {
    pub fn from_buffer<'a>(mem: &'a [u16]) -> &'a Self {
        unsafe { std::mem::transmute(mem) }
    }

    pub fn from_nul_term<'a>(mem: &'a [u16]) -> &'a Self {
        let end = mem.iter().position(|&c| c == 0).unwrap();
        Self::from_buffer(&mem[..end])
    }

    pub unsafe fn from_ptr<'a>(mem: memory::Mem<'a>, addr: u32) -> Option<&'a Self> {
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

    pub fn len(&self) -> usize {
        self.0.len()
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

impl<'a> std::fmt::Debug for &'a Str16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.to_string()))
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
