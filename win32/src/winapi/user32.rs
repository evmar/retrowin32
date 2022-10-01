#![allow(non_snake_case)]

use super::X86;
use crate::winapi;

pub fn RegisterClassA(_x86: &mut X86, lpWndClass: u32) -> u32 {
    log::warn!("todo: RegisterClassA({:x})", lpWndClass);
    0
}

pub fn CreateWindowExA(
    _x86: &mut X86,
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
    log::warn!("todo: CreateWindowExA({dwExStyle:x}, {lpClassName:x}, {lpWindowName:x}, {dwStyle:x}, {X:x}, {Y:x}, {nWidth:x}, {nHeight:x}, {hWndParent:x}, {hMenu:x}, {hInstance:x}, {lpParam:x})");
    0
}

pub fn UpdateWindow(_x86: &mut X86, hWnd: u32) -> u32 {
    log::warn!("todo: UpdateWindow({hWnd:x})");
    0
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
);
