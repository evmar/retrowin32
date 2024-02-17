//! Types exposed by the Windows API.

use std::marker::PhantomData;

pub type WORD = u16;
pub type DWORD = u32;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct HANDLE<T> {
    pub raw: u32,
    marker: PhantomData<*const T>,
}

unsafe impl<T: 'static> memory::Pod for HANDLE<T> {}

impl<T> HANDLE<T> {
    pub const fn from_raw(raw: u32) -> Self {
        HANDLE {
            raw,
            marker: PhantomData,
        }
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
        self.raw == 0
    }
    pub fn is_invalid(&self) -> bool {
        self.raw == -1i32 as u32
    }
}

impl<T> std::fmt::Debug for HANDLE<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("HANDLE({:x})", self.raw))
    }
}

impl<T> std::fmt::LowerHex for HANDLE<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.raw.fmt(f)
    }
}

impl<'a, T> crate::winapi::stack_args::FromX86<'a> for HANDLE<T> {
    unsafe fn from_stack(mem: memory::Mem, sp: u32) -> Self {
        Self::from_raw(mem.get::<u32>(sp))
    }
}

impl<T> crate::winapi::stack_args::ToX86 for HANDLE<T> {
    fn to_raw(&self) -> u32 {
        self.raw
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HFILET;
pub type HFILE = HANDLE<HFILET>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HWNDT;
pub type HWND = HANDLE<HWNDT>;

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

#[repr(C, packed)]
#[derive(Debug)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
unsafe impl memory::Pod for RECT {}

#[repr(C, packed)]
#[derive(Debug)]
pub struct POINT {
    pub x: DWORD,
    pub y: DWORD,
}
unsafe impl memory::Pod for POINT {}
