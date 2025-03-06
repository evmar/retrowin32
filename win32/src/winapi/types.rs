//! Types exposed by the Windows API.

use memory::Extensions;
use std::borrow::Cow;

pub use super::handle::HANDLE;
pub use std::ffi::CStr;

pub trait StrExt<'a> {
    fn to_str_or_warn(&'a self) -> Cow<'a, str>;
}

impl<'a> StrExt<'a> for &'a CStr {
    fn to_str_or_warn(&'a self) -> Cow<'a, str> {
        match self.to_str() {
            Ok(str) => Cow::Borrowed(str),
            Err(_err) => {
                log::warn!("[TODO: {:?}]", self);
                Cow::Owned(format!("TODO: convert {:?} using code page", self))
            }
        }
    }
}

pub type WORD = u16;
pub type DWORD = u32;
pub type LPARAM = u32;

pub type HRESULT = u32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HFILET;
pub type HFILE = HANDLE<HFILET>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HFINDT;
pub type HFIND = HANDLE<HFINDT>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HRSRCT;
pub type HRSRC = HANDLE<HRSRCT>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HWNDT;
pub type HWND = HANDLE<HWNDT>;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
unsafe impl memory::Pod for RECT {}

impl RECT {
    pub fn clip(&self, other: &RECT) -> RECT {
        RECT {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub fn origin(&self) -> POINT {
        POINT {
            x: self.left,
            y: self.top,
        }
    }

    pub fn size(&self) -> POINT {
        POINT {
            x: self.right - self.left,
            y: self.bottom - self.top,
        }
    }

    pub fn contains(&self, point: POINT) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }

    pub fn add(&self, delta: POINT) -> RECT {
        RECT {
            left: self.left + delta.x,
            top: self.top + delta.y,
            right: self.right + delta.x,
            bottom: self.bottom + delta.y,
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
unsafe impl memory::Pod for POINT {}

impl POINT {
    pub fn add(&self, delta: POINT) -> POINT {
        POINT {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }

    pub fn sub(&self, delta: POINT) -> POINT {
        POINT {
            x: self.x - delta.x,
            y: self.y - delta.y,
        }
    }

    pub fn mul(&self, o: POINT) -> POINT {
        POINT {
            x: self.x * o.x,
            y: self.y * o.y,
        }
    }

    pub fn div(&self, o: POINT) -> POINT {
        POINT {
            x: self.x / o.x,
            y: self.y / o.y,
        }
    }
}

impl<'a> crate::calling_convention::FromStack<'a> for POINT {
    unsafe fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let x = mem.get_pod::<i32>(sp);
        let y = mem.get_pod::<i32>(sp + 4);
        POINT { x, y }
    }
}

pub const MAX_PATH: usize = 260;
