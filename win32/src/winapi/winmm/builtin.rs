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
        winapi::{self, calling_convention::*, *},
    };
    use ::memory::Extensions;
    use winapi::winmm::*;
    pub unsafe fn PlaySoundW(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let pszSound = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let hmod = <HMODULE>::from_stack(mem, stack_args + 4u32);
        let fdwSound = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/misc") {
            crate::trace::Record::new(
                winapi::winmm::PlaySoundW_pos,
                "winmm/misc",
                "PlaySoundW",
                &[
                    ("pszSound", &pszSound),
                    ("hmod", &hmod),
                    ("fdwSound", &fdwSound),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::PlaySoundW(machine, pszSound, hmod, fdwSound);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn joyGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uJoyID = <u32>::from_stack(mem, stack_args + 0u32);
        let pjc = <Option<&mut JOYCAPSA>>::from_stack(mem, stack_args + 4u32);
        let cbjc = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/joy") {
            crate::trace::Record::new(
                winapi::winmm::joyGetDevCapsA_pos,
                "winmm/joy",
                "joyGetDevCapsA",
                &[("uJoyID", &uJoyID), ("pjc", &pjc), ("cbjc", &cbjc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::joyGetDevCapsA(machine, uJoyID, pjc, cbjc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn joyGetNumDevs(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("winmm/joy") {
            crate::trace::Record::new(
                winapi::winmm::joyGetNumDevs_pos,
                "winmm/joy",
                "joyGetNumDevs",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::joyGetNumDevs(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn joyGetPosEx(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uJoyID = <u32>::from_stack(mem, stack_args + 0u32);
        let pji = <Option<&mut JOYINFOEX>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("winmm/joy") {
            crate::trace::Record::new(
                winapi::winmm::joyGetPosEx_pos,
                "winmm/joy",
                "joyGetPosEx",
                &[("uJoyID", &uJoyID), ("pji", &pji)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::joyGetPosEx(machine, uJoyID, pji);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mciGetErrorStringA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let mcierr = <u32>::from_stack(mem, stack_args + 0u32);
        let pszText = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let cchText = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/mci") {
            crate::trace::Record::new(
                winapi::winmm::mciGetErrorStringA_pos,
                "winmm/mci",
                "mciGetErrorStringA",
                &[
                    ("mcierr", &mcierr),
                    ("pszText", &pszText),
                    ("cchText", &cchText),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mciGetErrorStringA(machine, mcierr, pszText, cchText);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mciSendCommandA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("winmm/misc") {
            crate::trace::Record::new(
                winapi::winmm::mciSendCommandA_pos,
                "winmm/misc",
                "mciSendCommandA",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mciSendCommandA(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mciSendStringA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let lpstrCommand = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpstrReturnString = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 4u32);
        let hwndCallback = <HWND>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("winmm/mci") {
            crate::trace::Record::new(
                winapi::winmm::mciSendStringA_pos,
                "winmm/mci",
                "mciSendStringA",
                &[
                    ("lpstrCommand", &lpstrCommand),
                    ("lpstrReturnString", &lpstrReturnString),
                    ("hwndCallback", &hwndCallback),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::winmm::mciSendStringA(machine, lpstrCommand, lpstrReturnString, hwndCallback);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutClose(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutClose_pos,
                "winmm/midi",
                "midiOutClose",
                &[("hmo", &hmo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::midiOutClose(machine, hmo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
        let pmoc = <Option<&mut MIDIOUTCAPSA>>::from_stack(mem, stack_args + 4u32);
        let cbmoc = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutGetDevCapsA_pos,
                "winmm/midi",
                "midiOutGetDevCapsA",
                &[
                    ("uDeviceID", &uDeviceID),
                    ("pmoc", &pmoc),
                    ("cbmoc", &cbmoc),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::midiOutGetDevCapsA(machine, uDeviceID, pmoc, cbmoc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutGetNumDevs(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutGetNumDevs_pos,
                "winmm/midi",
                "midiOutGetNumDevs",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::midiOutGetNumDevs(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutOpen(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let phmo = <Option<&mut HMIDIOUT>>::from_stack(mem, stack_args + 0u32);
        let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
        let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
        let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
        let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutOpen_pos,
                "winmm/midi",
                "midiOutOpen",
                &[
                    ("phmo", &phmo),
                    ("uDeviceID", &uDeviceID),
                    ("dwCallback", &dwCallback),
                    ("dwInstance", &dwInstance),
                    ("fdwOpen", &fdwOpen),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::winmm::midiOutOpen(machine, phmo, uDeviceID, dwCallback, dwInstance, fdwOpen);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutReset(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutReset_pos,
                "winmm/midi",
                "midiOutReset",
                &[("hmo", &hmo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::midiOutReset(machine, hmo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutSetVolume(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutSetVolume_pos,
                "winmm/midi",
                "midiOutSetVolume",
                &[("hmo", &hmo), ("dwVolume", &dwVolume)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::midiOutSetVolume(machine, hmo, dwVolume);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn midiOutShortMsg(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
        let dwMsg = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("winmm/midi") {
            crate::trace::Record::new(
                winapi::winmm::midiOutShortMsg_pos,
                "winmm/midi",
                "midiOutShortMsg",
                &[("hmo", &hmo), ("dwMsg", &dwMsg)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::midiOutShortMsg(machine, hmo, dwMsg);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mixerClose(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmx = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/mixer") {
            crate::trace::Record::new(
                winapi::winmm::mixerClose_pos,
                "winmm/mixer",
                "mixerClose",
                &[("hmx", &hmx)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mixerClose(machine, hmx);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mixerGetControlDetailsA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("winmm/mixer") {
            crate::trace::Record::new(
                winapi::winmm::mixerGetControlDetailsA_pos,
                "winmm/mixer",
                "mixerGetControlDetailsA",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mixerGetControlDetailsA(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mixerGetLineControlsA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
        let pmxlc = <u32>::from_stack(mem, stack_args + 4u32);
        let fdwControls = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/mixer") {
            crate::trace::Record::new(
                winapi::winmm::mixerGetLineControlsA_pos,
                "winmm/mixer",
                "mixerGetLineControlsA",
                &[
                    ("hmxobj", &hmxobj),
                    ("pmxlc", &pmxlc),
                    ("fdwControls", &fdwControls),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mixerGetLineControlsA(machine, hmxobj, pmxlc, fdwControls);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mixerGetLineInfoA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmxobj = <HMIXEROBJ>::from_stack(mem, stack_args + 0u32);
        let pmxl = <u32>::from_stack(mem, stack_args + 4u32);
        let fdwInfo = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/mixer") {
            crate::trace::Record::new(
                winapi::winmm::mixerGetLineInfoA_pos,
                "winmm/mixer",
                "mixerGetLineInfoA",
                &[("hmxobj", &hmxobj), ("pmxl", &pmxl), ("fdwInfo", &fdwInfo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mixerGetLineInfoA(machine, hmxobj, pmxl, fdwInfo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mixerOpen(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let phmx = <u32>::from_stack(mem, stack_args + 0u32);
        let uMxId = <u32>::from_stack(mem, stack_args + 4u32);
        let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
        let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
        let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("winmm/mixer") {
            crate::trace::Record::new(
                winapi::winmm::mixerOpen_pos,
                "winmm/mixer",
                "mixerOpen",
                &[
                    ("phmx", &phmx),
                    ("uMxId", &uMxId),
                    ("dwCallback", &dwCallback),
                    ("dwInstance", &dwInstance),
                    ("fdwOpen", &fdwOpen),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::winmm::mixerOpen(machine, phmx, uMxId, dwCallback, dwInstance, fdwOpen);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn mixerSetControlDetails(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
        let pmxcd = <u32>::from_stack(mem, stack_args + 4u32);
        let fdwDetails = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/mixer") {
            crate::trace::Record::new(
                winapi::winmm::mixerSetControlDetails_pos,
                "winmm/mixer",
                "mixerSetControlDetails",
                &[
                    ("hmxobj", &hmxobj),
                    ("pmxcd", &pmxcd),
                    ("fdwDetails", &fdwDetails),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::mixerSetControlDetails(machine, hmxobj, pmxcd, fdwDetails);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn timeBeginPeriod(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/time") {
            crate::trace::Record::new(
                winapi::winmm::timeBeginPeriod_pos,
                "winmm/time",
                "timeBeginPeriod",
                &[("uPeriod", &uPeriod)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::timeBeginPeriod(machine, uPeriod);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn timeEndPeriod(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/time") {
            crate::trace::Record::new(
                winapi::winmm::timeEndPeriod_pos,
                "winmm/time",
                "timeEndPeriod",
                &[("uPeriod", &uPeriod)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::timeEndPeriod(machine, uPeriod);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn timeGetDevCaps(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let ptc = <Option<&mut TIMECAPS>>::from_stack(mem, stack_args + 0u32);
        let cbtc = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("winmm/time") {
            crate::trace::Record::new(
                winapi::winmm::timeGetDevCaps_pos,
                "winmm/time",
                "timeGetDevCaps",
                &[("ptc", &ptc), ("cbtc", &cbtc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::timeGetDevCaps(machine, ptc, cbtc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn timeGetTime(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("winmm/time") {
            crate::trace::Record::new(
                winapi::winmm::timeGetTime_pos,
                "winmm/time",
                "timeGetTime",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::timeGetTime(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn timeKillEvent(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uTimerID = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/time") {
            crate::trace::Record::new(
                winapi::winmm::timeKillEvent_pos,
                "winmm/time",
                "timeKillEvent",
                &[("uTimerID", &uTimerID)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::timeKillEvent(machine, uTimerID);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn timeSetEvent(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uDelay = <u32>::from_stack(mem, stack_args + 0u32);
        let uResolution = <u32>::from_stack(mem, stack_args + 4u32);
        let lpTimeProc = <u32>::from_stack(mem, stack_args + 8u32);
        let dwUser = <u32>::from_stack(mem, stack_args + 12u32);
        let fuEvent = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("winmm/time") {
            crate::trace::Record::new(
                winapi::winmm::timeSetEvent_pos,
                "winmm/time",
                "timeSetEvent",
                &[
                    ("uDelay", &uDelay),
                    ("uResolution", &uResolution),
                    ("lpTimeProc", &lpTimeProc),
                    ("dwUser", &dwUser),
                    ("fuEvent", &fuEvent),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutClose(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutClose_pos,
                "winmm/wave",
                "waveOutClose",
                &[("hwo", &hwo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutClose(machine, hwo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
        let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, stack_args + 4u32);
        let cbwoc = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutGetDevCapsA_pos,
                "winmm/wave",
                "waveOutGetDevCapsA",
                &[
                    ("uDeviceID", &uDeviceID),
                    ("pwoc", &pwoc),
                    ("cbwoc", &cbwoc),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutGetNumDevs_pos,
                "winmm/wave",
                "waveOutGetNumDevs",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetNumDevs(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutGetPosition(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pmmt = <Option<&mut MMTIME>>::from_stack(mem, stack_args + 4u32);
        let cbmmt = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutGetPosition_pos,
                "winmm/wave",
                "waveOutGetPosition",
                &[("hwo", &hwo), ("pmmt", &pmmt), ("cbmmt", &cbmmt)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutGetVolume(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pdwVolume = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutGetVolume_pos,
                "winmm/wave",
                "waveOutGetVolume",
                &[("hwo", &hwo), ("pdwVolume", &pdwVolume)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutGetVolume(machine, hwo, pdwVolume);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutOpen(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, stack_args + 0u32);
        let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
        let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 8u32);
        let dwCallback = <u32>::from_stack(mem, stack_args + 12u32);
        let dwInstance = <u32>::from_stack(mem, stack_args + 16u32);
        let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutOpen_pos,
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
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutOpen(
            machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutPause(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutPause_pos,
                "winmm/wave",
                "waveOutPause",
                &[("hwo", &hwo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutPause(machine, hwo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
        let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutPrepareHeader_pos,
                "winmm/wave",
                "waveOutPrepareHeader",
                &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutReset(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutReset_pos,
                "winmm/wave",
                "waveOutReset",
                &[("hwo", &hwo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutReset(machine, hwo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutRestart(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutRestart_pos,
                "winmm/wave",
                "waveOutRestart",
                &[("hwo", &hwo)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutRestart(machine, hwo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutSetVolume(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutSetVolume_pos,
                "winmm/wave",
                "waveOutSetVolume",
                &[("hwo", &hwo), ("dwVolume", &dwVolume)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutSetVolume(machine, hwo, dwVolume);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutUnprepareHeader(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pwh = <Option<&mut WAVEHDR>>::from_stack(mem, stack_args + 4u32);
        let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutUnprepareHeader_pos,
                "winmm/wave",
                "waveOutUnprepareHeader",
                &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutUnprepareHeader(machine, hwo, pwh, cbwh);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn waveOutWrite(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
        let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
        let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("winmm/wave") {
            crate::trace::Record::new(
                winapi::winmm::waveOutWrite_pos,
                "winmm/wave",
                "waveOutWrite",
                &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
}
const SHIMS: [Shim; 39usize] = [
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
        name: "timeGetDevCaps",
        func: Handler::Sync(wrappers::timeGetDevCaps),
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
