use bitflags::bitflags;

use crate::{
    host,
    winapi::{stack_args::ToX86, types::*},
    Machine,
};

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
}

fn msg_from_message(message: host::Message) -> MSG {
    match message {
        host::Message::Quit => MSG {
            hwnd: HWND::null(),
            message: WM::QUIT as u32,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
            lPrivate: 0,
        },
    }
}

fn fill_message_queue(machine: &mut Machine, wait: bool) {
    if !machine.state.user32.messages.is_empty() {
        return;
    }

    if let Some(msg) = machine.host.get_message(wait) {
        machine
            .state
            .user32
            .messages
            .push_back(msg_from_message(msg));
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

fn enqueue_paint(machine: &mut Machine) {
    machine.state.user32.messages.push_front(MSG {
        hwnd: HWND::null(),
        message: WM::PAINT as u32,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    });
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

    fill_message_queue(machine, false);
    if machine.state.user32.messages.is_empty() {
        enqueue_paint(machine);
    }

    // TODO: obey HWND.
    let remove = wRemoveMsg.unwrap();
    let msg: &MSG = match machine.state.user32.messages.front() {
        Some(msg) => msg,
        None => return false,
    };
    *lpMsg = msg.clone();

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
) -> i32 {
    assert_eq!(wMsgFilterMin, 0);
    assert_eq!(wMsgFilterMax, 0);

    fill_message_queue(machine, false);
    if machine.state.user32.messages.is_empty() {
        enqueue_paint(machine);
    } else {
        fill_message_queue(machine, true);
    }

    let msg = lpMsg.unwrap();
    *msg = machine.state.user32.messages.pop_front().unwrap();
    if msg.message == WM::QUIT as u32 {
        return 0;
    }
    return 1;
}

#[win32_derive::dllexport]
pub fn GetMessageW(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
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
    let window = &machine.state.user32.windows[msg.hwnd.to_raw() as usize - 1];
    // TODO: SetWindowLong can change the wndproc.
    machine
        .call_x86(
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
) -> bool {
    true // success
}
