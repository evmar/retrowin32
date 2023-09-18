#![allow(non_snake_case)]

mod message;
mod resource;
mod window;

pub use super::gdi32::HDC;
use super::{stack_args::ToX86, types::*};
use crate::{machine::Machine, pe};
pub use message::*;
use num_traits::FromPrimitive;
pub use resource::*;
use std::collections::VecDeque;
use std::rc::Rc;
pub use window::*;

const TRACE_CONTEXT: &'static str = "user32";

type HINSTANCE = u32;

pub struct State {
    pub resources: pe::IMAGE_DATA_DIRECTORY,
    wndclasses: Vec<Rc<WndClass>>,
    windows: Vec<Window>,
    messages: VecDeque<MSG>,
}
impl State {
    pub fn get_window(&mut self, hwnd: HWND) -> &mut Window {
        &mut self.windows[hwnd.to_raw() as usize - 1]
    }
}
impl Default for State {
    fn default() -> Self {
        State {
            resources: pe::IMAGE_DATA_DIRECTORY::default(),
            wndclasses: Vec::new(),
            windows: Vec::new(),
            messages: VecDeque::new(),
        }
    }
}

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
    machine.host.write(
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
pub fn DialogBoxParamA(
    _machine: &mut Machine,
    hInstance: u32,
    lpTemplateName: u32,
    hWndParent: HWND,
    lpDialogFunc: u32,
    dwInitParam: u32,
) -> u32 {
    log::warn!("TODO: DialogBoxParamA({hInstance:x}, {lpTemplateName:x}, {hWndParent:x}, {lpDialogFunc:x}, {dwInitParam:x})");
    // TODO: this should run a nested message loop that will call back into lpDialogFunc,
    // which then will call EndDialog to end the nested message loop and return the value
    // passed to EndDialog.
    // Unfortunately we don't know what value to return here otherwise; it's application specific.
    0
}

#[derive(Debug, FromPrimitive)]
enum SystemMetric {
    CXSCREEN = 1,
    CYSCREEN = 2,
    CYCAPTION = 4,
    CYBORDER = 6,
    CXFRAME = 32,
    CYFRAME = 33,
}

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_machine: &mut Machine, nIndex: u32) -> u32 {
    let metric = match SystemMetric::from_u32(nIndex) {
        Some(metric) => metric,
        None => {
            log::error!("GetSystemMetrics({nIndex})");
            return 0;
        }
    };
    match metric {
        SystemMetric::CXSCREEN => 640,
        SystemMetric::CYSCREEN => 480,
        SystemMetric::CYCAPTION => 3,
        SystemMetric::CYBORDER => 1,
        SystemMetric::CXFRAME => 8,
        SystemMetric::CYFRAME => 8,
    }
}