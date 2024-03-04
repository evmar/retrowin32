use bitflags::bitflags;

use crate::{host, winapi::types::*, Machine, MouseButton};

const TRACE_CONTEXT: &'static str = "user32/resource";

#[repr(C)]
#[derive(Clone)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: u32,
    pub lParam: u32,
    pub time: u32,
    // TODO: struct POINT
    pub pt_x: u32,
    pub pt_y: u32,
    pub lPrivate: u32,
}
unsafe impl memory::Pod for MSG {}
impl std::fmt::Debug for MSG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wm = WM::try_from(self.message);
        f.debug_struct("MSG")
            .field("hwnd", &self.hwnd)
            .field("message", &wm)
            .field("wParam", &self.wParam)
            .field("lParam", &self.lParam)
            .field("time", &self.time)
            .field("pt_x", &self.pt_x)
            .field("pt_y", &self.pt_y)
            .field("lPrivate", &self.lPrivate)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, win32_derive::TryFromEnum)]
pub enum WM {
    NULL = 0,
    CREATE = 0x0001,
    PAINT = 0x000F,
    QUIT = 0x0012,
    ACTIVATEAPP = 0x001C,
    LBUTTONDOWN = 0x0201,
    LBUTTONUP = 0x0202,
    LBUTTONDBLCLK = 0x0203,
    RBUTTONDOWN = 0x0204,
    RBUTTONUP = 0x0205,
    RBUTTONDBLCLK = 0x0206,
    MBUTTONDOWN = 0x0207,
    MBUTTONUP = 0x0208,
    MBUTTONDBLCLK = 0x0209,
}

fn msg_from_message(message: host::Message) -> MSG {
    let mut msg = MSG {
        hwnd: HWND::from_raw(message.hwnd),
        message: WM::QUIT as u32, // will be overwritten
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    };

    match &message.detail {
        host::MessageDetail::Quit => {
            msg.message = WM::QUIT as u32;
        }
        host::MessageDetail::Mouse(mouse) => {
            msg.message = match (mouse.button, mouse.down) {
                (MouseButton::Left, true) => WM::LBUTTONDOWN,
                (MouseButton::Left, false) => WM::LBUTTONUP,
                (MouseButton::Right, true) => WM::RBUTTONDOWN,
                (MouseButton::Right, false) => WM::RBUTTONUP,
                (MouseButton::Middle, true) => WM::MBUTTONDOWN,
                (MouseButton::Middle, false) => WM::MBUTTONUP,
            } as u32;
            msg.wParam = 0; // TODO:  modifiers
            msg.lParam = (mouse.y << 16) | mouse.x;
        }
    }

    msg
}

fn fill_message_queue(machine: &mut Machine, hwnd: HWND) -> bool {
    if !machine.state.user32.messages.is_empty() {
        return true;
    }

    if let Some(msg) = machine.host.get_message(false) {
        machine
            .state
            .user32
            .messages
            .push_back(msg_from_message(msg));
        return true;
    }

    if enqueue_paint_if_needed(machine, hwnd) {
        return true;
    }

    false
}

#[cfg(feature = "x86-emu")]
async fn await_message(machine: &mut Machine, _hwnd: HWND) {
    loop {
        crate::shims::block(machine).await;
        if let Some(msg) = machine.host.get_message(false) {
            machine
                .state
                .user32
                .messages
                .push_back(msg_from_message(msg));
            return;
        }
    }
}

#[cfg(not(feature = "x86-emu"))]
async fn await_message(machine: &mut Machine, _hwnd: HWND) {
    let msg = machine.host.get_message(true).unwrap();
    machine
        .state
        .user32
        .messages
        .push_back(msg_from_message(msg));
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

/// Enqueues a WM_PAINT if the given hwnd (or any hwnd) needs a paint.
fn enqueue_paint_if_needed(machine: &mut Machine, hwnd: HWND) -> bool {
    let hwnd = if hwnd.is_null() {
        match machine.state.user32.windows.iter().find(|w| w.need_paint) {
            Some(w) => w.hwnd,
            None => return false,
        }
    } else {
        if !machine.state.user32.get_window(hwnd).unwrap().need_paint {
            return false;
        }
        hwnd
    };
    machine.state.user32.messages.push_front(MSG {
        hwnd,
        message: WM::PAINT as u32,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    });
    true
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
    assert_eq!(wMsgFilterMin, 0);
    assert_eq!(wMsgFilterMax, 0);
    let lpMsg = lpMsg.unwrap();

    fill_message_queue(machine, hWnd);

    let msg: &MSG = match machine.state.user32.messages.front() {
        Some(msg) => msg,
        None => return false,
    };
    if !hWnd.is_null() && (!msg.hwnd.is_null() && msg.hwnd != hWnd) {
        todo!("message for other window");
    }
    *lpMsg = msg.clone();

    let remove = wRemoveMsg.unwrap();
    if remove.contains(RemoveMsg::PM_REMOVE) {
        machine.state.user32.messages.pop_front();
    }

    true
}

#[win32_derive::dllexport]
pub async fn GetMessageA(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    assert_eq!(wMsgFilterMin, 0);
    assert_eq!(wMsgFilterMax, 0);

    if !fill_message_queue(machine, hWnd) {
        await_message(machine, hWnd).await;
    }

    let msg = lpMsg.unwrap();
    *msg = machine.state.user32.messages.pop_front().unwrap();
    if !hWnd.is_null() && (!msg.hwnd.is_null() && msg.hwnd != hWnd) {
        todo!("message for other window");
    }
    if msg.message == WM::QUIT as u32 {
        return 0;
    }
    return 1;
}

#[win32_derive::dllexport]
pub async fn GetMessageW(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax).await
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
pub async fn DispatchMessageA(machine: &mut Machine, lpMsg: Option<&MSG>) -> u32 {
    let msg = lpMsg.unwrap();
    if msg.hwnd.is_null() {
        // No associated hwnd.
        return 0;
    }
    let wndproc = machine
        .state
        .user32
        .get_window(msg.hwnd)
        .unwrap()
        .wndclass
        .wndproc;
    // TODO: SetWindowLong can change the wndproc.
    machine
        .call_x86(
            wndproc,
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

#[win32_derive::dllexport]
pub async fn DispatchMessageW(machine: &mut Machine, lpMsg: Option<&MSG>) -> u32 {
    DispatchMessageA(machine, lpMsg).await
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(machine: &mut Machine, nExitCode: i32) -> u32 {
    machine.state.user32.messages.push_back(MSG {
        hwnd: HWND::null(),
        message: WM::QUIT as u32,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    });
    0 // unused
}

#[win32_derive::dllexport]
pub fn TranslateAcceleratorW(
    _machine: &mut Machine,
    hWnd: HWND,
    hAccTable: u32,
    lpMsg: Option<&MSG>,
) -> u32 {
    0 // success
}
