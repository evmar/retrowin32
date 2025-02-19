use crate::{
    winapi::{calling_convention::ArrayWithSizeMut, *},
    Machine,
};

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
    lpText: Option<&CStr>,
    lpCaption: Option<&CStr>,
    uType: u32,
) -> u32 {
    let caption = lpCaption.unwrap();
    let caption = caption.to_str_or_warn();
    let text = lpText.unwrap();
    let text = text.to_str_or_warn();
    log::info!("MessageBox: {caption}\n{text}");
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
    log::info!(
        "MessageBox: {}\n{}",
        lpCaption.unwrap().to_string(),
        lpText.unwrap().to_string()
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
pub fn DialogBoxParamW(
    _machine: &mut Machine,
    hInstance: u32,
    lpTemplateName: u32,
    hWndParent: HWND,
    lpDialogFunc: u32,
    dwInitParam: u32,
) -> u32 {
    todo!();
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

#[win32_derive::dllexport]
pub fn SetDlgItemTextA(
    _machine: &mut Machine,
    hDlg: HWND,
    nIDDlgItem: i32,
    lpString: Option<&str>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn SetDlgItemTextW(
    _machine: &mut Machine,
    hDlg: HWND,
    nIDDlgItem: i32,
    lpString: Option<&Str16>,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn SetDlgItemInt(
    _machine: &mut Machine,
    hDlg: HWND,
    nIDDlgItem: i32,
    uValue: u32,
    _bSigned: bool,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetDlgItem(machine: &mut Machine, hDlg: HWND, nIDDlgItem: i32) -> HWND {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetDlgItemTextW(
    machine: &mut Machine,
    hDlg: HWND,
    nIDDlgItem: i32,
    lpString: ArrayWithSizeMut<u16>,
) -> u32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetDlgItemInt(
    _machine: &mut Machine,
    hDlg: HWND,
    nIDDlgItem: i32,
    lpTranslated: Option<&mut u32>,
    bSigned: bool,
) -> u32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn CheckRadioButton(
    _machine: &mut Machine,
    hDlg: HWND,
    nIDFirstButton: i32,
    nIDLastButton: i32,
    nIDCheckButton: i32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn CheckDlgButton(_machine: &mut Machine, hDlg: HWND, nIDButton: i32, uCheck: u32) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn IsDlgButtonChecked(_machine: &mut Machine, hDlg: HWND, nIDButton: i32) -> u32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn EndDialog(_machine: &mut Machine, hDlg: HWND, nResult: Option<&mut u32>) -> bool {
    todo!();
}
