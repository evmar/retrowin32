#![allow(non_snake_case, unused_variables)]

use crate::{winapi::types::*, Machine};

const TRACE_CONTEXT: &'static str = "user32/dialog";

/*
pub mod MessageBoxFlags {
    pub const ABORTRETRYIGNORE: u32 = 0x00000002;
    pub const CANCELTRYCONTINUE: u32 = 0x00000006;
    pub const HELP: u32 = 0x00004000;
    pub const OK: u32 = 0x00000000;
    pub const OKCANCEL: u32 = 0x00000001;
    pub const RETRYCANCEL: u32 = 0x00000005;
    pub const YESNO: u32 = 0x00000004;
    pub const YESNOCANCEL: u32 = 0x00000003;

    pub const ICONEXCLAMATION: u32 = 0x00000030;
    pub const ICONWARNING: u32 = 0x00000030;
    pub const ICONINFORMATION: u32 = 0x00000040;
    pub const ICONASTERISK: u32 = 0x00000040;
    pub const ICONQUESTION: u32 = 0x00000020;
    pub const ICONSTOP: u32 = 0x00000010;
    pub const ICONERROR: u32 = 0x00000010;
    pub const ICONHAND: u32 = 0x00000010;

    pub const DEFBUTTON1: u32 = 0x00000000;
    pub const DEFBUTTON2: u32 = 0x00000100;
    pub const DEFBUTTON3: u32 = 0x00000200;
    pub const DEFBUTTON4: u32 = 0x00000300;

    pub const APPLMODAL: u32 = 0x00000000;
    pub const SYSTEMMODAL: u32 = 0x00001000;
    pub const TASKMODAL: u32 = 0x00002000;
    pub const DEFAULT_DESKTOP_ONLY: u32 = 0x00020000;
    pub const RIGHT: u32 = 0x00080000;
    pub const RTLREADING: u32 = 0x00100000;
    pub const SETFOREGROUND: u32 = 0x00010000;
    pub const TOPMOST: u32 = 0x00040000;
    pub const SERVICE_NOTIFICATION: u32 = 0x00200000;
}
*/

#[win32_derive::dllexport]
pub fn MessageBoxA(
    machine: &mut Machine,
    hWnd: HWND,
    lpText: Option<&str>,
    lpCaption: Option<&str>,
    uType: u32,
) -> u32 {
    machine.host.log(
        format!(
            "MessageBox: {}\n{}",
            lpCaption.unwrap_or("Error"),
            lpText.unwrap_or("")
        )
        .as_bytes(),
    );
    1 // IDOK
}

#[win32_derive::dllexport]
pub fn MessageBoxW(
    machine: &mut Machine,
    hWnd: HWND,
    lpText: Option<&Str16>,
    lpCaption: Option<&Str16>,
    uType: u32,
) -> u32 {
    machine.host.log(
        format!(
            "MessageBox: {}\n{}",
            lpCaption.unwrap().to_string(),
            lpText.unwrap().to_string()
        )
        .as_bytes(),
    );
    1 // IDOK
}

#[win32_derive::dllexport]
pub fn DialogBoxParamA(
    _machine: &mut Machine,
    hInstance: u32,
    lpTemplateName: u32,
    hWndParent: HWND,
    lpDialogFunc: u32,
    dwInitParam: u32,
) -> u32 {
    log::warn!("TODO: not showing dialog");
    // TODO: this should run a nested message loop that will call back into lpDialogFunc,
    // which then will call EndDialog to end the nested message loop and return the value
    // passed to EndDialog.
    // Unfortunately we don't know what value to return here otherwise; it's application specific.
    0
}

#[win32_derive::dllexport]
pub fn DialogBoxIndirectParamA(
    _machine: &mut Machine,
    hInstance: u32,
    hDialogTemplate: u32,
    hWndParent: HWND,
    lpDialogFunc: u32,
    dwInitParam: u32,
) -> i32 {
    log::warn!("TODO: not showing dialog");
    -1 // error
}
