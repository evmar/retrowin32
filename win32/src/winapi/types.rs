//! Types exposed by the Windows API.

pub type WORD = u16;
pub type DWORD = u32;

// Handles like HFILE etc. are just u32s.
// I looked at using PhantomData to declare distinct types but I think using
// a macro generates an equivalent amount of code and it's way less confusing
// to work with.
macro_rules! declare_handle {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
        #[repr(transparent)]
        pub struct $name(u32);
        unsafe impl memory::Pod for $name {}
        #[allow(dead_code)]
        impl $name {
            pub const fn from_raw(raw: u32) -> Self {
                $name(raw)
            }

            // Handles have both null and invalid states, whoopsie.
            // https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
            pub fn null() -> Self {
                Self::from_raw(0)
            }
            pub fn invalid() -> Self {
                Self::from_raw(-1i32 as u32)
            }
            pub fn is_null(&self) -> bool {
                self.0 == 0
            }
            pub fn is_invalid(&self) -> bool {
                *self == Self::invalid()
            }
        }
        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                self.0.fmt(f)
            }
        }
        impl crate::winapi::stack_args::FromX86 for $name {
            fn from_raw(raw: u32) -> Self {
                $name(raw)
            }
        }
        impl crate::winapi::stack_args::ToX86 for $name {
            fn to_raw(&self) -> u32 {
                self.0
            }
        }
    };
}

pub(crate) use declare_handle;

declare_handle!(HFILE);
declare_handle!(HWND);

/// UTF-16 string view.
pub struct Str16<'a>(&'a [u16]);

impl<'a> Str16<'a> {
    pub fn from_buffer(mem: &'a [u16]) -> Self {
        Str16(mem)
    }
    pub fn from_nul_term(mem: &'a [u16]) -> Self {
        let end = mem.iter().position(|&c| c == 0).unwrap();
        Str16(&mem[..end])
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

impl<'a> std::fmt::Debug for Str16<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.to_string()))
    }
}

pub struct String16(pub Vec<u16>);

impl String16 {
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
