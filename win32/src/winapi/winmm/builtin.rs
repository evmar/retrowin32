#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate::winapi::winmm::{self, *};
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn PlaySoundW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let pszSound = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let hmod = <HMODULE>::from_stack(mem, stack_args + 4u32);
            let fdwSound = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/misc") {
                trace::Record::new(
                    winmm::PlaySoundW_pos,
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
            let result = winmm::PlaySoundW(sys, pszSound, hmod, fdwSound);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn joyGetDevCapsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uJoyID = <u32>::from_stack(mem, stack_args + 0u32);
            let pjc = <Option<&mut JOYCAPSA>>::from_stack(mem, stack_args + 4u32);
            let cbjc = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/joy") {
                trace::Record::new(
                    winmm::joyGetDevCapsA_pos,
                    "winmm/joy",
                    "joyGetDevCapsA",
                    &[("uJoyID", &uJoyID), ("pjc", &pjc), ("cbjc", &cbjc)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::joyGetDevCapsA(sys, uJoyID, pjc, cbjc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn joyGetNumDevs(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/joy") {
                trace::Record::new(winmm::joyGetNumDevs_pos, "winmm/joy", "joyGetNumDevs", &[])
                    .enter()
            } else {
                None
            };
            let result = winmm::joyGetNumDevs(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn joyGetPosEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uJoyID = <u32>::from_stack(mem, stack_args + 0u32);
            let pji = <Option<&mut JOYINFOEX>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("winmm/joy") {
                trace::Record::new(
                    winmm::joyGetPosEx_pos,
                    "winmm/joy",
                    "joyGetPosEx",
                    &[("uJoyID", &uJoyID), ("pji", &pji)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::joyGetPosEx(sys, uJoyID, pji);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mciGetErrorStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let mcierr = <u32>::from_stack(mem, stack_args + 0u32);
            let pszText = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let cchText = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/mci") {
                trace::Record::new(
                    winmm::mciGetErrorStringA_pos,
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
            let result = winmm::mciGetErrorStringA(sys, mcierr, pszText, cchText);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mciSendCommandA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/misc") {
                trace::Record::new(
                    winmm::mciSendCommandA_pos,
                    "winmm/misc",
                    "mciSendCommandA",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::mciSendCommandA(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mciSendStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpstrCommand = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpstrReturnString = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let hwndCallback = <HWND>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("winmm/mci") {
                trace::Record::new(
                    winmm::mciSendStringA_pos,
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
            let result = winmm::mciSendStringA(sys, lpstrCommand, lpstrReturnString, hwndCallback);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiInGetNumDevs(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiInGetNumDevs_pos,
                    "winmm/midi",
                    "midiInGetNumDevs",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::midiInGetNumDevs(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutClose(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutClose_pos,
                    "winmm/midi",
                    "midiOutClose",
                    &[("hmo", &hmo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::midiOutClose(sys, hmo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutGetDevCapsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
            let pmoc = <Option<&mut MIDIOUTCAPSA>>::from_stack(mem, stack_args + 4u32);
            let cbmoc = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutGetDevCapsA_pos,
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
            let result = winmm::midiOutGetDevCapsA(sys, uDeviceID, pmoc, cbmoc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutGetNumDevs(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutGetNumDevs_pos,
                    "winmm/midi",
                    "midiOutGetNumDevs",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::midiOutGetNumDevs(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutOpen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let phmo = <Option<&mut HMIDIOUT>>::from_stack(mem, stack_args + 0u32);
            let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
            let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
            let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
            let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutOpen_pos,
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
            let result = winmm::midiOutOpen(sys, phmo, uDeviceID, dwCallback, dwInstance, fdwOpen);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutReset(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutReset_pos,
                    "winmm/midi",
                    "midiOutReset",
                    &[("hmo", &hmo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::midiOutReset(sys, hmo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutSetVolume(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
            let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutSetVolume_pos,
                    "winmm/midi",
                    "midiOutSetVolume",
                    &[("hmo", &hmo), ("dwVolume", &dwVolume)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::midiOutSetVolume(sys, hmo, dwVolume);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn midiOutShortMsg(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmo = <HMIDIOUT>::from_stack(mem, stack_args + 0u32);
            let dwMsg = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("winmm/midi") {
                trace::Record::new(
                    winmm::midiOutShortMsg_pos,
                    "winmm/midi",
                    "midiOutShortMsg",
                    &[("hmo", &hmo), ("dwMsg", &dwMsg)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::midiOutShortMsg(sys, hmo, dwMsg);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mixerClose(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmx = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/mixer") {
                trace::Record::new(
                    winmm::mixerClose_pos,
                    "winmm/mixer",
                    "mixerClose",
                    &[("hmx", &hmx)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::mixerClose(sys, hmx);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mixerGetControlDetailsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/mixer") {
                trace::Record::new(
                    winmm::mixerGetControlDetailsA_pos,
                    "winmm/mixer",
                    "mixerGetControlDetailsA",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::mixerGetControlDetailsA(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mixerGetLineControlsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
            let pmxlc = <u32>::from_stack(mem, stack_args + 4u32);
            let fdwControls = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/mixer") {
                trace::Record::new(
                    winmm::mixerGetLineControlsA_pos,
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
            let result = winmm::mixerGetLineControlsA(sys, hmxobj, pmxlc, fdwControls);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mixerGetLineInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmxobj = <HMIXEROBJ>::from_stack(mem, stack_args + 0u32);
            let pmxl = <u32>::from_stack(mem, stack_args + 4u32);
            let fdwInfo = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/mixer") {
                trace::Record::new(
                    winmm::mixerGetLineInfoA_pos,
                    "winmm/mixer",
                    "mixerGetLineInfoA",
                    &[("hmxobj", &hmxobj), ("pmxl", &pmxl), ("fdwInfo", &fdwInfo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::mixerGetLineInfoA(sys, hmxobj, pmxl, fdwInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mixerOpen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let phmx = <u32>::from_stack(mem, stack_args + 0u32);
            let uMxId = <u32>::from_stack(mem, stack_args + 4u32);
            let dwCallback = <u32>::from_stack(mem, stack_args + 8u32);
            let dwInstance = <u32>::from_stack(mem, stack_args + 12u32);
            let fdwOpen = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("winmm/mixer") {
                trace::Record::new(
                    winmm::mixerOpen_pos,
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
            let result = winmm::mixerOpen(sys, phmx, uMxId, dwCallback, dwInstance, fdwOpen);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn mixerSetControlDetails(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hmxobj = <u32>::from_stack(mem, stack_args + 0u32);
            let pmxcd = <u32>::from_stack(mem, stack_args + 4u32);
            let fdwDetails = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/mixer") {
                trace::Record::new(
                    winmm::mixerSetControlDetails_pos,
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
            let result = winmm::mixerSetControlDetails(sys, hmxobj, pmxcd, fdwDetails);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn retrowin32_time_thread_main(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn>>> {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(
                    winmm::retrowin32_time_thread_main_pos,
                    "winmm/time",
                    "retrowin32_time_thread_main",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let machine: *mut crate::Machine = sys.machine() as *mut _;
            Box::pin(async move {
                let machine = &mut *machine;
                let result = winmm::retrowin32_time_thread_main(machine).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn retrowin32_wave_thread_main(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn>>> {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::retrowin32_wave_thread_main_pos,
                    "winmm/wave",
                    "retrowin32_wave_thread_main",
                    &[("hwo", &hwo)],
                )
                .enter()
            } else {
                None
            };
            let machine: *mut crate::Machine = sys.machine() as *mut _;
            Box::pin(async move {
                let machine = &mut *machine;
                let result = winmm::retrowin32_wave_thread_main(machine, hwo).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn timeBeginPeriod(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(
                    winmm::timeBeginPeriod_pos,
                    "winmm/time",
                    "timeBeginPeriod",
                    &[("uPeriod", &uPeriod)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::timeBeginPeriod(sys, uPeriod);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn timeEndPeriod(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uPeriod = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(
                    winmm::timeEndPeriod_pos,
                    "winmm/time",
                    "timeEndPeriod",
                    &[("uPeriod", &uPeriod)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::timeEndPeriod(sys, uPeriod);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn timeGetDevCaps(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let ptc = <Option<&mut TIMECAPS>>::from_stack(mem, stack_args + 0u32);
            let cbtc = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(
                    winmm::timeGetDevCaps_pos,
                    "winmm/time",
                    "timeGetDevCaps",
                    &[("ptc", &ptc), ("cbtc", &cbtc)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::timeGetDevCaps(sys, ptc, cbtc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn timeGetTime(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(winmm::timeGetTime_pos, "winmm/time", "timeGetTime", &[]).enter()
            } else {
                None
            };
            let result = winmm::timeGetTime(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn timeKillEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uTimerID = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(
                    winmm::timeKillEvent_pos,
                    "winmm/time",
                    "timeKillEvent",
                    &[("uTimerID", &uTimerID)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::timeKillEvent(sys, uTimerID);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn timeSetEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uDelay = <u32>::from_stack(mem, stack_args + 0u32);
            let uResolution = <u32>::from_stack(mem, stack_args + 4u32);
            let lpTimeProc = <u32>::from_stack(mem, stack_args + 8u32);
            let dwUser = <u32>::from_stack(mem, stack_args + 12u32);
            let fuEvent = <Result<TIME, u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("winmm/time") {
                trace::Record::new(
                    winmm::timeSetEvent_pos,
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
            let result = winmm::timeSetEvent(
                &mut *(sys.machine() as *mut crate::Machine),
                uDelay,
                uResolution,
                lpTimeProc,
                dwUser,
                fuEvent,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutClose(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutClose_pos,
                    "winmm/wave",
                    "waveOutClose",
                    &[("hwo", &hwo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutClose(&mut *(sys.machine() as *mut crate::Machine), hwo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutGetDevCapsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, stack_args + 0u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, stack_args + 4u32);
            let cbwoc = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutGetDevCapsA_pos,
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
            let result = winmm::waveOutGetDevCapsA(sys, uDeviceID, pwoc, cbwoc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutGetNumDevs(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutGetNumDevs_pos,
                    "winmm/wave",
                    "waveOutGetNumDevs",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutGetNumDevs(&mut *(sys.machine() as *mut crate::Machine));
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutGetPosition(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, stack_args + 4u32);
            let cbmmt = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutGetPosition_pos,
                    "winmm/wave",
                    "waveOutGetPosition",
                    &[("hwo", &hwo), ("pmmt", &pmmt), ("cbmmt", &cbmmt)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutGetPosition(
                &mut *(sys.machine() as *mut crate::Machine),
                hwo,
                pmmt,
                cbmmt,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutGetVolume(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pdwVolume = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutGetVolume_pos,
                    "winmm/wave",
                    "waveOutGetVolume",
                    &[("hwo", &hwo), ("pdwVolume", &pdwVolume)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutGetVolume(sys, hwo, pdwVolume);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutOpen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, stack_args + 0u32);
            let uDeviceID = <u32>::from_stack(mem, stack_args + 4u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 8u32);
            let dwCallback = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInstance = <u32>::from_stack(mem, stack_args + 16u32);
            let fdwOpen = <WaveOutOpenFlags>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutOpen_pos,
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
            let result = winmm::waveOutOpen(
                &mut *(sys.machine() as *mut crate::Machine),
                phwo,
                uDeviceID,
                pwfx,
                dwCallback,
                dwInstance,
                fdwOpen,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutPause(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutPause_pos,
                    "winmm/wave",
                    "waveOutPause",
                    &[("hwo", &hwo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutPause(sys, hwo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutPrepareHeader(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutPrepareHeader_pos,
                    "winmm/wave",
                    "waveOutPrepareHeader",
                    &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutPrepareHeader(sys, hwo, pwh, cbwh);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutReset(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutReset_pos,
                    "winmm/wave",
                    "waveOutReset",
                    &[("hwo", &hwo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutReset(sys, hwo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutRestart(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutRestart_pos,
                    "winmm/wave",
                    "waveOutRestart",
                    &[("hwo", &hwo)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutRestart(sys, hwo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutSetVolume(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let dwVolume = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutSetVolume_pos,
                    "winmm/wave",
                    "waveOutSetVolume",
                    &[("hwo", &hwo), ("dwVolume", &dwVolume)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutSetVolume(sys, hwo, dwVolume);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutUnprepareHeader(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <Option<&mut WAVEHDR>>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutUnprepareHeader_pos,
                    "winmm/wave",
                    "waveOutUnprepareHeader",
                    &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
                )
                .enter()
            } else {
                None
            };
            let result = winmm::waveOutUnprepareHeader(sys, hwo, pwh, cbwh);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn waveOutWrite(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, stack_args + 0u32);
            let pwh = <u32>::from_stack(mem, stack_args + 4u32);
            let cbwh = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("winmm/wave") {
                trace::Record::new(
                    winmm::waveOutWrite_pos,
                    "winmm/wave",
                    "waveOutWrite",
                    &[("hwo", &hwo), ("pwh", &pwh), ("cbwh", &cbwh)],
                )
                .enter()
            } else {
                None
            };
            let result =
                winmm::waveOutWrite(&mut *(sys.machine() as *mut crate::Machine), hwo, pwh, cbwh);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 42usize] = [
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
        name: "midiInGetNumDevs",
        func: Handler::Sync(wrappers::midiInGetNumDevs),
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
        name: "retrowin32_time_thread_main",
        func: Handler::Async(wrappers::retrowin32_time_thread_main),
    },
    Shim {
        name: "retrowin32_wave_thread_main",
        func: Handler::Async(wrappers::retrowin32_wave_thread_main),
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
