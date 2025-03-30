#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        calling_convention::*,
        machine::Machine,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use winapi::gdi32::*;
    pub unsafe fn BitBlt(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdcDst = <HDC>::from_stack(mem, stack_args + 0u32);
        let xDst = <i32>::from_stack(mem, stack_args + 4u32);
        let yDst = <i32>::from_stack(mem, stack_args + 8u32);
        let w = <i32>::from_stack(mem, stack_args + 12u32);
        let h = <i32>::from_stack(mem, stack_args + 16u32);
        let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
        let xSrc = <i32>::from_stack(mem, stack_args + 24u32);
        let ySrc = <i32>::from_stack(mem, stack_args + 28u32);
        let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 32u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::BitBlt_pos,
                "gdi32/bitmap",
                "BitBlt",
                &[
                    ("hdcDst", &hdcDst),
                    ("xDst", &xDst),
                    ("yDst", &yDst),
                    ("w", &w),
                    ("h", &h),
                    ("hdcSrc", &hdcSrc),
                    ("xSrc", &xSrc),
                    ("ySrc", &ySrc),
                    ("rop", &rop),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::BitBlt(machine, hdcDst, xDst, yDst, w, h, hdcSrc, xSrc, ySrc, rop);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateBitmap(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let nWidth = <u32>::from_stack(mem, stack_args + 0u32);
        let nHeight = <u32>::from_stack(mem, stack_args + 4u32);
        let nPlanes = <u32>::from_stack(mem, stack_args + 8u32);
        let nBitCount = <u32>::from_stack(mem, stack_args + 12u32);
        let lpBits = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateBitmap_pos,
                "gdi32/bitmap",
                "CreateBitmap",
                &[
                    ("nWidth", &nWidth),
                    ("nHeight", &nHeight),
                    ("nPlanes", &nPlanes),
                    ("nBitCount", &nBitCount),
                    ("lpBits", &lpBits),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let cx = <u32>::from_stack(mem, stack_args + 4u32);
        let cy = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateCompatibleBitmap_pos,
                "gdi32/bitmap",
                "CreateCompatibleBitmap",
                &[("hdc", &hdc), ("cx", &cx), ("cy", &cy)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateCompatibleDC(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/dc") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateCompatibleDC_pos,
                "gdi32/dc",
                "CreateCompatibleDC",
                &[("hdc", &hdc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateDIBSection(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
        let usage = <u32>::from_stack(mem, stack_args + 8u32);
        let ppvBits = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let hSection = <u32>::from_stack(mem, stack_args + 16u32);
        let offset = <u32>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateDIBSection_pos,
                "gdi32/bitmap",
                "CreateDIBSection",
                &[
                    ("hdc", &hdc),
                    ("pbmi", &pbmi),
                    ("usage", &usage),
                    ("ppvBits", &ppvBits),
                    ("hSection", &hSection),
                    ("offset", &offset),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateDIBitmap(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let pbmih = <Option<&mut BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
        let flInit = <u32>::from_stack(mem, stack_args + 8u32);
        let pjBits = <Option<&mut u8>>::from_stack(mem, stack_args + 12u32);
        let pbmi = <Option<&mut BITMAPINFO>>::from_stack(mem, stack_args + 16u32);
        let iUsage = <u32>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateDIBitmap_pos,
                "gdi32/bitmap",
                "CreateDIBitmap",
                &[
                    ("hdc", &hdc),
                    ("pbmih", &pbmih),
                    ("flInit", &flInit),
                    ("pjBits", &pjBits),
                    ("pbmi", &pbmi),
                    ("iUsage", &iUsage),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::CreateDIBitmap(machine, hdc, pbmih, flInit, pjBits, pbmi, iUsage);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateFontA(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let cHeight = <i32>::from_stack(mem, stack_args + 0u32);
        let cWidth = <i32>::from_stack(mem, stack_args + 4u32);
        let cEscapement = <i32>::from_stack(mem, stack_args + 8u32);
        let cOrientation = <i32>::from_stack(mem, stack_args + 12u32);
        let cWeight = <u32>::from_stack(mem, stack_args + 16u32);
        let bItalic = <u32>::from_stack(mem, stack_args + 20u32);
        let bUnderline = <u32>::from_stack(mem, stack_args + 24u32);
        let bStrikeOut = <u32>::from_stack(mem, stack_args + 28u32);
        let iCharSet = <u32>::from_stack(mem, stack_args + 32u32);
        let iOutPrecision = <u32>::from_stack(mem, stack_args + 36u32);
        let iClipPrecision = <u32>::from_stack(mem, stack_args + 40u32);
        let iQuality = <u32>::from_stack(mem, stack_args + 44u32);
        let iPitchAndFamily = <u32>::from_stack(mem, stack_args + 48u32);
        let pszFaceName = <Option<&str>>::from_stack(mem, stack_args + 52u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateFontA_pos,
                "gdi32/text",
                "CreateFontA",
                &[
                    ("cHeight", &cHeight),
                    ("cWidth", &cWidth),
                    ("cEscapement", &cEscapement),
                    ("cOrientation", &cOrientation),
                    ("cWeight", &cWeight),
                    ("bItalic", &bItalic),
                    ("bUnderline", &bUnderline),
                    ("bStrikeOut", &bStrikeOut),
                    ("iCharSet", &iCharSet),
                    ("iOutPrecision", &iOutPrecision),
                    ("iClipPrecision", &iClipPrecision),
                    ("iQuality", &iQuality),
                    ("iPitchAndFamily", &iPitchAndFamily),
                    ("pszFaceName", &pszFaceName),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::CreateFontA(
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
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreatePalette(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let plpal = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/palette") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreatePalette_pos,
                "gdi32/palette",
                "CreatePalette",
                &[("plpal", &plpal)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::CreatePalette(machine, plpal);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreatePen(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let iStyle = <Result<PS, u32>>::from_stack(mem, stack_args + 0u32);
        let cWidth = <u32>::from_stack(mem, stack_args + 4u32);
        let color = <COLORREF>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreatePen_pos,
                "gdi32/draw",
                "CreatePen",
                &[("iStyle", &iStyle), ("cWidth", &cWidth), ("color", &color)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::CreatePen(machine, iStyle, cWidth, color);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn CreateSolidBrush(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let color = <COLORREF>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::CreateSolidBrush_pos,
                "gdi32/draw",
                "CreateSolidBrush",
                &[("color", &color)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::CreateSolidBrush(machine, color);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn DeleteDC(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/dc") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::DeleteDC_pos,
                "gdi32/dc",
                "DeleteDC",
                &[("hdc", &hdc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::DeleteDC(machine, hdc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn DeleteObject(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/object") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::DeleteObject_pos,
                "gdi32/object",
                "DeleteObject",
                &[("handle", &handle)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::DeleteObject(machine, handle);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn EnumFontFamiliesExA(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let lpLogfont = <Option<&mut LOGFONTA>>::from_stack(mem, stack_args + 4u32);
        let lpProc = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <LPARAM>::from_stack(mem, stack_args + 12u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::EnumFontFamiliesExA_pos,
                "gdi32/text",
                "EnumFontFamiliesExA",
                &[
                    ("hdc", &hdc),
                    ("lpLogfont", &lpLogfont),
                    ("lpProc", &lpProc),
                    ("lParam", &lParam),
                    ("dwFlags", &dwFlags),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::EnumFontFamiliesExA(machine, hdc, lpLogfont, lpProc, lParam, dwFlags);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetDCOrgEx(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/dc") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetDCOrgEx_pos,
                "gdi32/dc",
                "GetDCOrgEx",
                &[("hdc", &hdc), ("lpPoint", &lpPoint)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetDCOrgEx(machine, hdc, lpPoint);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetDIBits(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let hbm = <HBITMAP>::from_stack(mem, stack_args + 4u32);
        let start = <u32>::from_stack(mem, stack_args + 8u32);
        let cLines = <u32>::from_stack(mem, stack_args + 12u32);
        let lpvBits = <Option<&mut u8>>::from_stack(mem, stack_args + 16u32);
        let lpbmi = <Option<&mut BITMAPINFO>>::from_stack(mem, stack_args + 20u32);
        let usage = <u32>::from_stack(mem, stack_args + 24u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetDIBits_pos,
                "gdi32/bitmap",
                "GetDIBits",
                &[
                    ("hdc", &hdc),
                    ("hbm", &hbm),
                    ("start", &start),
                    ("cLines", &cLines),
                    ("lpvBits", &lpvBits),
                    ("lpbmi", &lpbmi),
                    ("usage", &usage),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::GetDIBits(machine, hdc, hbm, start, cLines, lpvBits, lpbmi, usage);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetDeviceCaps(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/dc") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetDeviceCaps_pos,
                "gdi32/dc",
                "GetDeviceCaps",
                &[("hdc", &hdc), ("index", &index)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetDeviceCaps(machine, hdc, index);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetLayout(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/dc") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetLayout_pos,
                "gdi32/dc",
                "GetLayout",
                &[("hdc", &hdc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetLayout(machine, hdc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetObjectA(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
        let bytes = <u32>::from_stack(mem, stack_args + 4u32);
        let out = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/object") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetObjectA_pos,
                "gdi32/object",
                "GetObjectA",
                &[("handle", &handle), ("bytes", &bytes), ("out", &out)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetObjectA(machine, handle, bytes, out);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetPaletteEntries(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
        let iStart = <u32>::from_stack(mem, stack_args + 4u32);
        let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
        let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/palette") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetPaletteEntries_pos,
                "gdi32/palette",
                "GetPaletteEntries",
                &[
                    ("hpal", &hpal),
                    ("iStart", &iStart),
                    ("cEntries", &cEntries),
                    ("pPalEntries", &pPalEntries),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetPaletteEntries(machine, hpal, iStart, cEntries, pPalEntries);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetPixel(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <u32>::from_stack(mem, stack_args + 4u32);
        let y = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetPixel_pos,
                "gdi32/draw",
                "GetPixel",
                &[("hdc", &hdc), ("x", &x), ("y", &y)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetPixel(machine, hdc, x, y);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetStockObject(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/object") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetStockObject_pos,
                "gdi32/object",
                "GetStockObject",
                &[("i", &i)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetStockObject(machine, i);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetSystemPaletteEntries(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let iStart = <u32>::from_stack(mem, stack_args + 4u32);
        let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
        let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/palette") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetSystemPaletteEntries_pos,
                "gdi32/palette",
                "GetSystemPaletteEntries",
                &[
                    ("hdc", &hdc),
                    ("iStart", &iStart),
                    ("cEntries", &cEntries),
                    ("pPalEntries", &pPalEntries),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::gdi32::GetSystemPaletteEntries(machine, hdc, iStart, cEntries, pPalEntries);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let c = <i32>::from_stack(mem, stack_args + 8u32);
        let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetTextExtentPoint32A_pos,
                "gdi32/text",
                "GetTextExtentPoint32A",
                &[
                    ("hdc", &hdc),
                    ("lpString", &lpString),
                    ("c", &c),
                    ("psizl", &psizl),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetTextExtentPoint32W(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let c = <i32>::from_stack(mem, stack_args + 8u32);
        let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetTextExtentPoint32W_pos,
                "gdi32/text",
                "GetTextExtentPoint32W",
                &[
                    ("hdc", &hdc),
                    ("lpString", &lpString),
                    ("c", &c),
                    ("psizl", &psizl),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetTextExtentPoint32W(machine, hdc, lpString, c, psizl);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetTextMetricsA(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetTextMetricsA_pos,
                "gdi32/text",
                "GetTextMetricsA",
                &[("hdc", &hdc), ("lptm", &lptm)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetTextMetricsA(machine, hdc, lptm);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn GetTextMetricsW(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::GetTextMetricsW_pos,
                "gdi32/text",
                "GetTextMetricsW",
                &[("hdc", &hdc), ("lptm", &lptm)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::GetTextMetricsW(machine, hdc, lptm);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn LineDDA(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let xStart = <i32>::from_stack(mem, stack_args + 0u32);
        let yStart = <i32>::from_stack(mem, stack_args + 4u32);
        let xEnd = <i32>::from_stack(mem, stack_args + 8u32);
        let yEnd = <i32>::from_stack(mem, stack_args + 12u32);
        let lpProc = <u32>::from_stack(mem, stack_args + 16u32);
        let data = <u32>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::LineDDA_pos,
                "gdi32/draw",
                "LineDDA",
                &[
                    ("xStart", &xStart),
                    ("yStart", &yStart),
                    ("xEnd", &xEnd),
                    ("yEnd", &yEnd),
                    ("lpProc", &lpProc),
                    ("data", &data),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::LineDDA(machine, xStart, yStart, xEnd, yEnd, lpProc, data);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn LineTo(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <i32>::from_stack(mem, stack_args + 4u32);
        let y = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::LineTo_pos,
                "gdi32/draw",
                "LineTo",
                &[("hdc", &hdc), ("x", &x), ("y", &y)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::LineTo(machine, hdc, x, y);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn MoveToEx(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <i32>::from_stack(mem, stack_args + 4u32);
        let y = <i32>::from_stack(mem, stack_args + 8u32);
        let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::MoveToEx_pos,
                "gdi32/draw",
                "MoveToEx",
                &[("hdc", &hdc), ("x", &x), ("y", &y), ("lppt", &lppt)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn PatBlt(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <i32>::from_stack(mem, stack_args + 4u32);
        let y = <i32>::from_stack(mem, stack_args + 8u32);
        let w = <i32>::from_stack(mem, stack_args + 12u32);
        let h = <i32>::from_stack(mem, stack_args + 16u32);
        let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::PatBlt_pos,
                "gdi32/bitmap",
                "PatBlt",
                &[
                    ("hdc", &hdc),
                    ("x", &x),
                    ("y", &y),
                    ("w", &w),
                    ("h", &h),
                    ("rop", &rop),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::PatBlt(machine, hdc, x, y, w, h, rop);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn PtVisible(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <i32>::from_stack(mem, stack_args + 4u32);
        let y = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::PtVisible_pos,
                "gdi32/draw",
                "PtVisible",
                &[("hdc", &hdc), ("x", &x), ("y", &y)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::PtVisible(machine, hdc, x, y);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn RealizePalette(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/palette") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::RealizePalette_pos,
                "gdi32/palette",
                "RealizePalette",
                &[("hdc", &hdc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::RealizePalette(machine, hdc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SelectObject(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let hGdiObj = <HGDIOBJ>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/object") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SelectObject_pos,
                "gdi32/object",
                "SelectObject",
                &[("hdc", &hdc), ("hGdiObj", &hGdiObj)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SelectPalette(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let hPal = <HPALETTE>::from_stack(mem, stack_args + 4u32);
        let bForceBkgd = <bool>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/palette") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SelectPalette_pos,
                "gdi32/palette",
                "SelectPalette",
                &[("hdc", &hdc), ("hPal", &hPal), ("bForceBkgd", &bForceBkgd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SelectPalette(machine, hdc, hPal, bForceBkgd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetBkColor(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetBkColor_pos,
                "gdi32/draw",
                "SetBkColor",
                &[("hdc", &hdc), ("color", &color)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetBkColor(machine, hdc, color);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetBkMode(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let mode = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetBkMode_pos,
                "gdi32/draw",
                "SetBkMode",
                &[("hdc", &hdc), ("mode", &mode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetBkMode(machine, hdc, mode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetBrushOrgEx(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <i32>::from_stack(mem, stack_args + 4u32);
        let y = <i32>::from_stack(mem, stack_args + 8u32);
        let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetBrushOrgEx_pos,
                "gdi32/draw",
                "SetBrushOrgEx",
                &[("hdc", &hdc), ("x", &x), ("y", &y), ("lppt", &lppt)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetBrushOrgEx(machine, hdc, x, y, lppt);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let xDst = <i32>::from_stack(mem, stack_args + 4u32);
        let yDst = <i32>::from_stack(mem, stack_args + 8u32);
        let w = <i32>::from_stack(mem, stack_args + 12u32);
        let h = <i32>::from_stack(mem, stack_args + 16u32);
        let xSrc = <i32>::from_stack(mem, stack_args + 20u32);
        let ySrc = <i32>::from_stack(mem, stack_args + 24u32);
        let StartScan = <u32>::from_stack(mem, stack_args + 28u32);
        let cLines = <u32>::from_stack(mem, stack_args + 32u32);
        let lpBits = <u32>::from_stack(mem, stack_args + 36u32);
        let lpBmi = <u32>::from_stack(mem, stack_args + 40u32);
        let iUsage = <u32>::from_stack(mem, stack_args + 44u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetDIBitsToDevice_pos,
                "gdi32/bitmap",
                "SetDIBitsToDevice",
                &[
                    ("hdc", &hdc),
                    ("xDst", &xDst),
                    ("yDst", &yDst),
                    ("w", &w),
                    ("h", &h),
                    ("xSrc", &xSrc),
                    ("ySrc", &ySrc),
                    ("StartScan", &StartScan),
                    ("cLines", &cLines),
                    ("lpBits", &lpBits),
                    ("lpBmi", &lpBmi),
                    ("iUsage", &iUsage),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetDIBitsToDevice(
            machine, hdc, xDst, yDst, w, h, xSrc, ySrc, StartScan, cLines, lpBits, lpBmi, iUsage,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetLayout(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let l = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/dc") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetLayout_pos,
                "gdi32/dc",
                "SetLayout",
                &[("hdc", &hdc), ("l", &l)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetLayout(machine, hdc, l);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetPaletteEntries(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
        let iStart = <u32>::from_stack(mem, stack_args + 4u32);
        let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
        let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/palette") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetPaletteEntries_pos,
                "gdi32/palette",
                "SetPaletteEntries",
                &[
                    ("hpal", &hpal),
                    ("iStart", &iStart),
                    ("cEntries", &cEntries),
                    ("pPalEntries", &pPalEntries),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetPaletteEntries(machine, hpal, iStart, cEntries, pPalEntries);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetPixel(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <u32>::from_stack(mem, stack_args + 4u32);
        let y = <u32>::from_stack(mem, stack_args + 8u32);
        let color = <COLORREF>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetPixel_pos,
                "gdi32/draw",
                "SetPixel",
                &[("hdc", &hdc), ("x", &x), ("y", &y), ("color", &color)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetPixel(machine, hdc, x, y, color);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetROP2(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let rop2 = <Result<R2, u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/draw") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetROP2_pos,
                "gdi32/draw",
                "SetROP2",
                &[("hdc", &hdc), ("rop2", &rop2)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetROP2(machine, hdc, rop2);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetTextAlign(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let fMode = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetTextAlign_pos,
                "gdi32/text",
                "SetTextAlign",
                &[("hdc", &hdc), ("fMode", &fMode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetTextAlign(machine, hdc, fMode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn SetTextColor(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::SetTextColor_pos,
                "gdi32/text",
                "SetTextColor",
                &[("hdc", &hdc), ("color", &color)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::SetTextColor(machine, hdc, color);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn StretchBlt(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdcDst = <HDC>::from_stack(mem, stack_args + 0u32);
        let xDst = <i32>::from_stack(mem, stack_args + 4u32);
        let yDst = <i32>::from_stack(mem, stack_args + 8u32);
        let wDst = <i32>::from_stack(mem, stack_args + 12u32);
        let hDst = <i32>::from_stack(mem, stack_args + 16u32);
        let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
        let xSrc = <i32>::from_stack(mem, stack_args + 24u32);
        let ySrc = <i32>::from_stack(mem, stack_args + 28u32);
        let wSrc = <i32>::from_stack(mem, stack_args + 32u32);
        let hSrc = <i32>::from_stack(mem, stack_args + 36u32);
        let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 40u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::StretchBlt_pos,
                "gdi32/bitmap",
                "StretchBlt",
                &[
                    ("hdcDst", &hdcDst),
                    ("xDst", &xDst),
                    ("yDst", &yDst),
                    ("wDst", &wDst),
                    ("hDst", &hDst),
                    ("hdcSrc", &hdcSrc),
                    ("xSrc", &xSrc),
                    ("ySrc", &ySrc),
                    ("wSrc", &wSrc),
                    ("hSrc", &hSrc),
                    ("rop", &rop),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::StretchBlt(
            machine, hdcDst, xDst, yDst, wDst, hDst, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn StretchDIBits(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let xDst = <i32>::from_stack(mem, stack_args + 4u32);
        let yDst = <i32>::from_stack(mem, stack_args + 8u32);
        let wDst = <i32>::from_stack(mem, stack_args + 12u32);
        let hDst = <i32>::from_stack(mem, stack_args + 16u32);
        let xSrc = <i32>::from_stack(mem, stack_args + 20u32);
        let ySrc = <i32>::from_stack(mem, stack_args + 24u32);
        let wSrc = <i32>::from_stack(mem, stack_args + 28u32);
        let hSrc = <i32>::from_stack(mem, stack_args + 32u32);
        let lpBits = <u32>::from_stack(mem, stack_args + 36u32);
        let lpBmi = <u32>::from_stack(mem, stack_args + 40u32);
        let iUsage = <u32>::from_stack(mem, stack_args + 44u32);
        let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 48u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/bitmap") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::StretchDIBits_pos,
                "gdi32/bitmap",
                "StretchDIBits",
                &[
                    ("hdc", &hdc),
                    ("xDst", &xDst),
                    ("yDst", &yDst),
                    ("wDst", &wDst),
                    ("hDst", &hDst),
                    ("xSrc", &xSrc),
                    ("ySrc", &ySrc),
                    ("wSrc", &wSrc),
                    ("hSrc", &hSrc),
                    ("lpBits", &lpBits),
                    ("lpBmi", &lpBmi),
                    ("iUsage", &iUsage),
                    ("rop", &rop),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::StretchDIBits(
            machine, hdc, xDst, yDst, wDst, hDst, xSrc, ySrc, wSrc, hSrc, lpBits, lpBmi, iUsage,
            rop,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn TextOutA(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <u32>::from_stack(mem, stack_args + 4u32);
        let y = <u32>::from_stack(mem, stack_args + 8u32);
        let lpString = <Array<u8>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::TextOutA_pos,
                "gdi32/text",
                "TextOutA",
                &[("hdc", &hdc), ("x", &x), ("y", &y), ("lpString", &lpString)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::TextOutA(machine, hdc, x, y, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn TextOutW(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
        let x = <u32>::from_stack(mem, stack_args + 4u32);
        let y = <u32>::from_stack(mem, stack_args + 8u32);
        let lpString = <Array<u16>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("gdi32/text") {
            crate::winapi::trace::Record::new(
                winapi::gdi32::TextOutW_pos,
                "gdi32/text",
                "TextOutW",
                &[("hdc", &hdc), ("x", &x), ("y", &y), ("lpString", &lpString)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::gdi32::TextOutW(machine, hdc, x, y, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
}
const SHIMS: [Shim; 48usize] = [
    Shim {
        name: "BitBlt",
        func: Handler::Sync(wrappers::BitBlt),
    },
    Shim {
        name: "CreateBitmap",
        func: Handler::Sync(wrappers::CreateBitmap),
    },
    Shim {
        name: "CreateCompatibleBitmap",
        func: Handler::Sync(wrappers::CreateCompatibleBitmap),
    },
    Shim {
        name: "CreateCompatibleDC",
        func: Handler::Sync(wrappers::CreateCompatibleDC),
    },
    Shim {
        name: "CreateDIBSection",
        func: Handler::Sync(wrappers::CreateDIBSection),
    },
    Shim {
        name: "CreateDIBitmap",
        func: Handler::Sync(wrappers::CreateDIBitmap),
    },
    Shim {
        name: "CreateFontA",
        func: Handler::Sync(wrappers::CreateFontA),
    },
    Shim {
        name: "CreatePalette",
        func: Handler::Sync(wrappers::CreatePalette),
    },
    Shim {
        name: "CreatePen",
        func: Handler::Sync(wrappers::CreatePen),
    },
    Shim {
        name: "CreateSolidBrush",
        func: Handler::Sync(wrappers::CreateSolidBrush),
    },
    Shim {
        name: "DeleteDC",
        func: Handler::Sync(wrappers::DeleteDC),
    },
    Shim {
        name: "DeleteObject",
        func: Handler::Sync(wrappers::DeleteObject),
    },
    Shim {
        name: "EnumFontFamiliesExA",
        func: Handler::Sync(wrappers::EnumFontFamiliesExA),
    },
    Shim {
        name: "GetDCOrgEx",
        func: Handler::Sync(wrappers::GetDCOrgEx),
    },
    Shim {
        name: "GetDIBits",
        func: Handler::Sync(wrappers::GetDIBits),
    },
    Shim {
        name: "GetDeviceCaps",
        func: Handler::Sync(wrappers::GetDeviceCaps),
    },
    Shim {
        name: "GetLayout",
        func: Handler::Sync(wrappers::GetLayout),
    },
    Shim {
        name: "GetObjectA",
        func: Handler::Sync(wrappers::GetObjectA),
    },
    Shim {
        name: "GetPaletteEntries",
        func: Handler::Sync(wrappers::GetPaletteEntries),
    },
    Shim {
        name: "GetPixel",
        func: Handler::Sync(wrappers::GetPixel),
    },
    Shim {
        name: "GetStockObject",
        func: Handler::Sync(wrappers::GetStockObject),
    },
    Shim {
        name: "GetSystemPaletteEntries",
        func: Handler::Sync(wrappers::GetSystemPaletteEntries),
    },
    Shim {
        name: "GetTextExtentPoint32A",
        func: Handler::Sync(wrappers::GetTextExtentPoint32A),
    },
    Shim {
        name: "GetTextExtentPoint32W",
        func: Handler::Sync(wrappers::GetTextExtentPoint32W),
    },
    Shim {
        name: "GetTextMetricsA",
        func: Handler::Sync(wrappers::GetTextMetricsA),
    },
    Shim {
        name: "GetTextMetricsW",
        func: Handler::Sync(wrappers::GetTextMetricsW),
    },
    Shim {
        name: "LineDDA",
        func: Handler::Sync(wrappers::LineDDA),
    },
    Shim {
        name: "LineTo",
        func: Handler::Sync(wrappers::LineTo),
    },
    Shim {
        name: "MoveToEx",
        func: Handler::Sync(wrappers::MoveToEx),
    },
    Shim {
        name: "PatBlt",
        func: Handler::Sync(wrappers::PatBlt),
    },
    Shim {
        name: "PtVisible",
        func: Handler::Sync(wrappers::PtVisible),
    },
    Shim {
        name: "RealizePalette",
        func: Handler::Sync(wrappers::RealizePalette),
    },
    Shim {
        name: "SelectObject",
        func: Handler::Sync(wrappers::SelectObject),
    },
    Shim {
        name: "SelectPalette",
        func: Handler::Sync(wrappers::SelectPalette),
    },
    Shim {
        name: "SetBkColor",
        func: Handler::Sync(wrappers::SetBkColor),
    },
    Shim {
        name: "SetBkMode",
        func: Handler::Sync(wrappers::SetBkMode),
    },
    Shim {
        name: "SetBrushOrgEx",
        func: Handler::Sync(wrappers::SetBrushOrgEx),
    },
    Shim {
        name: "SetDIBitsToDevice",
        func: Handler::Sync(wrappers::SetDIBitsToDevice),
    },
    Shim {
        name: "SetLayout",
        func: Handler::Sync(wrappers::SetLayout),
    },
    Shim {
        name: "SetPaletteEntries",
        func: Handler::Sync(wrappers::SetPaletteEntries),
    },
    Shim {
        name: "SetPixel",
        func: Handler::Sync(wrappers::SetPixel),
    },
    Shim {
        name: "SetROP2",
        func: Handler::Sync(wrappers::SetROP2),
    },
    Shim {
        name: "SetTextAlign",
        func: Handler::Sync(wrappers::SetTextAlign),
    },
    Shim {
        name: "SetTextColor",
        func: Handler::Sync(wrappers::SetTextColor),
    },
    Shim {
        name: "StretchBlt",
        func: Handler::Sync(wrappers::StretchBlt),
    },
    Shim {
        name: "StretchDIBits",
        func: Handler::Sync(wrappers::StretchDIBits),
    },
    Shim {
        name: "TextOutA",
        func: Handler::Sync(wrappers::TextOutA),
    },
    Shim {
        name: "TextOutW",
        func: Handler::Sync(wrappers::TextOutW),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "gdi32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/gdi32.dll"),
};
