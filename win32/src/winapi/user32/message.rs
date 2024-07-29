use crate::{host, winapi::types::*, Machine, MouseButton};
use bitflags::bitflags;

const TRACE_CONTEXT: &'static str = "user32/message";

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
    // The docs mention an lPrivate field, but old versions of MSG don't have the lPrivate field.
    // We don't want to accidentally overwrite adjacent data when putting a MSG into memory, so
    // we leave it out.  See commit 30d1bb3ea9c955b82a724f36490dbe0914af5355.
}
unsafe impl memory::Pod for MSG {}

// Manually implement Debug so we decode the WM_FOO value.
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
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, win32_derive::TryFromEnum)]
pub enum WM {
    NULL = 0,
    CREATE = 0x0001,
    MOVE = 0x0003,
    SIZE = 0x0005,
    ACTIVATE = 0x0006,
    PAINT = 0x000F,
    QUIT = 0x0012,
    ACTIVATEAPP = 0x001C,
    WINDOWPOSCHANGED = 0x0047,
    TIMER = 0x0113,
    MOUSEMOVE = 0x0200,
    LBUTTONDOWN = 0x0201,
    LBUTTONUP = 0x0202,
    LBUTTONDBLCLK = 0x0203,
    RBUTTONDOWN = 0x0204,
    RBUTTONUP = 0x0205,
    RBUTTONDBLCLK = 0x0206,
    MBUTTONDOWN = 0x0207,
    MBUTTONUP = 0x0208,
    MBUTTONDBLCLK = 0x0209,
    USER = 0x0400,
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
    };

    match &message.detail {
        host::MessageDetail::Quit => {
            msg.message = WM::QUIT as u32;
        }
        host::MessageDetail::Mouse(mouse) => {
            msg.message = match (mouse.button, mouse.down) {
                (MouseButton::None, _) => WM::MOUSEMOVE,
                (MouseButton::Left, true) => WM::LBUTTONDOWN,
                (MouseButton::Left, false) => WM::LBUTTONUP,
                (MouseButton::Right, true) => WM::RBUTTONDOWN,
                (MouseButton::Right, false) => WM::RBUTTONUP,
                (MouseButton::Middle, true) => WM::MBUTTONDOWN,
                (MouseButton::Middle, false) => WM::MBUTTONUP,
            } as u32;
            msg.wParam = 0; // TODO:  modifiers
            msg.lParam = (mouse.y << 16) | mouse.x;
            msg.pt_x = mouse.x;
            msg.pt_y = mouse.y;
        }
    }

    msg
}

/// Returns Ok if an event is enqueued.
/// Returns Err(wait) if we need to wait for an event.
fn enqueue_timer_event_if_ready(machine: &mut Machine, hwnd: HWND) -> Result<(), Option<u32>> {
    if machine.state.user32.timers.is_empty() {
        return Err(None);
    }

    let now = machine.host.time();
    if let Some(timer) = machine.state.user32.timers.find_next(hwnd, now) {
        machine
            .state
            .user32
            .messages
            .push_back(timer.generate_wm_timer(now));
        return Ok(());
    }

    let soonest = machine.state.user32.timers.soonest();
    Err(Some(soonest))
}

/// Returns Ok if an event is enqueued.
/// Returns Err(wait) if we need to wait for an event.
fn fill_message_queue(machine: &mut Machine, hwnd: HWND) -> Result<(), Option<u32>> {
    if let Some(msg) = machine.host.get_message() {
        machine
            .state
            .user32
            .messages
            .push_back(msg_from_message(msg));
        return Ok(());
    }

    if enqueue_paint_if_needed(machine, hwnd) {
        return Ok(());
    }

    enqueue_timer_event_if_ready(machine, hwnd)
}

#[cfg(feature = "x86-emu")]
async fn await_message(machine: &mut Machine, _hwnd: HWND, wait: Option<u32>) {
    machine.emu.x86.cpu_mut().block(wait).await;
}

#[cfg(not(feature = "x86-emu"))]
async fn await_message(machine: &mut Machine, _hwnd: HWND, wait: Option<u32>) {
    machine.host.block(wait);
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
        match machine
            .state
            .user32
            .windows
            .iter()
            .find(|w| w.dirty.is_some())
        {
            Some(w) => w.hwnd,
            None => return false,
        }
    } else {
        if !machine
            .state
            .user32
            .windows
            .get(hwnd)
            .unwrap()
            .dirty
            .is_some()
        {
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
    });
    true
}

fn find_message(machine: &mut Machine, hwnd: HWND, min: u32, max: u32) -> Option<usize> {
    machine.state.user32.messages.iter().position(|msg| {
        if !hwnd.is_null() && (!msg.hwnd.is_null() && msg.hwnd != hwnd) {
            return false;
        }
        if min != 0 && max != 0 && (msg.message < min || msg.message > max) {
            return false;
        }
        true
    })
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
    let lpMsg = lpMsg.unwrap();

    let _ = fill_message_queue(machine, hWnd);

    if let Some(index) = find_message(machine, hWnd, wMsgFilterMin, wMsgFilterMax) {
        *lpMsg = machine.state.user32.messages.get(index).unwrap().clone();
        let remove = wRemoveMsg.unwrap();
        if remove.contains(RemoveMsg::PM_REMOVE) {
            machine.state.user32.messages.remove(index);
        }
        return true;
    }
    false
}

#[win32_derive::dllexport]
pub fn PeekMessageW(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMsg: Result<RemoveMsg, u32>,
) -> bool {
    PeekMessageA(
        machine,
        lpMsg,
        hWnd,
        wMsgFilterMin,
        wMsgFilterMax,
        wRemoveMsg,
    )
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

    loop {
        match fill_message_queue(machine, hWnd) {
            Ok(_) => break,
            Err(wait_until) => await_message(machine, hWnd, wait_until).await,
        }
    }

    if let Some(index) = find_message(machine, hWnd, wMsgFilterMin, wMsgFilterMax) {
        let msg = machine.state.user32.messages.get(index).unwrap().clone();
        machine.state.user32.messages.remove(index);
        *lpMsg.unwrap() = msg.clone();
        if msg.message == WM::QUIT as u32 {
            return 0;
        }
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

pub async fn dispatch_message(machine: &mut Machine, msg: &MSG) -> u32 {
    assert!(!msg.hwnd.is_null());
    let wndproc = machine
        .state
        .user32
        .windows
        .get(msg.hwnd)
        .unwrap()
        .wndclass
        .wndproc;
    if wndproc == 0 {
        log::error!("window has no wndproc, skipping message dispatch");
        return 0;
    }
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
    // TODO: copy eax to return value
    0
}

#[win32_derive::dllexport]
pub async fn DispatchMessageA(machine: &mut Machine, lpMsg: Option<&MSG>) -> u32 {
    let msg = lpMsg.unwrap();
    if msg.hwnd.is_null() {
        // No associated hwnd.
        return 0;
    }
    dispatch_message(machine, msg).await;
    0
}

#[win32_derive::dllexport]
pub async fn DispatchMessageW(machine: &mut Machine, lpMsg: Option<&MSG>) -> u32 {
    let msg = lpMsg.unwrap();
    if msg.hwnd.is_null() {
        // No associated hwnd.
        return 0;
    }
    dispatch_message(machine, msg).await;
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
    });
    0 // unused
}

#[win32_derive::dllexport]
pub fn PostMessageW(machine: &mut Machine, hWnd: HWND, Msg: u32, wParam: u32, lParam: u32) -> bool {
    machine.state.user32.messages.push_back(MSG {
        hwnd: hWnd,
        message: Msg,
        wParam,
        lParam,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
    true
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

#[win32_derive::dllexport]
pub async fn SendMessageA(
    machine: &mut Machine,
    hWnd: HWND,
    Msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    let msg = MSG {
        hwnd: hWnd,
        message: Msg.unwrap() as u32,
        wParam,
        lParam,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    dispatch_message(machine, &msg).await
}

#[win32_derive::dllexport]
pub fn MsgWaitForMultipleObjects(
    _machine: &mut Machine,
    nCount: u32,
    pHandles: u32,
    fWaitAll: bool,
    dwMilliseconds: u32,
    dwWakeMask: u32,
) -> u32 {
    // TODO: implement me
    258 // WAIT_TIMEOUT
}
