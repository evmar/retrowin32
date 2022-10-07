#![allow(non_snake_case)]

use std::collections::HashMap;

use super::X86;
use crate::{memory::Memory, winapi};

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

pub struct Window {}

pub struct State {
    #[allow(dead_code)] // TODO
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

fn RegisterClassA(_x86: &mut X86, lpWndClass: u32) -> u32 {
    log::warn!("todo: RegisterClassA({:x})", lpWndClass);
    0
}

fn CreateWindowExA(
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

fn UpdateWindow(_x86: &mut X86, hWnd: u32) -> u32 {
    log::warn!("todo: UpdateWindow({hWnd:x})");
    0
}

fn ShowWindow(_x86: &mut X86, _hWnd: u32, _nCmdShow: u32) -> u32 {
    0
}

fn SetFocus(_x86: &mut X86, _hWnd: u32) -> u32 {
    // TODO: supposed to return previous focused hwnd.
    0
}

fn MessageBoxA(x86: &mut X86, _hWnd: u32, lpText: u32, lpCaption: u32, _uType: u32) -> u32 {
    let caption = &x86.mem[lpCaption as usize..].read_strz();
    let text = &x86.mem[lpText as usize..].read_strz();
    x86.host
        .write(format!("MessageBox: {}\n{}", caption, text).as_bytes());
    1 // IDOK
}

fn PeekMessageA(
    _x86: &mut X86,
    lpMsg: u32,
    hWnd: u32,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMs: u32,
) -> u32 {
    log::warn!(
        "PeekMessageA({lpMsg:x}, {hWnd:x}, {wMsgFilterMin:x}, {wMsgFilterMax:x}, {wRemoveMs:x})"
    );
    0 // no messages
}

fn LoadIconA(_x86: &mut X86, _hInstance: u32, _lpIconName: u32) -> u32 {
    0
}

fn LoadCursorA(_x86: &mut X86, _hInstance: u32, _lpCursorName: u32) -> u32 {
    0
}

fn LoadImageA(
    _x86: &mut X86,
    hInstance: u32,
    name: u32,
    typ: u32,
    cx: u32,
    cy: u32,
    fuLoad: u32,
) -> u32 {
    // 400000, 65, 0, 5dc, 118, 0
    log::warn!("LoadImageA({hInstance:x}, {name:x}, {typ:x}, {cx:x}, {cy:x}, {fuLoad:x})");

    if !IS_INTRESOURCE(name) {
        log::error!("unimplemented image name {name:x}");
        return 0;
    }

    const IMAGE_BITMAP: u32 = 0;
    match typ {
        IMAGE_BITMAP => {}
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return 0;
        }
    };

    if fuLoad != 0 {
        log::error!("unimplemented fuLoad {:x}", fuLoad);
        return 0;
    }

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

    fn MessageBoxA(hWnd: u32, lpText: u32, lpCaption: u32, uType: u32);

    fn PeekMessageA(lpMsg: u32, hWnd: u32, wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMs: u32);

    fn LoadIconA(hInstance: u32, lpIconName: u32);
    fn LoadCursorA(hInstance: u32, lpCursorName: u32);
    fn LoadImageA(hInstance: u32, name: u32, typ: u32, cx: u32, cy: u32, fuLoad: u32);
    fn GetSystemMetrics(nIndex: u32);
);
