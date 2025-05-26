//! Types exposed by the Windows API.

use crate::handle::HANDLE;
use std::borrow::Cow;

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
pub struct HWNDT;
pub type HWND = HANDLE<HWNDT>;
