// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
::windows_targets::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : u32) -> !);
::windows_targets::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : STD_HANDLE) -> HANDLE);
::windows_targets::link!("kernel32.dll" "system" fn WriteFile(hfile : HANDLE, lpbuffer : *const u8, nnumberofbytestowrite : u32, lpnumberofbyteswritten : *mut u32, lpoverlapped : *mut OVERLAPPED) -> BOOL);
::windows_targets::link!("user32.dll" "system" fn BeginPaint(hwnd : HWND, lppaint : *mut PAINTSTRUCT) -> HDC);
::windows_targets::link!("user32.dll" "system" fn CreateWindowExA(dwexstyle : WINDOW_EX_STYLE, lpclassname : PCSTR, lpwindowname : PCSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : HWND, hmenu : HMENU, hinstance : HINSTANCE, lpparam : *const ::core::ffi::c_void) -> HWND);
::windows_targets::link!("user32.dll" "system" fn DefWindowProcA(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
::windows_targets::link!("user32.dll" "system" fn DispatchMessageA(lpmsg : *const MSG) -> LRESULT);
::windows_targets::link!("user32.dll" "system" fn EndPaint(hwnd : HWND, lppaint : *const PAINTSTRUCT) -> BOOL);
::windows_targets::link!("user32.dll" "system" fn FillRect(hdc : HDC, lprc : *const RECT, hbr : HBRUSH) -> i32);
::windows_targets::link!("user32.dll" "system" fn GetMessageA(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> BOOL);
::windows_targets::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32) -> ());
::windows_targets::link!("user32.dll" "system" fn RegisterClassA(lpwndclass : *const WNDCLASSA) -> u16);
::windows_targets::link!("user32.dll" "system" fn ShowWindow(hwnd : HWND, ncmdshow : SHOW_WINDOW_CMD) -> BOOL);
::windows_targets::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> BOOL);
pub type BOOL = i32;
pub const COLOR_WINDOW: SYS_COLOR_INDEX = 5i32;
pub const CW_USEDEFAULT: i32 = -2147483648i32;
pub type HANDLE = isize;
pub type HBRUSH = isize;
pub type HCURSOR = isize;
pub type HDC = isize;
pub type HICON = isize;
pub type HINSTANCE = isize;
pub type HMENU = isize;
pub type HWND = isize;
pub type LPARAM = isize;
pub type LRESULT = isize;
#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
impl ::core::marker::Copy for MSG {}
impl ::core::clone::Clone for MSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: HANDLE,
}
impl ::core::marker::Copy for OVERLAPPED {}
impl ::core::clone::Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for OVERLAPPED_0 {}
impl ::core::clone::Clone for OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl ::core::marker::Copy for OVERLAPPED_0_0 {}
impl ::core::clone::Clone for OVERLAPPED_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [u8; 32],
}
impl ::core::marker::Copy for PAINTSTRUCT {}
impl ::core::clone::Clone for PAINTSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PCSTR = *const u8;
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for POINT {}
impl ::core::clone::Clone for POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RECT {}
impl ::core::clone::Clone for RECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SHOW_WINDOW_CMD = i32;
pub type STD_HANDLE = u32;
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
pub const SW_NORMAL: SHOW_WINDOW_CMD = 1i32;
pub type SYS_COLOR_INDEX = i32;
pub type WINDOW_EX_STYLE = u32;
pub type WINDOW_STYLE = u32;
pub const WM_DESTROY: u32 = 2u32;
pub const WM_PAINT: u32 = 15u32;
#[repr(C)]
pub struct WNDCLASSA {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: PCSTR,
    pub lpszClassName: PCSTR,
}
impl ::core::marker::Copy for WNDCLASSA {}
impl ::core::clone::Clone for WNDCLASSA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WNDCLASS_STYLES = u32;
pub type WNDPROC = ::core::option::Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT,
>;
pub type WPARAM = usize;
pub const WS_OVERLAPPED: WINDOW_STYLE = 0u32;