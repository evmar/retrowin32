use bitflags::bitflags;

use crate::{
    winapi::{stack_args::ToX86, types::*},
    Machine,
};

const TRACE_CONTEXT: &'static str = "user32/resource";

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: WM,
    pub wParam: u32,
    pub lParam: u32,
    pub time: u32,
    // TODO: struct POINT
    pub pt_x: u32,
    pub pt_y: u32,
    pub lPrivate: u32,
}
unsafe impl memory::Pod for MSG {}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum WM {
    CREATE = 0x0001,
    ACTIVATEAPP = 0x001C,
}
impl TryFrom<u32> for WM {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == WM::CREATE as u32 => WM::CREATE,
            x if x == WM::ACTIVATEAPP as u32 => WM::ACTIVATEAPP,
            x => return Err(x),
        })
    }
}

bitflags! {
    pub struct RemoveMsg: u32 {
        const PM_NOREMOVE = 0x0000;
        const PM_REMOVE = 0x0001;
        const PM_NOYIELD = 0x0002;
    }
}
impl TryFrom<u32> for RemoveMsg {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        RemoveMsg::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMsg: Result<RemoveMsg, u32>,
) -> bool {
    machine.host.pump_messages();

    // TODO: obey HWND.
    let remove = wRemoveMsg.unwrap();
    let msg = match machine.state.user32.messages.front() {
        Some(msg) => msg,
        None => return false,
    };
    *lpMsg.unwrap() = msg.clone();

    if remove.contains(RemoveMsg::PM_REMOVE) {
        machine.state.user32.messages.pop_front();
    }

    true
}

#[win32_derive::dllexport]
pub fn GetMessageA(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> bool {
    if !PeekMessageA(
        machine,
        lpMsg,
        hWnd,
        wMsgFilterMin,
        wMsgFilterMax,
        Ok(RemoveMsg::PM_REMOVE),
    ) {
        todo!();
    }
    return true;
}

#[win32_derive::dllexport]
pub fn WaitMessage(_machine: &mut Machine) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_machine: &mut Machine, lpMsg: Option<&MSG>) -> bool {
    // TODO: translate key-related messages into enqueuing a WM_CHAR.
    false // no message translated
}

#[win32_derive::dllexport]
pub async fn DispatchMessageA(m: *mut Machine, lpMsg: Option<&MSG>) -> u32 {
    let machine = unsafe { &mut *m };
    let msg = lpMsg.unwrap();
    let window = &machine.state.user32.windows[msg.hwnd.to_raw() as usize - 1];
    // TODO: SetWindowLong can change the wndproc.
    crate::shims::call_x86(
        machine,
        window.wndclass.wndproc,
        vec![
            msg.hwnd.to_raw(),
            msg.message as u32,
            msg.wParam,
            msg.lParam,
        ],
    )
    .await;
    0
}
