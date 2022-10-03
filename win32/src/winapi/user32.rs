#![allow(non_snake_case)]

use std::collections::HashMap;

use super::X86;
use crate::winapi;

pub struct Window {}

pub struct State {
    windows: HashMap<u32, Window>,
    next_hwnd: u32,
}
impl State {
    pub fn new() -> Self {
        State {
            windows: HashMap::new(),
            next_hwnd: 1,
        }
    }
}

pub fn RegisterClassA(_x86: &mut X86, lpWndClass: u32) -> u32 {
    log::warn!("todo: RegisterClassA({:x})", lpWndClass);
    0
}

pub fn CreateWindowExA(
    x86: &mut X86,
    dwExStyle: u32,
    lpClassName: u32,
    lpWindowName: u32,
    dwStyle: u32,
    X: u32,
    Y: u32,
    nWidth: u32,
    nHeight: u32,
    hWndParent: u32,
    hMenu: u32,
    hInstance: u32,
    lpParam: u32,
) -> u32 {
    let hwnd = x86.state.user32.next_hwnd;
    x86.state.user32.next_hwnd += 1;

    log::warn!("todo: CreateWindowExA({dwExStyle:x}, {lpClassName:x}, {lpWindowName:x}, {dwStyle:x}, {X:x}, {Y:x}, {nWidth:x}, {nHeight:x}, {hWndParent:x}, {hMenu:x}, {hInstance:x}, {lpParam:x})");

    hwnd
}

pub fn UpdateWindow(_x86: &mut X86, hWnd: u32) -> u32 {
    log::warn!("todo: UpdateWindow({hWnd:x})");
    0
}

pub fn ShowWindow(_x86: &mut X86, _hWnd: u32, _nCmdShow: u32) -> u32 {
    0
}

pub fn SetFocus(_x86: &mut X86, _hWnd: u32) -> u32 {
    // TODO: supposed to return previous focused hwnd.
    0
}

fn LoadIconA(_x86: &mut X86, _hInstance: u32, _lpIconName: u32) -> u32 {
    0
}

fn LoadCursorA(_x86: &mut X86, _hInstance: u32, _lpCursorName: u32) -> u32 {
    0
}

fn GetSystemMetrics(_x86: &mut X86, nIndex: u32) -> u32 {
    const SM_CXSCREEN: u32 = 0;
    const SM_CYSCREEN: u32 = 1;

    match nIndex {
        SM_CXSCREEN => 640,
        SM_CYSCREEN => 480,
        _ => {
            log::warn!("GetSystemMetrics({nIndex})");
            0
        }
    }
}

winapi!(
    fn RegisterClassA(lpWndClass: u32);
    fn CreateWindowExA(
        dwExStyle: u32,
        lpClassName: u32,
        lpWindowName: u32,
        dwStyle: u32,
        X: u32,
        Y: u32,
        nWidth: u32,
        nHeight: u32,
        hWndParent: u32,
        hMenu: u32,
        hInstance: u32,
        lpParam: u32,
    );
    fn UpdateWindow(hWnd: u32);
    fn ShowWindow(hWnd: u32, nCmdShow: u32);
    fn SetFocus(hWnd: u32);

    fn LoadIconA(hInstance: u32, lpIconName: u32);
    fn LoadCursorA(hInstance: u32, lpCursorName: u32);
    fn GetSystemMetrics(nIndex: u32);
);
