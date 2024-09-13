use crate::{winapi::types::*, Machine};

use super::{MSG, WM};

const TRACE_CONTEXT: &'static str = "user32/timer";

#[derive(Debug)]
pub struct Timer {
    id: u32,
    /// Associated window, if any.
    hwnd: HWND,
    /// Milliseconds between ticks.
    period: u32,
    /// Time when timer should fire next.
    next: u32,
    /// Function to call when timer fires.
    func: u32,
}

impl Timer {
    pub fn generate_wm_timer(&mut self, now: u32) -> MSG {
        self.next = now + self.period;
        MSG {
            hwnd: self.hwnd,
            message: WM::TIMER as u32,
            wParam: self.id,
            lParam: self.func,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Timers(Vec<Timer>);

impl Timers {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Find the soonest ready to fire timer matching the given hwnd.
    pub fn find_next(&mut self, hwnd: HWND, now: u32) -> Option<&mut Timer> {
        self.0
            .iter_mut()
            .filter(|t| (hwnd.is_null() || t.hwnd == hwnd) && t.next <= now)
            .min_by_key(|t| t.next)
        // TODO: do we need to consider hwnd == null timers too?
    }

    pub fn soonest(&self) -> u32 {
        self.0.iter().map(|t| t.next).min().unwrap_or(0)
    }
}

#[win32_derive::dllexport]
pub fn KillTimer(machine: &mut Machine, hWnd: HWND, uIDEvent: u32) -> bool {
    let timers = &mut machine.state.user32.timers.0;
    let index = timers
        .iter()
        .position(|t| t.hwnd == hWnd && t.id == uIDEvent);

    if let Some(index) = index {
        timers.swap_remove(index);
        true
    } else {
        false
    }
}

#[win32_derive::dllexport]
pub fn SetTimer(
    machine: &mut Machine,
    hWnd: HWND,
    nIDEvent: u32,
    uElapse: u32,
    lpTimerFunc: u32,
) -> u32 {
    const USER_TIMER_MINIMUM: u32 = 0x0000_000A;
    const USER_TIMER_MAXIMUM: u32 = 0x7FFF_FFFF;
    let uElapse = num_traits::clamp(uElapse, USER_TIMER_MINIMUM, USER_TIMER_MAXIMUM);

    if lpTimerFunc != 0 {
        log::warn!("timer callbacks unimplemented");
    }

    let id = match machine
        .state
        .user32
        .timers
        .0
        .iter_mut()
        .find(|t| t.hwnd == hWnd && t.id == nIDEvent)
    {
        Some(timer) => {
            timer.period = uElapse;
            timer.next = machine.host.ticks() + uElapse;
            timer.func = lpTimerFunc;
            timer.id
        }
        None => {
            let id = machine.state.user32.timers.0.len() as u32 + 1;
            let timer = Timer {
                id,
                hwnd: hWnd,
                period: uElapse,
                next: machine.host.ticks() + uElapse,
                func: lpTimerFunc,
            };
            machine.state.user32.timers.0.push(timer);
            id
        }
    };

    id
}
