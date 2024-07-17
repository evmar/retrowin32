#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[doc = r" Generated code, do not edit."]
use crate::shims;
pub struct Symbol {
    pub ordinal: Option<usize>,
    pub shim: shims::Shim,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
    #[doc = r" Raw bytes of generated .dll."]
    pub raw: &'static [u8],
}
pub mod advapi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::advapi32::*;
        pub unsafe fn RegCloseKey(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            winapi::advapi32::RegCloseKey(machine, hKey).to_raw()
        }
        pub unsafe fn RegCreateKeyExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpSubKey = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let lpClass = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            let dwOptions = <u32>::from_stack(mem, esp + 20u32);
            let samDesired = <u32>::from_stack(mem, esp + 24u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 28u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, esp + 32u32);
            let lpdwDisposition = <Option<&mut u32>>::from_stack(mem, esp + 36u32);
            winapi::advapi32::RegCreateKeyExW(
                machine,
                hKey,
                lpSubKey,
                Reserved,
                lpClass,
                dwOptions,
                samDesired,
                lpSecurityAttributes,
                phkResult,
                lpdwDisposition,
            )
            .to_raw()
        }
        pub unsafe fn RegQueryValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegQueryValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            )
            .to_raw()
        }
        pub unsafe fn RegSetValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <u32>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let cbData = <u32>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegSetValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                cbData,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const RegCloseKey: Shim = Shim {
            name: "RegCloseKey",
            func: impls::RegCloseKey,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegCreateKeyExW: Shim = Shim {
            name: "RegCreateKeyExW",
            func: impls::RegCreateKeyExW,
            stack_consumed: 36u32,
            is_async: false,
        };
        pub const RegQueryValueExW: Shim = Shim {
            name: "RegQueryValueExW",
            func: impls::RegQueryValueExW,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const RegSetValueExW: Shim = Shim {
            name: "RegSetValueExW",
            func: impls::RegSetValueExW,
            stack_consumed: 24u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 4usize] = [
        Symbol {
            ordinal: None,
            shim: shims::RegCloseKey,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegCreateKeyExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegQueryValueExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegSetValueExW,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "advapi32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/advapi32.dll"),
    };
}
pub mod bass {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::bass::*;
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_ChannelGetPosition(machine, mode).to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4).to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            let arg5 = <u32>::from_stack(mem, esp + 20u32);
            winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5).to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_MusicPlay(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2).to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::bass::BASS_Start(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const BASS_ChannelGetPosition: Shim = Shim {
            name: "BASS_ChannelGetPosition",
            func: impls::BASS_ChannelGetPosition,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const BASS_Init: Shim = Shim {
            name: "BASS_Init",
            func: impls::BASS_Init,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const BASS_MusicLoad: Shim = Shim {
            name: "BASS_MusicLoad",
            func: impls::BASS_MusicLoad,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const BASS_MusicPlay: Shim = Shim {
            name: "BASS_MusicPlay",
            func: impls::BASS_MusicPlay,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const BASS_MusicSetPositionScaler: Shim = Shim {
            name: "BASS_MusicSetPositionScaler",
            func: impls::BASS_MusicSetPositionScaler,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const BASS_Start: Shim = Shim {
            name: "BASS_Start",
            func: impls::BASS_Start,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 6usize] = [
        Symbol {
            ordinal: None,
            shim: shims::BASS_ChannelGetPosition,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_Init,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicLoad,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicPlay,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicSetPositionScaler,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_Start,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/bass.dll"),
    };
}
pub mod ddraw {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ddraw::*;
        pub unsafe fn DirectDrawCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let iid = <Option<&GUID>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const DirectDrawCreate: Shim = Shim {
            name: "DirectDrawCreate",
            func: impls::DirectDrawCreate,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DirectDrawCreateClipper: Shim = Shim {
            name: "DirectDrawCreateClipper",
            func: impls::DirectDrawCreateClipper,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DirectDrawCreateEx: Shim = Shim {
            name: "DirectDrawCreateEx",
            func: impls::DirectDrawCreateEx,
            stack_consumed: 16u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 3usize] = [
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreate,
        },
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreateClipper,
        },
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreateEx,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ddraw.dll"),
    };
}
pub mod dsound {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::dsound::*;
        pub unsafe fn DirectSoundCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let ppDS = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, esp + 4u32);
            let lpContext = <u32>::from_stack(mem, esp + 8u32);
            winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const DirectSoundCreate: Shim = Shim {
            name: "DirectSoundCreate",
            func: impls::DirectSoundCreate,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DirectSoundEnumerateA: Shim = Shim {
            name: "DirectSoundEnumerateA",
            func: impls::DirectSoundEnumerateA,
            stack_consumed: 8u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            ordinal: Some(1usize),
            shim: shims::DirectSoundCreate,
        },
        Symbol {
            ordinal: Some(2usize),
            shim: shims::DirectSoundEnumerateA,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/dsound.dll"),
    };
}
pub mod gdi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::gdi32::*;
        pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let x1 = <u32>::from_stack(mem, esp + 28u32);
            let y1 = <u32>::from_stack(mem, esp + 32u32);
            let rop = <u32>::from_stack(mem, esp + 36u32);
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw()
        }
        pub unsafe fn CreateBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nWidth = <u32>::from_stack(mem, esp + 4u32);
            let nHeight = <u32>::from_stack(mem, esp + 8u32);
            let nPlanes = <u32>::from_stack(mem, esp + 12u32);
            let nBitCount = <u32>::from_stack(mem, esp + 16u32);
            let lpBits = <u32>::from_stack(mem, esp + 20u32);
            winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits)
                .to_raw()
        }
        pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let cx = <u32>::from_stack(mem, esp + 8u32);
            let cy = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy).to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, esp + 8u32);
            let usage = <u32>::from_stack(mem, esp + 12u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let hSection = <u32>::from_stack(mem, esp + 20u32);
            let offset = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset)
                .to_raw()
        }
        pub unsafe fn CreateFontA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let cHeight = <i32>::from_stack(mem, esp + 4u32);
            let cWidth = <i32>::from_stack(mem, esp + 8u32);
            let cEscapement = <i32>::from_stack(mem, esp + 12u32);
            let cOrientation = <i32>::from_stack(mem, esp + 16u32);
            let cWeight = <u32>::from_stack(mem, esp + 20u32);
            let bItalic = <u32>::from_stack(mem, esp + 24u32);
            let bUnderline = <u32>::from_stack(mem, esp + 28u32);
            let bStrikeOut = <u32>::from_stack(mem, esp + 32u32);
            let iCharSet = <u32>::from_stack(mem, esp + 36u32);
            let iOutPrecision = <u32>::from_stack(mem, esp + 40u32);
            let iClipPrecision = <u32>::from_stack(mem, esp + 44u32);
            let iQuality = <u32>::from_stack(mem, esp + 48u32);
            let iPitchAndFamily = <u32>::from_stack(mem, esp + 52u32);
            let pszFaceName = <Option<&str>>::from_stack(mem, esp + 56u32);
            winapi::gdi32::CreateFontA(
                machine,
                cHeight,
                cWidth,
                cEscapement,
                cOrientation,
                cWeight,
                bItalic,
                bUnderline,
                bStrikeOut,
                iCharSet,
                iOutPrecision,
                iClipPrecision,
                iQuality,
                iPitchAndFamily,
                pszFaceName,
            )
            .to_raw()
        }
        pub unsafe fn CreatePen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, esp + 4u32);
            let cWidth = <u32>::from_stack(mem, esp + 8u32);
            let color = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreatePen(machine, iStyle, cWidth, color).to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteDC(machine, hdc).to_raw()
        }
        pub unsafe fn DeleteObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteObject(machine, handle).to_raw()
        }
        pub unsafe fn GetDeviceCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDeviceCaps(machine, hdc, index).to_raw()
        }
        pub unsafe fn GetLayout(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetLayout(machine, hdc).to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            let bytes = <u32>::from_stack(mem, esp + 8u32);
            let out = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetObjectA(machine, handle, bytes, out).to_raw()
        }
        pub unsafe fn GetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetPixel(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetStockObject(machine, i).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextMetricsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsA(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn LineTo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::LineTo(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn MoveToEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw()
        }
        pub unsafe fn SetBkColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <u32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn SetBkMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let mode = <i32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkMode(machine, hdc, mode).to_raw()
        }
        pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let w = <u32>::from_stack(mem, esp + 16u32);
            let h = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let StartScan = <u32>::from_stack(mem, esp + 32u32);
            let cLines = <u32>::from_stack(mem, esp + 36u32);
            let lpvBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, esp + 44u32);
            let ColorUse = <u32>::from_stack(mem, esp + 48u32);
            winapi::gdi32::SetDIBitsToDevice(
                machine, hdc, xDest, yDest, w, h, xSrc, ySrc, StartScan, cLines, lpvBits, lpbmi,
                ColorUse,
            )
            .to_raw()
        }
        pub unsafe fn SetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let color = <u32>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetPixel(machine, hdc, x, y, color).to_raw()
        }
        pub unsafe fn SetROP2(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetROP2(machine, hdc, rop2).to_raw()
        }
        pub unsafe fn SetTextColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <u32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetTextColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDest = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let wDest = <u32>::from_stack(mem, esp + 16u32);
            let hDest = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let xSrc = <u32>::from_stack(mem, esp + 28u32);
            let ySrc = <u32>::from_stack(mem, esp + 32u32);
            let wSrc = <u32>::from_stack(mem, esp + 36u32);
            let hSrc = <u32>::from_stack(mem, esp + 40u32);
            let rop = <u32>::from_stack(mem, esp + 44u32);
            winapi::gdi32::StretchBlt(
                machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            )
            .to_raw()
        }
        pub unsafe fn StretchDIBits(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let DestWidth = <u32>::from_stack(mem, esp + 16u32);
            let DestHeight = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let SrcWidth = <u32>::from_stack(mem, esp + 32u32);
            let SrcHeight = <u32>::from_stack(mem, esp + 36u32);
            let lpBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, esp + 44u32);
            let iUsage = <u32>::from_stack(mem, esp + 48u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 52u32);
            winapi::gdi32::StretchDIBits(
                machine, hdc, xDest, yDest, DestWidth, DestHeight, xSrc, ySrc, SrcWidth, SrcHeight,
                lpBits, lpbmi, iUsage, rop,
            )
            .to_raw()
        }
        pub unsafe fn TextOutA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u8>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutA(machine, hdc, x, y, lpString).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const BitBlt: Shim = Shim {
            name: "BitBlt",
            func: impls::BitBlt,
            stack_consumed: 36u32,
            is_async: false,
        };
        pub const CreateBitmap: Shim = Shim {
            name: "CreateBitmap",
            func: impls::CreateBitmap,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const CreateCompatibleBitmap: Shim = Shim {
            name: "CreateCompatibleBitmap",
            func: impls::CreateCompatibleBitmap,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const CreateCompatibleDC: Shim = Shim {
            name: "CreateCompatibleDC",
            func: impls::CreateCompatibleDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const CreateDIBSection: Shim = Shim {
            name: "CreateDIBSection",
            func: impls::CreateDIBSection,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const CreateFontA: Shim = Shim {
            name: "CreateFontA",
            func: impls::CreateFontA,
            stack_consumed: 56u32,
            is_async: false,
        };
        pub const CreatePen: Shim = Shim {
            name: "CreatePen",
            func: impls::CreatePen,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DeleteDC: Shim = Shim {
            name: "DeleteDC",
            func: impls::DeleteDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DeleteObject: Shim = Shim {
            name: "DeleteObject",
            func: impls::DeleteObject,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetDeviceCaps: Shim = Shim {
            name: "GetDeviceCaps",
            func: impls::GetDeviceCaps,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetLayout: Shim = Shim {
            name: "GetLayout",
            func: impls::GetLayout,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetObjectA: Shim = Shim {
            name: "GetObjectA",
            func: impls::GetObjectA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetPixel: Shim = Shim {
            name: "GetPixel",
            func: impls::GetPixel,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetStockObject: Shim = Shim {
            name: "GetStockObject",
            func: impls::GetStockObject,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetTextExtentPoint32A: Shim = Shim {
            name: "GetTextExtentPoint32A",
            func: impls::GetTextExtentPoint32A,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetTextMetricsA: Shim = Shim {
            name: "GetTextMetricsA",
            func: impls::GetTextMetricsA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LineTo: Shim = Shim {
            name: "LineTo",
            func: impls::LineTo,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const MoveToEx: Shim = Shim {
            name: "MoveToEx",
            func: impls::MoveToEx,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SelectObject: Shim = Shim {
            name: "SelectObject",
            func: impls::SelectObject,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetBkColor: Shim = Shim {
            name: "SetBkColor",
            func: impls::SetBkColor,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetBkMode: Shim = Shim {
            name: "SetBkMode",
            func: impls::SetBkMode,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetDIBitsToDevice: Shim = Shim {
            name: "SetDIBitsToDevice",
            func: impls::SetDIBitsToDevice,
            stack_consumed: 48u32,
            is_async: false,
        };
        pub const SetPixel: Shim = Shim {
            name: "SetPixel",
            func: impls::SetPixel,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetROP2: Shim = Shim {
            name: "SetROP2",
            func: impls::SetROP2,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetTextColor: Shim = Shim {
            name: "SetTextColor",
            func: impls::SetTextColor,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const StretchBlt: Shim = Shim {
            name: "StretchBlt",
            func: impls::StretchBlt,
            stack_consumed: 44u32,
            is_async: false,
        };
        pub const StretchDIBits: Shim = Shim {
            name: "StretchDIBits",
            func: impls::StretchDIBits,
            stack_consumed: 52u32,
            is_async: false,
        };
        pub const TextOutA: Shim = Shim {
            name: "TextOutA",
            func: impls::TextOutA,
            stack_consumed: 20u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 28usize] = [
        Symbol {
            ordinal: None,
            shim: shims::BitBlt,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateBitmap,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCompatibleBitmap,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCompatibleDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateDIBSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFontA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreatePen,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDeviceCaps,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLayout,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetObjectA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetPixel,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStockObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTextExtentPoint32A,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTextMetricsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LineTo,
        },
        Symbol {
            ordinal: None,
            shim: shims::MoveToEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::SelectObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetBkColor,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetBkMode,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetDIBitsToDevice,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetPixel,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetROP2,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetTextColor,
        },
        Symbol {
            ordinal: None,
            shim: shims::StretchBlt,
        },
        Symbol {
            ordinal: None,
            shim: shims::StretchDIBits,
        },
        Symbol {
            ordinal: None,
            shim: shims::TextOutA,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/gdi32.dll"),
    };
}
pub mod kernel32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::kernel32::*;
        pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let first = <u32>::from_stack(mem, esp + 4u32);
            let handler = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler).to_raw()
        }
        pub unsafe fn CloseHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hObject = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::CloseHandle(machine, hObject).to_raw()
        }
        pub unsafe fn CreateEventA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, esp + 4u32);
            let bManualReset = <bool>::from_stack(mem, esp + 8u32);
            let bInitialState = <bool>::from_stack(mem, esp + 12u32);
            let lpName = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
            winapi::kernel32::CreateFileA(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
            winapi::kernel32::CreateFileW(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpThreadAttributes = <u32>::from_stack(mem, esp + 4u32);
            let dwStackSize = <u32>::from_stack(mem, esp + 8u32);
            let lpStartAddress = <u32>::from_stack(mem, esp + 12u32);
            let lpParameter = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationFlags = <u32>::from_stack(mem, esp + 20u32);
            let lpThreadId = <u32>::from_stack(mem, esp + 24u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::kernel32::CreateThread(
                        machine,
                        lpThreadAttributes,
                        dwStackSize,
                        lpStartAddress,
                        lpParameter,
                        dwCreationFlags,
                        lpThreadId,
                    )
                    .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 24u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::CreateThread(
                    machine,
                    lpThreadAttributes,
                    dwStackSize,
                    lpStartAddress,
                    lpParameter,
                    dwCreationFlags,
                    lpThreadId
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn DeleteFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteFileA(machine, lpFileName).to_raw()
        }
        pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule).to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uExitCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ExitProcess(machine, uExitCode).to_raw()
        }
        pub unsafe fn FindClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FindClose(machine, hFindFile).to_raw()
        }
        pub unsafe fn FindFirstFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData).to_raw()
        }
        pub unsafe fn FindNextFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData).to_raw()
        }
        pub unsafe fn FindResourceA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <u32>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FindResourceW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <u32>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FormatMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
            winapi::kernel32::FormatMessageW(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _penv = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw()
        }
        pub unsafe fn FreeLibrary(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeLibrary(machine, hLibModule).to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetACP(machine).to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _CodePage = <u32>::from_stack(mem, esp + 4u32);
            let _lpCPInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineA(machine).to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineW(machine).to_raw()
        }
        pub unsafe fn GetConsoleMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode).to_raw()
        }
        pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleScreenBufferInfo(
                machine,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            )
            .to_raw()
        }
        pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer).to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcessId(machine).to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThread(machine).to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThreadId(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStrings(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableW(machine, name, buf).to_raw()
        }
        pub unsafe fn GetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileAttributesA(machine, lpFileName).to_raw()
        }
        pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation).to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileType(machine, hFile).to_raw()
        }
        pub unsafe fn GetFullPathNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameA(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetFullPathNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameW(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetLastError(machine).to_raw()
        }
        pub unsafe fn GetLocalTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetLocalTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let _lpFilename = <u32>::from_stack(mem, esp + 8u32);
            let _nSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <u32>::from_stack(mem, esp + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetPrivateProfileIntW(
                machine, lpAppName, lpKeyName, nDefault, lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 24u32);
            winapi::kernel32::GetPrivateProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetProcessHeap(machine).to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _time = <Option<&mut FILETIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTimeAsFileTime(machine, _time).to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetTickCount(machine).to_raw()
        }
        pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation).to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetVersion(machine).to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpVersionInformation = <Option<&mut OSVERSIONINFO>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFree(machine, hMem).to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, esp + 8u32);
            let dwBytes = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, esp + 4u32);
            let dwInitialSize = <u32>::from_stack(mem, esp + 8u32);
            let dwMaximumSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize).to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::HeapDestroy(machine, hHeap).to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            let dwBytes = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw()
        }
        pub unsafe fn HeapSetInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, esp + 4u32);
            let HeapInformationClass = <u32>::from_stack(mem, esp + 8u32);
            let HeapInformation = <u32>::from_stack(mem, esp + 12u32);
            let HeapInformationLength = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapSetInformation(
                machine,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            )
            .to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::InitOnceBeginInitialize(
                machine, lpInitOnce, dwFlags, fPending, lpContext,
            )
            .to_raw()
        }
        pub unsafe fn InitOnceComplete(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpContext = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext).to_raw()
        }
        pub unsafe fn InitializeCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                lpCriticalSection,
                dwSpinCount,
            )
            .to_raw()
        }
        pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            let flags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitializeCriticalSectionEx(
                machine,
                lpCriticalSection,
                dwSpinCount,
                flags,
            )
            .to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InterlockedIncrement(machine, addend).to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadReadPtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadWritePtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::IsDebuggerPresent(machine).to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LoadLibraryA(machine, filename).to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let hFile = <HFILE>::from_stack(mem, esp + 8u32);
            let dwFlags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw()
        }
        pub unsafe fn LoadResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <u32>::from_stack(mem, esp + 4u32);
            let hResInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LoadResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn LocalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn LocalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LocalFree(machine, hMem).to_raw()
        }
        pub unsafe fn LockResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hResData = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LockResource(machine, hResData).to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMultiByteStr = <u32>::from_stack(mem, esp + 12u32);
            let cbMultiByte = <i32>::from_stack(mem, esp + 16u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 20u32);
            winapi::kernel32::MultiByteToWideChar(
                machine,
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            )
            .to_raw()
        }
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::NtCurrentTeb(machine).to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::OutputDebugStringA(machine, msg).to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPerformanceCount = <Option<&mut LARGE_INTEGER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount).to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency).to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped)
                .to_raw()
        }
        pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, esp + 4u32);
            let _add = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add).to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEvent(machine, hEvent).to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lDistanceToMove = <i32>::from_stack(mem, esp + 8u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, esp + 12u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            )
            .to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uNumber = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetHandleCount(machine, uNumber).to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwErrCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetLastError(machine, dwErrCode).to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let dwPriorityClass = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass).to_raw()
        }
        pub unsafe fn SetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            let hHandle = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle).to_raw()
        }
        pub unsafe fn SetThreadDescription(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription).to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let nPriority = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadPriority(machine, hThread, nPriority).to_raw()
        }
        pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes).to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw()
        }
        pub unsafe fn Sleep(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::kernel32::Sleep(machine, dwMilliseconds).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 4u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::Sleep(machine, dwMilliseconds));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::TlsAlloc(machine).to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            let lpTlsValue = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw()
        }
        pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, esp + 12u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, flAllocationType, flProtec)
                .to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let dwFreeType = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flNewProtect = <u32>::from_stack(mem, esp + 12u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            )
            .to_raw()
        }
        pub unsafe fn VirtualQuery(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, esp + 8u32);
            let dwLength = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength).to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHandle = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds).to_raw()
        }
        pub unsafe fn WriteConsoleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let _lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            )
            .to_raw()
        }
        pub unsafe fn lstrcmpiA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyW(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrlenA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenA(machine, lpString).to_raw()
        }
        pub unsafe fn lstrlenW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenW(machine, lpString).to_raw()
        }
        pub unsafe fn retrowin32_main(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::kernel32::retrowin32_main(machine, entry_point).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 4u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::retrowin32_main(machine, entry_point));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn retrowin32_thread_main(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            let param = <u32>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::kernel32::retrowin32_thread_main(machine, entry_point, param).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 8u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::retrowin32_thread_main(
                    machine,
                    entry_point,
                    param
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const AcquireSRWLockExclusive: Shim = Shim {
            name: "AcquireSRWLockExclusive",
            func: impls::AcquireSRWLockExclusive,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const AcquireSRWLockShared: Shim = Shim {
            name: "AcquireSRWLockShared",
            func: impls::AcquireSRWLockShared,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const AddVectoredExceptionHandler: Shim = Shim {
            name: "AddVectoredExceptionHandler",
            func: impls::AddVectoredExceptionHandler,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CloseHandle: Shim = Shim {
            name: "CloseHandle",
            func: impls::CloseHandle,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const CreateEventA: Shim = Shim {
            name: "CreateEventA",
            func: impls::CreateEventA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const CreateFileA: Shim = Shim {
            name: "CreateFileA",
            func: impls::CreateFileA,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const CreateFileW: Shim = Shim {
            name: "CreateFileW",
            func: impls::CreateFileW,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const CreateThread: Shim = Shim {
            name: "CreateThread",
            func: impls::CreateThread,
            stack_consumed: 24u32,
            is_async: true,
        };
        pub const DeleteCriticalSection: Shim = Shim {
            name: "DeleteCriticalSection",
            func: impls::DeleteCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DeleteFileA: Shim = Shim {
            name: "DeleteFileA",
            func: impls::DeleteFileA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DisableThreadLibraryCalls: Shim = Shim {
            name: "DisableThreadLibraryCalls",
            func: impls::DisableThreadLibraryCalls,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const EnterCriticalSection: Shim = Shim {
            name: "EnterCriticalSection",
            func: impls::EnterCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ExitProcess: Shim = Shim {
            name: "ExitProcess",
            func: impls::ExitProcess,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const FindClose: Shim = Shim {
            name: "FindClose",
            func: impls::FindClose,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const FindFirstFileA: Shim = Shim {
            name: "FindFirstFileA",
            func: impls::FindFirstFileA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FindNextFileA: Shim = Shim {
            name: "FindNextFileA",
            func: impls::FindNextFileA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FindResourceA: Shim = Shim {
            name: "FindResourceA",
            func: impls::FindResourceA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const FindResourceW: Shim = Shim {
            name: "FindResourceW",
            func: impls::FindResourceW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const FormatMessageW: Shim = Shim {
            name: "FormatMessageW",
            func: impls::FormatMessageW,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const FreeEnvironmentStringsA: Shim = Shim {
            name: "FreeEnvironmentStringsA",
            func: impls::FreeEnvironmentStringsA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const FreeLibrary: Shim = Shim {
            name: "FreeLibrary",
            func: impls::FreeLibrary,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetACP: Shim = Shim {
            name: "GetACP",
            func: impls::GetACP,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCPInfo: Shim = Shim {
            name: "GetCPInfo",
            func: impls::GetCPInfo,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetCommandLineA: Shim = Shim {
            name: "GetCommandLineA",
            func: impls::GetCommandLineA,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCommandLineW: Shim = Shim {
            name: "GetCommandLineW",
            func: impls::GetCommandLineW,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetConsoleMode: Shim = Shim {
            name: "GetConsoleMode",
            func: impls::GetConsoleMode,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetConsoleScreenBufferInfo: Shim = Shim {
            name: "GetConsoleScreenBufferInfo",
            func: impls::GetConsoleScreenBufferInfo,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetCurrentDirectoryA: Shim = Shim {
            name: "GetCurrentDirectoryA",
            func: impls::GetCurrentDirectoryA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetCurrentProcessId: Shim = Shim {
            name: "GetCurrentProcessId",
            func: impls::GetCurrentProcessId,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCurrentThread: Shim = Shim {
            name: "GetCurrentThread",
            func: impls::GetCurrentThread,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCurrentThreadId: Shim = Shim {
            name: "GetCurrentThreadId",
            func: impls::GetCurrentThreadId,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetEnvironmentStrings: Shim = Shim {
            name: "GetEnvironmentStrings",
            func: impls::GetEnvironmentStrings,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetEnvironmentStringsW: Shim = Shim {
            name: "GetEnvironmentStringsW",
            func: impls::GetEnvironmentStringsW,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetEnvironmentVariableA: Shim = Shim {
            name: "GetEnvironmentVariableA",
            func: impls::GetEnvironmentVariableA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetEnvironmentVariableW: Shim = Shim {
            name: "GetEnvironmentVariableW",
            func: impls::GetEnvironmentVariableW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetFileAttributesA: Shim = Shim {
            name: "GetFileAttributesA",
            func: impls::GetFileAttributesA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetFileInformationByHandle: Shim = Shim {
            name: "GetFileInformationByHandle",
            func: impls::GetFileInformationByHandle,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetFileType: Shim = Shim {
            name: "GetFileType",
            func: impls::GetFileType,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetFullPathNameA: Shim = Shim {
            name: "GetFullPathNameA",
            func: impls::GetFullPathNameA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetFullPathNameW: Shim = Shim {
            name: "GetFullPathNameW",
            func: impls::GetFullPathNameW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetLastError: Shim = Shim {
            name: "GetLastError",
            func: impls::GetLastError,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetLocalTime: Shim = Shim {
            name: "GetLocalTime",
            func: impls::GetLocalTime,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetModuleFileNameA: Shim = Shim {
            name: "GetModuleFileNameA",
            func: impls::GetModuleFileNameA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetModuleFileNameW: Shim = Shim {
            name: "GetModuleFileNameW",
            func: impls::GetModuleFileNameW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetModuleHandleA: Shim = Shim {
            name: "GetModuleHandleA",
            func: impls::GetModuleHandleA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetModuleHandleExW: Shim = Shim {
            name: "GetModuleHandleExW",
            func: impls::GetModuleHandleExW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetModuleHandleW: Shim = Shim {
            name: "GetModuleHandleW",
            func: impls::GetModuleHandleW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetPrivateProfileIntW: Shim = Shim {
            name: "GetPrivateProfileIntW",
            func: impls::GetPrivateProfileIntW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetPrivateProfileStringW: Shim = Shim {
            name: "GetPrivateProfileStringW",
            func: impls::GetPrivateProfileStringW,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const GetProcAddress: Shim = Shim {
            name: "GetProcAddress",
            func: impls::GetProcAddress,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetProcessHeap: Shim = Shim {
            name: "GetProcessHeap",
            func: impls::GetProcessHeap,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetStartupInfoA: Shim = Shim {
            name: "GetStartupInfoA",
            func: impls::GetStartupInfoA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetStartupInfoW: Shim = Shim {
            name: "GetStartupInfoW",
            func: impls::GetStartupInfoW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetStdHandle: Shim = Shim {
            name: "GetStdHandle",
            func: impls::GetStdHandle,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetSystemTimeAsFileTime: Shim = Shim {
            name: "GetSystemTimeAsFileTime",
            func: impls::GetSystemTimeAsFileTime,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetTickCount: Shim = Shim {
            name: "GetTickCount",
            func: impls::GetTickCount,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetTimeZoneInformation: Shim = Shim {
            name: "GetTimeZoneInformation",
            func: impls::GetTimeZoneInformation,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetVersion: Shim = Shim {
            name: "GetVersion",
            func: impls::GetVersion,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetVersionExA: Shim = Shim {
            name: "GetVersionExA",
            func: impls::GetVersionExA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GlobalAlloc: Shim = Shim {
            name: "GlobalAlloc",
            func: impls::GlobalAlloc,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GlobalFree: Shim = Shim {
            name: "GlobalFree",
            func: impls::GlobalFree,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const HeapAlloc: Shim = Shim {
            name: "HeapAlloc",
            func: impls::HeapAlloc,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapCreate: Shim = Shim {
            name: "HeapCreate",
            func: impls::HeapCreate,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapDestroy: Shim = Shim {
            name: "HeapDestroy",
            func: impls::HeapDestroy,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const HeapFree: Shim = Shim {
            name: "HeapFree",
            func: impls::HeapFree,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapReAlloc: Shim = Shim {
            name: "HeapReAlloc",
            func: impls::HeapReAlloc,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapSetInformation: Shim = Shim {
            name: "HeapSetInformation",
            func: impls::HeapSetInformation,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapSize: Shim = Shim {
            name: "HeapSize",
            func: impls::HeapSize,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitOnceBeginInitialize: Shim = Shim {
            name: "InitOnceBeginInitialize",
            func: impls::InitOnceBeginInitialize,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const InitOnceComplete: Shim = Shim {
            name: "InitOnceComplete",
            func: impls::InitOnceComplete,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitializeCriticalSection: Shim = Shim {
            name: "InitializeCriticalSection",
            func: impls::InitializeCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const InitializeCriticalSectionAndSpinCount: Shim = Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: impls::InitializeCriticalSectionAndSpinCount,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const InitializeCriticalSectionEx: Shim = Shim {
            name: "InitializeCriticalSectionEx",
            func: impls::InitializeCriticalSectionEx,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitializeSListHead: Shim = Shim {
            name: "InitializeSListHead",
            func: impls::InitializeSListHead,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const InterlockedIncrement: Shim = Shim {
            name: "InterlockedIncrement",
            func: impls::InterlockedIncrement,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsBadReadPtr: Shim = Shim {
            name: "IsBadReadPtr",
            func: impls::IsBadReadPtr,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsBadWritePtr: Shim = Shim {
            name: "IsBadWritePtr",
            func: impls::IsBadWritePtr,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsDebuggerPresent: Shim = Shim {
            name: "IsDebuggerPresent",
            func: impls::IsDebuggerPresent,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const IsProcessorFeaturePresent: Shim = Shim {
            name: "IsProcessorFeaturePresent",
            func: impls::IsProcessorFeaturePresent,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsValidCodePage: Shim = Shim {
            name: "IsValidCodePage",
            func: impls::IsValidCodePage,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LeaveCriticalSection: Shim = Shim {
            name: "LeaveCriticalSection",
            func: impls::LeaveCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LoadLibraryA: Shim = Shim {
            name: "LoadLibraryA",
            func: impls::LoadLibraryA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LoadLibraryExW: Shim = Shim {
            name: "LoadLibraryExW",
            func: impls::LoadLibraryExW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const LoadResource: Shim = Shim {
            name: "LoadResource",
            func: impls::LoadResource,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LocalAlloc: Shim = Shim {
            name: "LocalAlloc",
            func: impls::LocalAlloc,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LocalFree: Shim = Shim {
            name: "LocalFree",
            func: impls::LocalFree,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LockResource: Shim = Shim {
            name: "LockResource",
            func: impls::LockResource,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const MultiByteToWideChar: Shim = Shim {
            name: "MultiByteToWideChar",
            func: impls::MultiByteToWideChar,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const NtCurrentTeb: Shim = Shim {
            name: "NtCurrentTeb",
            func: impls::NtCurrentTeb,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const OutputDebugStringA: Shim = Shim {
            name: "OutputDebugStringA",
            func: impls::OutputDebugStringA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const QueryPerformanceCounter: Shim = Shim {
            name: "QueryPerformanceCounter",
            func: impls::QueryPerformanceCounter,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const QueryPerformanceFrequency: Shim = Shim {
            name: "QueryPerformanceFrequency",
            func: impls::QueryPerformanceFrequency,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ReadFile: Shim = Shim {
            name: "ReadFile",
            func: impls::ReadFile,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const ReleaseSRWLockExclusive: Shim = Shim {
            name: "ReleaseSRWLockExclusive",
            func: impls::ReleaseSRWLockExclusive,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ReleaseSRWLockShared: Shim = Shim {
            name: "ReleaseSRWLockShared",
            func: impls::ReleaseSRWLockShared,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetConsoleCtrlHandler: Shim = Shim {
            name: "SetConsoleCtrlHandler",
            func: impls::SetConsoleCtrlHandler,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetEvent: Shim = Shim {
            name: "SetEvent",
            func: impls::SetEvent,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetFilePointer: Shim = Shim {
            name: "SetFilePointer",
            func: impls::SetFilePointer,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetHandleCount: Shim = Shim {
            name: "SetHandleCount",
            func: impls::SetHandleCount,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetLastError: Shim = Shim {
            name: "SetLastError",
            func: impls::SetLastError,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetPriorityClass: Shim = Shim {
            name: "SetPriorityClass",
            func: impls::SetPriorityClass,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetStdHandle: Shim = Shim {
            name: "SetStdHandle",
            func: impls::SetStdHandle,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetThreadDescription: Shim = Shim {
            name: "SetThreadDescription",
            func: impls::SetThreadDescription,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetThreadPriority: Shim = Shim {
            name: "SetThreadPriority",
            func: impls::SetThreadPriority,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetThreadStackGuarantee: Shim = Shim {
            name: "SetThreadStackGuarantee",
            func: impls::SetThreadStackGuarantee,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetUnhandledExceptionFilter: Shim = Shim {
            name: "SetUnhandledExceptionFilter",
            func: impls::SetUnhandledExceptionFilter,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const Sleep: Shim = Shim {
            name: "Sleep",
            func: impls::Sleep,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const TlsAlloc: Shim = Shim {
            name: "TlsAlloc",
            func: impls::TlsAlloc,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const TlsFree: Shim = Shim {
            name: "TlsFree",
            func: impls::TlsFree,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const TlsGetValue: Shim = Shim {
            name: "TlsGetValue",
            func: impls::TlsGetValue,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const TlsSetValue: Shim = Shim {
            name: "TlsSetValue",
            func: impls::TlsSetValue,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const TryAcquireSRWLockExclusive: Shim = Shim {
            name: "TryAcquireSRWLockExclusive",
            func: impls::TryAcquireSRWLockExclusive,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const UnhandledExceptionFilter: Shim = Shim {
            name: "UnhandledExceptionFilter",
            func: impls::UnhandledExceptionFilter,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const VirtualAlloc: Shim = Shim {
            name: "VirtualAlloc",
            func: impls::VirtualAlloc,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const VirtualFree: Shim = Shim {
            name: "VirtualFree",
            func: impls::VirtualFree,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const VirtualProtect: Shim = Shim {
            name: "VirtualProtect",
            func: impls::VirtualProtect,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const VirtualQuery: Shim = Shim {
            name: "VirtualQuery",
            func: impls::VirtualQuery,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const WaitForSingleObject: Shim = Shim {
            name: "WaitForSingleObject",
            func: impls::WaitForSingleObject,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const WriteConsoleA: Shim = Shim {
            name: "WriteConsoleA",
            func: impls::WriteConsoleA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const WriteConsoleW: Shim = Shim {
            name: "WriteConsoleW",
            func: impls::WriteConsoleW,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const WriteFile: Shim = Shim {
            name: "WriteFile",
            func: impls::WriteFile,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const lstrcmpiA: Shim = Shim {
            name: "lstrcmpiA",
            func: impls::lstrcmpiA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const lstrcpyA: Shim = Shim {
            name: "lstrcpyA",
            func: impls::lstrcpyA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const lstrcpyW: Shim = Shim {
            name: "lstrcpyW",
            func: impls::lstrcpyW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const lstrlenA: Shim = Shim {
            name: "lstrlenA",
            func: impls::lstrlenA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const lstrlenW: Shim = Shim {
            name: "lstrlenW",
            func: impls::lstrlenW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const retrowin32_main: Shim = Shim {
            name: "retrowin32_main",
            func: impls::retrowin32_main,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const retrowin32_thread_main: Shim = Shim {
            name: "retrowin32_thread_main",
            func: impls::retrowin32_thread_main,
            stack_consumed: 8u32,
            is_async: true,
        };
    }
    const EXPORTS: [Symbol; 128usize] = [
        Symbol {
            ordinal: None,
            shim: shims::AcquireSRWLockExclusive,
        },
        Symbol {
            ordinal: None,
            shim: shims::AcquireSRWLockShared,
        },
        Symbol {
            ordinal: None,
            shim: shims::AddVectoredExceptionHandler,
        },
        Symbol {
            ordinal: None,
            shim: shims::CloseHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateEventA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFileW,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateThread,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DisableThreadLibraryCalls,
        },
        Symbol {
            ordinal: None,
            shim: shims::EnterCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::ExitProcess,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindClose,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindFirstFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindNextFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindResourceA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindResourceW,
        },
        Symbol {
            ordinal: None,
            shim: shims::FormatMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::FreeEnvironmentStringsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FreeLibrary,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetACP,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCPInfo,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCommandLineA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCommandLineW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetConsoleMode,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetConsoleScreenBufferInfo,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentDirectoryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentProcessId,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentThread,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentThreadId,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentStrings,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentStringsW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentVariableA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentVariableW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileAttributesA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileInformationByHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileType,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFullPathNameA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFullPathNameW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLastError,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLocalTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleFileNameA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleFileNameW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetPrivateProfileIntW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetPrivateProfileStringW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProcAddress,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProcessHeap,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStartupInfoA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStartupInfoW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStdHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemTimeAsFileTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTickCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTimeZoneInformation,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetVersion,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetVersionExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapCreate,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapDestroy,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapReAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapSetInformation,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapSize,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitOnceBeginInitialize,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitOnceComplete,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSectionAndSpinCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSectionEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeSListHead,
        },
        Symbol {
            ordinal: None,
            shim: shims::InterlockedIncrement,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsBadReadPtr,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsBadWritePtr,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsDebuggerPresent,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsProcessorFeaturePresent,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsValidCodePage,
        },
        Symbol {
            ordinal: None,
            shim: shims::LeaveCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadLibraryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadLibraryExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadResource,
        },
        Symbol {
            ordinal: None,
            shim: shims::LocalAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::LocalFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::LockResource,
        },
        Symbol {
            ordinal: None,
            shim: shims::MultiByteToWideChar,
        },
        Symbol {
            ordinal: None,
            shim: shims::NtCurrentTeb,
        },
        Symbol {
            ordinal: None,
            shim: shims::OutputDebugStringA,
        },
        Symbol {
            ordinal: None,
            shim: shims::QueryPerformanceCounter,
        },
        Symbol {
            ordinal: None,
            shim: shims::QueryPerformanceFrequency,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReadFile,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseSRWLockExclusive,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseSRWLockShared,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetConsoleCtrlHandler,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetEvent,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFilePointer,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetHandleCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetLastError,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetPriorityClass,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetStdHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadDescription,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadPriority,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadStackGuarantee,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetUnhandledExceptionFilter,
        },
        Symbol {
            ordinal: None,
            shim: shims::Sleep,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsGetValue,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsSetValue,
        },
        Symbol {
            ordinal: None,
            shim: shims::TryAcquireSRWLockExclusive,
        },
        Symbol {
            ordinal: None,
            shim: shims::UnhandledExceptionFilter,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualProtect,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualQuery,
        },
        Symbol {
            ordinal: None,
            shim: shims::WaitForSingleObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteConsoleA,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteConsoleW,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteFile,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrcmpiA,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrcpyA,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrcpyW,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrlenA,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrlenW,
        },
        Symbol {
            ordinal: None,
            shim: shims::retrowin32_main,
        },
        Symbol {
            ordinal: None,
            shim: shims::retrowin32_thread_main,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/kernel32.dll"),
    };
}
pub mod ntdll {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ntdll::*;
        pub unsafe fn NtReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let FileHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let Event = <u32>::from_stack(mem, esp + 8u32);
            let ApcRoutine = <u32>::from_stack(mem, esp + 12u32);
            let ApcContext = <u32>::from_stack(mem, esp + 16u32);
            let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, esp + 20u32);
            let Buffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 24u32);
            let ByteOffset = <Option<&mut u64>>::from_stack(mem, esp + 32u32);
            let Key = <u32>::from_stack(mem, esp + 36u32);
            winapi::ntdll::NtReadFile(
                machine,
                FileHandle,
                Event,
                ApcRoutine,
                ApcContext,
                IoStatusBlock,
                Buffer,
                ByteOffset,
                Key,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const NtReadFile: Shim = Shim {
            name: "NtReadFile",
            func: impls::NtReadFile,
            stack_consumed: 36u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: None,
        shim: shims::NtReadFile,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ntdll.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ntdll.dll"),
    };
}
pub mod ole32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ole32::*;
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
    }
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ole32.dll"),
    };
}
pub mod oleaut32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::oleaut32::*;
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
    }
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/oleaut32.dll"),
    };
}
pub mod retrowin32_test {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::retrowin32_test::*;
        pub unsafe fn retrowin32_test_callback1(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let data = <u32>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data)
                            .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 8u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::retrowin32_test::retrowin32_test_callback1(
                    machine, func, data
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const retrowin32_test_callback1: Shim = Shim {
            name: "retrowin32_test_callback1",
            func: impls::retrowin32_test_callback1,
            stack_consumed: 8u32,
            is_async: true,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: None,
        shim: shims::retrowin32_test_callback1,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "retrowin32_test.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/retrowin32_test.dll"),
    };
}
pub mod ucrtbase {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ucrtbase::*;
        pub unsafe fn __p___argc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argc(machine).to_raw()
        }
        pub unsafe fn __p___argv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argv(machine).to_raw()
        }
        pub unsafe fn _get_initial_narrow_environment(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_get_initial_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initterm(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            winapi::ucrtbase::_initterm(machine, start, end).to_raw()
        }
        pub unsafe fn _initterm_e(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            winapi::ucrtbase::_initterm_e(machine, start, end).to_raw()
        }
        pub unsafe fn exit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let status = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::exit(machine, status).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const __p___argc: Shim = Shim {
            name: "__p___argc",
            func: impls::__p___argc,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const __p___argv: Shim = Shim {
            name: "__p___argv",
            func: impls::__p___argv,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _get_initial_narrow_environment: Shim = Shim {
            name: "_get_initial_narrow_environment",
            func: impls::_get_initial_narrow_environment,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _initterm: Shim = Shim {
            name: "_initterm",
            func: impls::_initterm,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const _initterm_e: Shim = Shim {
            name: "_initterm_e",
            func: impls::_initterm_e,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const exit: Shim = Shim {
            name: "exit",
            func: impls::exit,
            stack_consumed: 4u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 6usize] = [
        Symbol {
            ordinal: None,
            shim: shims::__p___argc,
        },
        Symbol {
            ordinal: None,
            shim: shims::__p___argv,
        },
        Symbol {
            ordinal: None,
            shim: shims::_get_initial_narrow_environment,
        },
        Symbol {
            ordinal: None,
            shim: shims::_initterm,
        },
        Symbol {
            ordinal: None,
            shim: shims::_initterm_e,
        },
        Symbol {
            ordinal: None,
            shim: shims::exit,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ucrtbase.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ucrtbase.dll"),
    };
}
pub mod vcruntime140 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::vcruntime140::*;
        pub unsafe fn _CxxThrowException(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let pExceptionObject = <u32>::from_stack(mem, esp + 4u32);
            let pThrowInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::vcruntime140::_CxxThrowException(machine, pExceptionObject, pThrowInfo).to_raw()
        }
        pub unsafe fn memcmp(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lhs = <u32>::from_stack(mem, esp + 4u32);
            let rhs = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcmp(machine, lhs, rhs, len).to_raw()
        }
        pub unsafe fn memcpy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let src = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcpy(machine, dst, src, len).to_raw()
        }
        pub unsafe fn memset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let val = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memset(machine, dst, val, len).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const _CxxThrowException: Shim = Shim {
            name: "_CxxThrowException",
            func: impls::_CxxThrowException,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const memcmp: Shim = Shim {
            name: "memcmp",
            func: impls::memcmp,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const memcpy: Shim = Shim {
            name: "memcpy",
            func: impls::memcpy,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const memset: Shim = Shim {
            name: "memset",
            func: impls::memset,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 4usize] = [
        Symbol {
            ordinal: None,
            shim: shims::_CxxThrowException,
        },
        Symbol {
            ordinal: None,
            shim: shims::memcmp,
        },
        Symbol {
            ordinal: None,
            shim: shims::memcpy,
        },
        Symbol {
            ordinal: None,
            shim: shims::memset,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "vcruntime140.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/vcruntime140.dll"),
    };
}
pub mod version {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::version::*;
        pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpdwHandle = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const GetFileVersionInfoSizeA: Shim = Shim {
            name: "GetFileVersionInfoSizeA",
            func: impls::GetFileVersionInfoSizeA,
            stack_consumed: 8u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: None,
        shim: shims::GetFileVersionInfoSizeA,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "version.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/version.dll"),
    };
}
pub mod user32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::user32::*;
        pub unsafe fn AdjustWindowRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu).to_raw()
        }
        pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 16u32);
            winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle).to_raw()
        }
        pub unsafe fn AppendMenuA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uFlags = <u32>::from_stack(mem, esp + 8u32);
            let uIDNewItem = <u32>::from_stack(mem, esp + 12u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem).to_raw()
        }
        pub unsafe fn BeginPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::BeginPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn CheckMenuItem(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uIDCheckItem = <u32>::from_stack(mem, esp + 8u32);
            let uCheck = <u32>::from_stack(mem, esp + 12u32);
            winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck).to_raw()
        }
        pub unsafe fn ClientToScreen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ClientToScreen(machine, hWnd, lpPoint).to_raw()
        }
        pub unsafe fn CreateCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInst = <u32>::from_stack(mem, esp + 4u32);
            let xHotSpot = <u32>::from_stack(mem, esp + 8u32);
            let yHotSpot = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let pvANDPlane = <u32>::from_stack(mem, esp + 24u32);
            let pvXORPlane = <u32>::from_stack(mem, esp + 28u32);
            winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            )
            .to_raw()
        }
        pub unsafe fn CreateWindowExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::CreateWindowExA(
                        machine,
                        dwExStyle,
                        lpClassName,
                        lpWindowName,
                        dwStyle,
                        X,
                        Y,
                        nWidth,
                        nHeight,
                        hWndParent,
                        hMenu,
                        hInstance,
                        lpParam,
                    )
                    .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 48u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::CreateWindowExA(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn CreateWindowExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, Str16>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::CreateWindowExW(
                        machine,
                        dwExStyle,
                        lpClassName,
                        lpWindowName,
                        dwStyle,
                        X,
                        Y,
                        nWidth,
                        nHeight,
                        hWndParent,
                        hMenu,
                        hInstance,
                        lpParam,
                    )
                    .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 48u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::CreateWindowExW(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DefWindowProcA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 16u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DefWindowProcA(
                    machine, hWnd, msg, wParam, lParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DefWindowProcW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 16u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DefWindowProcW(
                    machine, hWnd, msg, wParam, lParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::DestroyWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let hDialogTemplate = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DialogBoxIndirectParamA(
                machine,
                hInstance,
                hDialogTemplate,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTemplateName = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DialogBoxParamA(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DispatchMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 4u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DispatchMessageA(machine, lpMsg));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DispatchMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::DispatchMessageW(machine, lpMsg).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 4u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DispatchMessageW(machine, lpMsg));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn EndPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::EndPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn FillRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <BrushOrColor>::from_stack(mem, esp + 12u32);
            winapi::user32::FillRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn FindWindowA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::FindWindowA(machine, lpClassName, lpWindowName).to_raw()
        }
        pub unsafe fn FrameRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <HBRUSH>::from_stack(mem, esp + 12u32);
            winapi::user32::FrameRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetActiveWindow(machine).to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetClientRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn GetDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetDesktopWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetDesktopWindow(machine).to_raw()
        }
        pub unsafe fn GetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetFocus(machine).to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetForegroundWindow(machine).to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetLastActivePopup(machine).to_raw()
        }
        pub unsafe fn GetMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::GetMessageA(
                        machine,
                        lpMsg,
                        hWnd,
                        wMsgFilterMin,
                        wMsgFilterMax,
                    )
                    .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 16u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::GetMessageA(
                    machine,
                    lpMsg,
                    hWnd,
                    wMsgFilterMin,
                    wMsgFilterMax
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn GetMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::GetMessageW(
                        machine,
                        lpMsg,
                        hWnd,
                        wMsgFilterMin,
                        wMsgFilterMax,
                    )
                    .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 16u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::GetMessageW(
                    machine,
                    lpMsg,
                    hWnd,
                    wMsgFilterMin,
                    wMsgFilterMax
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn GetSystemMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let bRevert = <bool>::from_stack(mem, esp + 8u32);
            winapi::user32::GetSystemMenu(machine, hWnd, bRevert).to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <Result<SystemMetric, u32>>::from_stack(mem, esp + 4u32);
            winapi::user32::GetSystemMetrics(machine, nIndex).to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetWindowDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetWindowLongA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIndex = <i32>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowLongA(machine, hWnd, nIndex).to_raw()
        }
        pub unsafe fn InvalidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase).to_raw()
        }
        pub unsafe fn InvalidateRgn(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hRgn = <HRGN>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase).to_raw()
        }
        pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTableName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName).to_raw()
        }
        pub unsafe fn LoadBitmapA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName).to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorA(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadCursorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorW(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconA(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadIconW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconW(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadMenuW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpMenuName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadMenuW(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn LoadStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn MapWindowPoints(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, esp + 4u32);
            let hWndTo = <HWND>::from_stack(mem, esp + 8u32);
            let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, esp + 12u32);
            winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints).to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&str>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MessageBoxW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MoveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let X = <u32>::from_stack(mem, esp + 8u32);
            let Y = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let bRepaint = <bool>::from_stack(mem, esp + 24u32);
            winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint).to_raw()
        }
        pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
            winapi::user32::PeekMessageA(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
            winapi::user32::PeekMessageW(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PostQuitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nExitCode = <i32>::from_stack(mem, esp + 4u32);
            winapi::user32::PostQuitMessage(machine, nExitCode).to_raw()
        }
        pub unsafe fn PtInRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            let pt = <POINT>::from_stack(mem, esp + 8u32);
            winapi::user32::PtInRect(machine, lprc, pt).to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassA(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExA(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassW(machine, lpWndClass).to_raw()
        }
        pub unsafe fn ReleaseCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::ReleaseCapture(machine).to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            let hdc = <HDC>::from_stack(mem, esp + 8u32);
            winapi::user32::ReleaseDC(machine, hwnd, hdc).to_raw()
        }
        pub unsafe fn SendMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 16u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::SendMessageA(
                    machine, hWnd, Msg, wParam, lParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn SetCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCapture(machine, hwnd).to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hCursor = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCursor(machine, hCursor).to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetFocus(machine, hWnd).to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetForegroundWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn SetMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hMenu = <HMENU>::from_stack(mem, esp + 8u32);
            winapi::user32::SetMenu(machine, hWnd, hMenu).to_raw()
        }
        pub unsafe fn SetRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let xLeft = <i32>::from_stack(mem, esp + 8u32);
            let yTop = <i32>::from_stack(mem, esp + 12u32);
            let xRight = <i32>::from_stack(mem, esp + 16u32);
            let yBottom = <i32>::from_stack(mem, esp + 20u32);
            winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom).to_raw()
        }
        pub unsafe fn SetTimer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIDEvent = <u32>::from_stack(mem, esp + 8u32);
            let uElapse = <u32>::from_stack(mem, esp + 12u32);
            let lpTimerFunc = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc).to_raw()
        }
        pub unsafe fn SetWindowPos(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hWndInsertAfter = <HWND>::from_stack(mem, esp + 8u32);
            let X = <i32>::from_stack(mem, esp + 12u32);
            let Y = <i32>::from_stack(mem, esp + 16u32);
            let cx = <i32>::from_stack(mem, esp + 20u32);
            let cy = <i32>::from_stack(mem, esp + 24u32);
            let uFlags = <Result<SWP, u32>>::from_stack(mem, esp + 28u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::SetWindowPos(
                        machine,
                        hWnd,
                        hWndInsertAfter,
                        X,
                        Y,
                        cx,
                        cy,
                        uFlags,
                    )
                    .await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 28u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::SetWindowPos(
                    machine,
                    hWnd,
                    hWndInsertAfter,
                    X,
                    Y,
                    cx,
                    cy,
                    uFlags
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn SetWindowTextA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::SetWindowTextA(machine, hWnd, lpString).to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let bShow = <bool>::from_stack(mem, esp + 4u32);
            winapi::user32::ShowCursor(machine, bShow).to_raw()
        }
        pub unsafe fn ShowWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::ShowWindow(machine, hWnd, nCmdShow).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 8u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::ShowWindow(machine, hWnd, nCmdShow));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hAccTable = <u32>::from_stack(mem, esp + 8u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 12u32);
            winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg).to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            winapi::user32::TranslateMessage(machine, lpMsg).to_raw()
        }
        pub unsafe fn UpdateWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::UpdateWindow(machine, hWnd).await;
                    let regs = &mut machine.emu.x86.cpu_mut().regs;
                    regs.eip = machine.emu.memory.mem().get_pod::<u32>(esp);
                    *regs.get32_mut(x86::Register::ESP) += 4u32 + 4;
                    regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine.emu.x86.cpu_mut().call_async(Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::UpdateWindow(machine, hWnd));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn ValidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ValidateRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::WaitMessage(machine).to_raw()
        }
        pub unsafe fn wsprintfA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, esp + 4u32);
            let fmt = <Option<&str>>::from_stack(mem, esp + 8u32);
            let args = <VarArgs>::from_stack(mem, esp + 12u32);
            winapi::user32::wsprintfA(machine, buf, fmt, args).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const AdjustWindowRect: Shim = Shim {
            name: "AdjustWindowRect",
            func: impls::AdjustWindowRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const AdjustWindowRectEx: Shim = Shim {
            name: "AdjustWindowRectEx",
            func: impls::AdjustWindowRectEx,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const AppendMenuA: Shim = Shim {
            name: "AppendMenuA",
            func: impls::AppendMenuA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const BeginPaint: Shim = Shim {
            name: "BeginPaint",
            func: impls::BeginPaint,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CheckMenuItem: Shim = Shim {
            name: "CheckMenuItem",
            func: impls::CheckMenuItem,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const ClientToScreen: Shim = Shim {
            name: "ClientToScreen",
            func: impls::ClientToScreen,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CreateCursor: Shim = Shim {
            name: "CreateCursor",
            func: impls::CreateCursor,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const CreateWindowExA: Shim = Shim {
            name: "CreateWindowExA",
            func: impls::CreateWindowExA,
            stack_consumed: 48u32,
            is_async: true,
        };
        pub const CreateWindowExW: Shim = Shim {
            name: "CreateWindowExW",
            func: impls::CreateWindowExW,
            stack_consumed: 48u32,
            is_async: true,
        };
        pub const DefWindowProcA: Shim = Shim {
            name: "DefWindowProcA",
            func: impls::DefWindowProcA,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const DefWindowProcW: Shim = Shim {
            name: "DefWindowProcW",
            func: impls::DefWindowProcW,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const DestroyWindow: Shim = Shim {
            name: "DestroyWindow",
            func: impls::DestroyWindow,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DialogBoxIndirectParamA: Shim = Shim {
            name: "DialogBoxIndirectParamA",
            func: impls::DialogBoxIndirectParamA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const DialogBoxParamA: Shim = Shim {
            name: "DialogBoxParamA",
            func: impls::DialogBoxParamA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const DispatchMessageA: Shim = Shim {
            name: "DispatchMessageA",
            func: impls::DispatchMessageA,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const DispatchMessageW: Shim = Shim {
            name: "DispatchMessageW",
            func: impls::DispatchMessageW,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const EndPaint: Shim = Shim {
            name: "EndPaint",
            func: impls::EndPaint,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FillRect: Shim = Shim {
            name: "FillRect",
            func: impls::FillRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const FindWindowA: Shim = Shim {
            name: "FindWindowA",
            func: impls::FindWindowA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FrameRect: Shim = Shim {
            name: "FrameRect",
            func: impls::FrameRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetActiveWindow: Shim = Shim {
            name: "GetActiveWindow",
            func: impls::GetActiveWindow,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetClientRect: Shim = Shim {
            name: "GetClientRect",
            func: impls::GetClientRect,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetDC: Shim = Shim {
            name: "GetDC",
            func: impls::GetDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetDesktopWindow: Shim = Shim {
            name: "GetDesktopWindow",
            func: impls::GetDesktopWindow,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetFocus: Shim = Shim {
            name: "GetFocus",
            func: impls::GetFocus,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetForegroundWindow: Shim = Shim {
            name: "GetForegroundWindow",
            func: impls::GetForegroundWindow,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetLastActivePopup: Shim = Shim {
            name: "GetLastActivePopup",
            func: impls::GetLastActivePopup,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetMessageA: Shim = Shim {
            name: "GetMessageA",
            func: impls::GetMessageA,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const GetMessageW: Shim = Shim {
            name: "GetMessageW",
            func: impls::GetMessageW,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const GetSystemMenu: Shim = Shim {
            name: "GetSystemMenu",
            func: impls::GetSystemMenu,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetSystemMetrics: Shim = Shim {
            name: "GetSystemMetrics",
            func: impls::GetSystemMetrics,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetWindowDC: Shim = Shim {
            name: "GetWindowDC",
            func: impls::GetWindowDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetWindowLongA: Shim = Shim {
            name: "GetWindowLongA",
            func: impls::GetWindowLongA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const InvalidateRect: Shim = Shim {
            name: "InvalidateRect",
            func: impls::InvalidateRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InvalidateRgn: Shim = Shim {
            name: "InvalidateRgn",
            func: impls::InvalidateRgn,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const LoadAcceleratorsW: Shim = Shim {
            name: "LoadAcceleratorsW",
            func: impls::LoadAcceleratorsW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadBitmapA: Shim = Shim {
            name: "LoadBitmapA",
            func: impls::LoadBitmapA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadCursorA: Shim = Shim {
            name: "LoadCursorA",
            func: impls::LoadCursorA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadCursorW: Shim = Shim {
            name: "LoadCursorW",
            func: impls::LoadCursorW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadIconA: Shim = Shim {
            name: "LoadIconA",
            func: impls::LoadIconA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadIconW: Shim = Shim {
            name: "LoadIconW",
            func: impls::LoadIconW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadImageA: Shim = Shim {
            name: "LoadImageA",
            func: impls::LoadImageA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const LoadMenuW: Shim = Shim {
            name: "LoadMenuW",
            func: impls::LoadMenuW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadStringA: Shim = Shim {
            name: "LoadStringA",
            func: impls::LoadStringA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const LoadStringW: Shim = Shim {
            name: "LoadStringW",
            func: impls::LoadStringW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MapWindowPoints: Shim = Shim {
            name: "MapWindowPoints",
            func: impls::MapWindowPoints,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MessageBoxA: Shim = Shim {
            name: "MessageBoxA",
            func: impls::MessageBoxA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MessageBoxW: Shim = Shim {
            name: "MessageBoxW",
            func: impls::MessageBoxW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MoveWindow: Shim = Shim {
            name: "MoveWindow",
            func: impls::MoveWindow,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const PeekMessageA: Shim = Shim {
            name: "PeekMessageA",
            func: impls::PeekMessageA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const PeekMessageW: Shim = Shim {
            name: "PeekMessageW",
            func: impls::PeekMessageW,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const PostQuitMessage: Shim = Shim {
            name: "PostQuitMessage",
            func: impls::PostQuitMessage,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const PtInRect: Shim = Shim {
            name: "PtInRect",
            func: impls::PtInRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const RegisterClassA: Shim = Shim {
            name: "RegisterClassA",
            func: impls::RegisterClassA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegisterClassExA: Shim = Shim {
            name: "RegisterClassExA",
            func: impls::RegisterClassExA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegisterClassW: Shim = Shim {
            name: "RegisterClassW",
            func: impls::RegisterClassW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ReleaseCapture: Shim = Shim {
            name: "ReleaseCapture",
            func: impls::ReleaseCapture,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const ReleaseDC: Shim = Shim {
            name: "ReleaseDC",
            func: impls::ReleaseDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SendMessageA: Shim = Shim {
            name: "SendMessageA",
            func: impls::SendMessageA,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const SetCapture: Shim = Shim {
            name: "SetCapture",
            func: impls::SetCapture,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetCursor: Shim = Shim {
            name: "SetCursor",
            func: impls::SetCursor,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetFocus: Shim = Shim {
            name: "SetFocus",
            func: impls::SetFocus,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetForegroundWindow: Shim = Shim {
            name: "SetForegroundWindow",
            func: impls::SetForegroundWindow,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetMenu: Shim = Shim {
            name: "SetMenu",
            func: impls::SetMenu,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetRect: Shim = Shim {
            name: "SetRect",
            func: impls::SetRect,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const SetTimer: Shim = Shim {
            name: "SetTimer",
            func: impls::SetTimer,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetWindowPos: Shim = Shim {
            name: "SetWindowPos",
            func: impls::SetWindowPos,
            stack_consumed: 28u32,
            is_async: true,
        };
        pub const SetWindowTextA: Shim = Shim {
            name: "SetWindowTextA",
            func: impls::SetWindowTextA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ShowCursor: Shim = Shim {
            name: "ShowCursor",
            func: impls::ShowCursor,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ShowWindow: Shim = Shim {
            name: "ShowWindow",
            func: impls::ShowWindow,
            stack_consumed: 8u32,
            is_async: true,
        };
        pub const TranslateAcceleratorW: Shim = Shim {
            name: "TranslateAcceleratorW",
            func: impls::TranslateAcceleratorW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const TranslateMessage: Shim = Shim {
            name: "TranslateMessage",
            func: impls::TranslateMessage,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const UpdateWindow: Shim = Shim {
            name: "UpdateWindow",
            func: impls::UpdateWindow,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const ValidateRect: Shim = Shim {
            name: "ValidateRect",
            func: impls::ValidateRect,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const WaitMessage: Shim = Shim {
            name: "WaitMessage",
            func: impls::WaitMessage,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const wsprintfA: Shim = Shim {
            name: "wsprintfA",
            func: impls::wsprintfA,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 76usize] = [
        Symbol {
            ordinal: None,
            shim: shims::AdjustWindowRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::AdjustWindowRectEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::AppendMenuA,
        },
        Symbol {
            ordinal: None,
            shim: shims::BeginPaint,
        },
        Symbol {
            ordinal: None,
            shim: shims::CheckMenuItem,
        },
        Symbol {
            ordinal: None,
            shim: shims::ClientToScreen,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateWindowExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateWindowExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::DefWindowProcA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DefWindowProcW,
        },
        Symbol {
            ordinal: None,
            shim: shims::DestroyWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::DialogBoxIndirectParamA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DialogBoxParamA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DispatchMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DispatchMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::EndPaint,
        },
        Symbol {
            ordinal: None,
            shim: shims::FillRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindWindowA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FrameRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetActiveWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetClientRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDesktopWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFocus,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetForegroundWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLastActivePopup,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemMenu,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemMetrics,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetWindowDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetWindowLongA,
        },
        Symbol {
            ordinal: None,
            shim: shims::InvalidateRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::InvalidateRgn,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadAcceleratorsW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadBitmapA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadCursorA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadCursorW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadIconA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadIconW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadImageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadMenuW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadStringA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadStringW,
        },
        Symbol {
            ordinal: None,
            shim: shims::MapWindowPoints,
        },
        Symbol {
            ordinal: None,
            shim: shims::MessageBoxA,
        },
        Symbol {
            ordinal: None,
            shim: shims::MessageBoxW,
        },
        Symbol {
            ordinal: None,
            shim: shims::MoveWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::PeekMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::PeekMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::PostQuitMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::PtInRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassA,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassW,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseCapture,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::SendMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetCapture,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFocus,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetForegroundWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetMenu,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetTimer,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetWindowPos,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetWindowTextA,
        },
        Symbol {
            ordinal: None,
            shim: shims::ShowCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::ShowWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::TranslateAcceleratorW,
        },
        Symbol {
            ordinal: None,
            shim: shims::TranslateMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::UpdateWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::ValidateRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::WaitMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::wsprintfA,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/user32.dll"),
    };
}
pub mod winmm {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::winmm::*;
        pub unsafe fn timeBeginPeriod(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, esp + 4u32);
            winapi::winmm::timeBeginPeriod(machine, uPeriod).to_raw()
        }
        pub unsafe fn timeGetTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::timeGetTime(machine).to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDelay = <u32>::from_stack(mem, esp + 4u32);
            let uResolution = <u32>::from_stack(mem, esp + 8u32);
            let lpTimeProc = <u32>::from_stack(mem, esp + 12u32);
            let dwUser = <u32>::from_stack(mem, esp + 16u32);
            let fuEvent = <u32>::from_stack(mem, esp + 20u32);
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent)
                .to_raw()
        }
        pub unsafe fn waveOutClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutClose(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, esp + 4u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, esp + 8u32);
            let cbwoc = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc).to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::waveOutGetNumDevs(machine).to_raw()
        }
        pub unsafe fn waveOutGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, esp + 8u32);
            let cbmmt = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt).to_raw()
        }
        pub unsafe fn waveOutOpen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, esp + 4u32);
            let uDeviceID = <u32>::from_stack(mem, esp + 8u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, esp + 12u32);
            let dwCallback = <u32>::from_stack(mem, esp + 16u32);
            let dwInstance = <u32>::from_stack(mem, esp + 20u32);
            let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, esp + 24u32);
            winapi::winmm::waveOutOpen(
                machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
            )
            .to_raw()
        }
        pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutReset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutReset(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutWrite(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims::Shim;
        pub const timeBeginPeriod: Shim = Shim {
            name: "timeBeginPeriod",
            func: impls::timeBeginPeriod,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const timeGetTime: Shim = Shim {
            name: "timeGetTime",
            func: impls::timeGetTime,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const timeSetEvent: Shim = Shim {
            name: "timeSetEvent",
            func: impls::timeSetEvent,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const waveOutClose: Shim = Shim {
            name: "waveOutClose",
            func: impls::waveOutClose,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const waveOutGetDevCapsA: Shim = Shim {
            name: "waveOutGetDevCapsA",
            func: impls::waveOutGetDevCapsA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const waveOutGetNumDevs: Shim = Shim {
            name: "waveOutGetNumDevs",
            func: impls::waveOutGetNumDevs,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const waveOutGetPosition: Shim = Shim {
            name: "waveOutGetPosition",
            func: impls::waveOutGetPosition,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const waveOutOpen: Shim = Shim {
            name: "waveOutOpen",
            func: impls::waveOutOpen,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const waveOutPrepareHeader: Shim = Shim {
            name: "waveOutPrepareHeader",
            func: impls::waveOutPrepareHeader,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const waveOutReset: Shim = Shim {
            name: "waveOutReset",
            func: impls::waveOutReset,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const waveOutWrite: Shim = Shim {
            name: "waveOutWrite",
            func: impls::waveOutWrite,
            stack_consumed: 12u32,
            is_async: false,
        };
    }
    const EXPORTS: [Symbol; 11usize] = [
        Symbol {
            ordinal: None,
            shim: shims::timeBeginPeriod,
        },
        Symbol {
            ordinal: None,
            shim: shims::timeGetTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::timeSetEvent,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutClose,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetDevCapsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetNumDevs,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetPosition,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutOpen,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutPrepareHeader,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutReset,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutWrite,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/winmm.dll"),
    };
}
