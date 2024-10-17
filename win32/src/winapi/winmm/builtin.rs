#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        machine::Machine,
        winapi::{self, stack_args::*, types::*},
    };
    use ::memory::Extensions;
    use winapi::winmm::*;
    pub unsafe fn PlaySoundW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let pszSound = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let hmod = <HMODULE>::from_stack(mem, stack_args + 4u32);
        let fdwSound = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/misc") {
            Some(crate::trace::trace_begin(
                "winmm/misc",
                "PlaySoundW",
                &[
                    ("pszSound", &pszSound),
                    ("hmod", &hmod),
                    ("fdwSound", &fdwSound),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::PlaySoundW(machine, pszSound, hmod, fdwSound);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::PlaySoundW_pos.0,
                winapi::winmm::PlaySoundW_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn joyGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uJoyID = <u32>::from_stack(mem, stack_args + 0u32);
        let pjc = <Option<&mut JOYCAPSA>>::from_stack(mem, stack_args + 4u32);
        let cbjc = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/joy") {
            Some(crate::trace::trace_begin(
                "winmm/joy",
                "joyGetDevCapsA",
                &[("uJoyID", &uJoyID), ("pjc", &pjc), ("cbjc", &cbjc)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::joyGetDevCapsA(machine, uJoyID, pjc, cbjc);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::joyGetDevCapsA_pos.0,
                winapi::winmm::joyGetDevCapsA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn joyGetNumDevs(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("winmm/joy") {
            Some(crate::trace::trace_begin("winmm/joy", "joyGetNumDevs", &[]))
        } else {
            None
        };
        let result = winapi::winmm::joyGetNumDevs(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::joyGetNumDevs_pos.0,
                winapi::winmm::joyGetNumDevs_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn joyGetPosEx(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uJoyID = <u32>::from_stack(mem, stack_args + 0u32);
        let pji = <Option<&mut JOYINFOEX>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("winmm/joy") {
            Some(crate::trace::trace_begin(
                "winmm/joy",
                "joyGetPosEx",
                &[("uJoyID", &uJoyID), ("pji", &pji)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::joyGetPosEx(machine, uJoyID, pji);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::joyGetPosEx_pos.0,
                winapi::winmm::joyGetPosEx_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mciGetErrorStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let mcierr = <u32>::from_stack(mem, stack_args + 0u32);
        let pszText = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let cchText = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/mci") {
            Some(crate::trace::trace_begin(
                "winmm/mci",
                "mciGetErrorStringA",
                &[
                    ("mcierr", &mcierr),
                    ("pszText", &pszText),
                    ("cchText", &cchText),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mciGetErrorStringA(machine, mcierr, pszText, cchText);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mciGetErrorStringA_pos.0,
                winapi::winmm::mciGetErrorStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mciSendCommandA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("winmm/misc") {
            Some(crate::trace::trace_begin(
                "winmm/misc",
                "mciSendCommandA",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mciSendCommandA(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mciSendCommandA_pos.0,
                winapi::winmm::mciSendCommandA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mciSendStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpstrCommand = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpstrReturnString = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
        let hwndCallback = <HWND>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("winmm/mci") {
            Some(crate::trace::trace_begin(
                "winmm/mci",
                "mciSendStringA",
                &[
                    ("lpstrCommand", &lpstrCommand),
                    ("lpstrReturnString", &lpstrReturnString),
                    ("hwndCallback", &hwndCallback),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::winmm::mciSendStringA(machine, lpstrCommand, lpstrReturnString, hwndCallback);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mciSendStringA_pos.0,
                winapi::winmm::mciSendStringA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutClose(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutClose",
                &[("hmo", &hmo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::midiOutClose(machine, hmo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutClose_pos.0,
                winapi::winmm::midiOutClose_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
        let pmoc = <Option<&mut MIDIOUTCAPSA>>::from_stack(mem, stack_args + 4u32);
        let cbmoc = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutGetDevCapsA",
                &[
                    ("uDeviceID", &uDeviceID),
                    ("pmoc", &pmoc),
                    ("cbmoc", &cbmoc),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::midiOutGetDevCapsA(machine, uDeviceID, pmoc, cbmoc);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutGetDevCapsA_pos.0,
                winapi::winmm::midiOutGetDevCapsA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutGetNumDevs(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutGetNumDevs",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::winmm::midiOutGetNumDevs(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutGetNumDevs_pos.0,
                winapi::winmm::midiOutGetNumDevs_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutOpen(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let phmo = <Option<&mut HMIDIOUT>>::from_stack(mem, stack_args + 0u32);
        let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
        let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
        let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
        let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutOpen",
                &[
                    ("phmo", &phmo),
                    ("uDeviceID", &uDeviceID),
                    ("dwCallback", &dwCallback),
                    ("dwInstance", &dwInstance),
                    ("fdwOpen", &fdwOpen),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::winmm::midiOutOpen(machine, phmo, uDeviceID, dwCallback, dwInstance, fdwOpen);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutOpen_pos.0,
                winapi::winmm::midiOutOpen_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutReset(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutReset",
                &[("hmo", &hmo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::midiOutReset(machine, hmo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutReset_pos.0,
                winapi::winmm::midiOutReset_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutSetVolume(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutSetVolume",
                &[("hmo", &hmo), ("dwVolume", &dwVolume)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::midiOutSetVolume(machine, hmo, dwVolume);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutSetVolume_pos.0,
                winapi::winmm::midiOutSetVolume_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn midiOutShortMsg(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let dwMsg = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("winmm/midi") {
            Some(crate::trace::trace_begin(
                "winmm/midi",
                "midiOutShortMsg",
                &[("hmo", &hmo), ("dwMsg", &dwMsg)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::midiOutShortMsg(machine, hmo, dwMsg);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::midiOutShortMsg_pos.0,
                winapi::winmm::midiOutShortMsg_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mixerClose(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmx = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/mixer") {
            Some(crate::trace::trace_begin(
                "winmm/mixer",
                "mixerClose",
                &[("hmx", &hmx)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mixerClose(machine, hmx);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mixerClose_pos.0,
                winapi::winmm::mixerClose_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mixerGetControlDetailsA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("winmm/mixer") {
            Some(crate::trace::trace_begin(
                "winmm/mixer",
                "mixerGetControlDetailsA",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mixerGetControlDetailsA(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mixerGetControlDetailsA_pos.0,
                winapi::winmm::mixerGetControlDetailsA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mixerGetLineControlsA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
        let pmxlc = <u32>::from_stack(mem, stack_args + 4u32);
        let fdwControls = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/mixer") {
            Some(crate::trace::trace_begin(
                "winmm/mixer",
                "mixerGetLineControlsA",
                &[
                    ("hmxobj", &hmxobj),
                    ("pmxlc", &pmxlc),
                    ("fdwControls", &fdwControls),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mixerGetLineControlsA(machine, hmxobj, pmxlc, fdwControls);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mixerGetLineControlsA_pos.0,
                winapi::winmm::mixerGetLineControlsA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mixerGetLineInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmxobj = <HMIXEROBJ>::from_stack(mem, stack_args + 0u32);
        let pmxl = <u32>::from_stack(mem, stack_args + 4u32);
        let fdwInfo = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/mixer") {
            Some(crate::trace::trace_begin(
                "winmm/mixer",
                "mixerGetLineInfoA",
                &[("hmxobj", &hmxobj), ("pmxl", &pmxl), ("fdwInfo", &fdwInfo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mixerGetLineInfoA(machine, hmxobj, pmxl, fdwInfo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mixerGetLineInfoA_pos.0,
                winapi::winmm::mixerGetLineInfoA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mixerOpen(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let phmx = <u32>::from_stack(mem, stack_args + 0u32);
        let uMxId = <u32>::from_stack(mem, stack_args + 4u32);
        let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
        let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
        let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("winmm/mixer") {
            Some(crate::trace::trace_begin(
                "winmm/mixer",
                "mixerOpen",
                &[
                    ("phmx", &phmx),
                    ("uMxId", &uMxId),
                    ("dwCallback", &dwCallback),
                    ("dwInstance", &dwInstance),
                    ("fdwOpen", &fdwOpen),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::winmm::mixerOpen(machine, phmx, uMxId, dwCallback, dwInstance, fdwOpen);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mixerOpen_pos.0,
                winapi::winmm::mixerOpen_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn mixerSetControlDetails(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
        let pmxcd = <u32>::from_stack(mem, stack_args + 4u32);
        let fdwDetails = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/mixer") {
            Some(crate::trace::trace_begin(
                "winmm/mixer",
                "mixerSetControlDetails",
                &[
                    ("hmxobj", &hmxobj),
                    ("pmxcd", &pmxcd),
                    ("fdwDetails", &fdwDetails),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::mixerSetControlDetails(machine, hmxobj, pmxcd, fdwDetails);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::mixerSetControlDetails_pos.0,
                winapi::winmm::mixerSetControlDetails_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn timeBeginPeriod(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/time") {
            Some(crate::trace::trace_begin(
                "winmm/time",
                "timeBeginPeriod",
                &[("uPeriod", &uPeriod)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::timeBeginPeriod(machine, uPeriod);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::timeBeginPeriod_pos.0,
                winapi::winmm::timeBeginPeriod_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn timeEndPeriod(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/time") {
            Some(crate::trace::trace_begin(
                "winmm/time",
                "timeEndPeriod",
                &[("uPeriod", &uPeriod)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::timeEndPeriod(machine, uPeriod);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::timeEndPeriod_pos.0,
                winapi::winmm::timeEndPeriod_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn timeGetTime(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("winmm/time") {
            Some(crate::trace::trace_begin("winmm/time", "timeGetTime", &[]))
        } else {
            None
        };
        let result = winapi::winmm::timeGetTime(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::timeGetTime_pos.0,
                winapi::winmm::timeGetTime_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn timeKillEvent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uTimerID = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/time") {
            Some(crate::trace::trace_begin(
                "winmm/time",
                "timeKillEvent",
                &[("uTimerID", &uTimerID)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::timeKillEvent(machine, uTimerID);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::timeKillEvent_pos.0,
                winapi::winmm::timeKillEvent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn timeSetEvent(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uDelay = <u32>::from_stack(mem, stack_args + 0u32);
        let uResolution = <u32>::from_stack(mem, stack_args + 4u32);
        let lpTimeProc = <u32>::from_stack(mem, stack_args + 8u32);
        let dwUser = <u32>::from_stack(mem, stack_args + 12u32);
        let fuEvent = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("winmm/time") {
            Some(crate::trace::trace_begin(
                "winmm/time",
                "timeSetEvent",
                &[
                    ("uDelay", &uDelay),
                    ("uResolution", &uResolution),
                    ("lpTimeProc", &lpTimeProc),
                    ("dwUser", &dwUser),
                    ("fuEvent", &fuEvent),
                ],
            ))
        } else {
            None
        };
        let result =
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::timeSetEvent_pos.0,
                winapi::winmm::timeSetEvent_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutClose(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutClose",
                &[("hwo", &hwo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutClose(machine, hwo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutClose_pos.0,
                winapi::winmm::waveOutClose_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
        let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, stack_args + 4u32);
        let cbwoc = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutGetDevCapsA",
                &[
                    ("uDeviceID", &uDeviceID),
                    ("pwoc", &pwoc),
                    ("cbwoc", &cbwoc),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutGetDevCapsA_pos.0,
                winapi::winmm::waveOutGetDevCapsA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutGetNumDevs",
                &[],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetNumDevs(machine);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutGetNumDevs_pos.0,
                winapi::winmm::waveOutGetNumDevs_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutGetPosition(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pmmt = <Option<&mut MMTIME>>::from_stack(mem, stack_args + 4u32);
        let cbmmt = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutGetPosition",
                &[("hwo", &hwo), ("pmmt", &pmmt), ("cbmmt", &cbmmt)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutGetPosition_pos.0,
                winapi::winmm::waveOutGetPosition_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutGetVolume(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pdwVolume = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutGetVolume",
                &[("hwo", &hwo), ("pdwVolume", &pdwVolume)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetVolume(machine, hwo, pdwVolume);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutGetVolume_pos.0,
                winapi::winmm::waveOutGetVolume_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutOpen(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, stack_args + 0u32);
        let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
        let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 8u32);
        let dwCallback = <u32>::from_stack(mem, stack_args + 12u32);
        let dwInstance = <u32>::from_stack(mem, stack_args + 16u32);
        let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, stack_args + 20u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutOpen",
                &[
                    ("phwo", &phwo),
                    ("uDeviceID", &uDeviceID),
                    ("pwfx", &pwfx),
                    ("dwCallback", &dwCallback),
                    ("dwInstance", &dwInstance),
                    ("fdwOpen", &fdwOpen),
                ],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutOpen(
            machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutOpen_pos.0,
                winapi::winmm::waveOutOpen_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutPause(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutPause",
                &[("hwo", &hwo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutPause(machine, hwo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutPause_pos.0,
                winapi::winmm::waveOutPause_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
        let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutPrepareHeader",
                &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutPrepareHeader_pos.0,
                winapi::winmm::waveOutPrepareHeader_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutReset(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutReset",
                &[("hwo", &hwo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutReset(machine, hwo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutReset_pos.0,
                winapi::winmm::waveOutReset_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutRestart(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutRestart",
                &[("hwo", &hwo)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutRestart(machine, hwo);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutRestart_pos.0,
                winapi::winmm::waveOutRestart_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutSetVolume(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutSetVolume",
                &[("hwo", &hwo), ("dwVolume", &dwVolume)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutSetVolume(machine, hwo, dwVolume);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutSetVolume_pos.0,
                winapi::winmm::waveOutSetVolume_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutUnprepareHeader(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pwh = <Option<&mut WAVEHDR>>::from_stack(mem, stack_args + 4u32);
        let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutUnprepareHeader",
                &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutUnprepareHeader(machine, hwo, pwh, cbwh);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutUnprepareHeader_pos.0,
                winapi::winmm::waveOutUnprepareHeader_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn waveOutWrite(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
        let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("winmm/wave") {
            Some(crate::trace::trace_begin(
                "winmm/wave",
                "waveOutWrite",
                &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
            ))
        } else {
            None
        };
        let result = winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::winmm::waveOutWrite_pos.0,
                winapi::winmm::waveOutWrite_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 38usize] = [
    Shim {
        name: "PlaySoundW",
        func: Handler::Sync(wrappers::PlaySoundW),
    },
    Shim {
        name: "joyGetDevCapsA",
        func: Handler::Sync(wrappers::joyGetDevCapsA),
    },
    Shim {
        name: "joyGetNumDevs",
        func: Handler::Sync(wrappers::joyGetNumDevs),
    },
    Shim {
        name: "joyGetPosEx",
        func: Handler::Sync(wrappers::joyGetPosEx),
    },
    Shim {
        name: "mciGetErrorStringA",
        func: Handler::Sync(wrappers::mciGetErrorStringA),
    },
    Shim {
        name: "mciSendCommandA",
        func: Handler::Sync(wrappers::mciSendCommandA),
    },
    Shim {
        name: "mciSendStringA",
        func: Handler::Sync(wrappers::mciSendStringA),
    },
    Shim {
        name: "midiOutClose",
        func: Handler::Sync(wrappers::midiOutClose),
    },
    Shim {
        name: "midiOutGetDevCapsA",
        func: Handler::Sync(wrappers::midiOutGetDevCapsA),
    },
    Shim {
        name: "midiOutGetNumDevs",
        func: Handler::Sync(wrappers::midiOutGetNumDevs),
    },
    Shim {
        name: "midiOutOpen",
        func: Handler::Sync(wrappers::midiOutOpen),
    },
    Shim {
        name: "midiOutReset",
        func: Handler::Sync(wrappers::midiOutReset),
    },
    Shim {
        name: "midiOutSetVolume",
        func: Handler::Sync(wrappers::midiOutSetVolume),
    },
    Shim {
        name: "midiOutShortMsg",
        func: Handler::Sync(wrappers::midiOutShortMsg),
    },
    Shim {
        name: "mixerClose",
        func: Handler::Sync(wrappers::mixerClose),
    },
    Shim {
        name: "mixerGetControlDetailsA",
        func: Handler::Sync(wrappers::mixerGetControlDetailsA),
    },
    Shim {
        name: "mixerGetLineControlsA",
        func: Handler::Sync(wrappers::mixerGetLineControlsA),
    },
    Shim {
        name: "mixerGetLineInfoA",
        func: Handler::Sync(wrappers::mixerGetLineInfoA),
    },
    Shim {
        name: "mixerOpen",
        func: Handler::Sync(wrappers::mixerOpen),
    },
    Shim {
        name: "mixerSetControlDetails",
        func: Handler::Sync(wrappers::mixerSetControlDetails),
    },
    Shim {
        name: "timeBeginPeriod",
        func: Handler::Sync(wrappers::timeBeginPeriod),
    },
    Shim {
        name: "timeEndPeriod",
        func: Handler::Sync(wrappers::timeEndPeriod),
    },
    Shim {
        name: "timeGetTime",
        func: Handler::Sync(wrappers::timeGetTime),
    },
    Shim {
        name: "timeKillEvent",
        func: Handler::Sync(wrappers::timeKillEvent),
    },
    Shim {
        name: "timeSetEvent",
        func: Handler::Sync(wrappers::timeSetEvent),
    },
    Shim {
        name: "waveOutClose",
        func: Handler::Sync(wrappers::waveOutClose),
    },
    Shim {
        name: "waveOutGetDevCapsA",
        func: Handler::Sync(wrappers::waveOutGetDevCapsA),
    },
    Shim {
        name: "waveOutGetNumDevs",
        func: Handler::Sync(wrappers::waveOutGetNumDevs),
    },
    Shim {
        name: "waveOutGetPosition",
        func: Handler::Sync(wrappers::waveOutGetPosition),
    },
    Shim {
        name: "waveOutGetVolume",
        func: Handler::Sync(wrappers::waveOutGetVolume),
    },
    Shim {
        name: "waveOutOpen",
        func: Handler::Sync(wrappers::waveOutOpen),
    },
    Shim {
        name: "waveOutPause",
        func: Handler::Sync(wrappers::waveOutPause),
    },
    Shim {
        name: "waveOutPrepareHeader",
        func: Handler::Sync(wrappers::waveOutPrepareHeader),
    },
    Shim {
        name: "waveOutReset",
        func: Handler::Sync(wrappers::waveOutReset),
    },
    Shim {
        name: "waveOutRestart",
        func: Handler::Sync(wrappers::waveOutRestart),
    },
    Shim {
        name: "waveOutSetVolume",
        func: Handler::Sync(wrappers::waveOutSetVolume),
    },
    Shim {
        name: "waveOutUnprepareHeader",
        func: Handler::Sync(wrappers::waveOutUnprepareHeader),
    },
    Shim {
        name: "waveOutWrite",
        func: Handler::Sync(wrappers::waveOutWrite),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "winmm.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/winmm.dll"),
};
