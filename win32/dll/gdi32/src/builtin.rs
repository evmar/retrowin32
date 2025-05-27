#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as gdi32;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn BitBlt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdcDst = <HDC>::from_stack(mem, stack_args + 0u32);
            let xDst = <i32>::from_stack(mem, stack_args + 4u32);
            let yDst = <i32>::from_stack(mem, stack_args + 8u32);
            let w = <i32>::from_stack(mem, stack_args + 12u32);
            let h = <i32>::from_stack(mem, stack_args + 16u32);
            let hdcSrc = <HDC>::from_stack(mem, stack_args + 20u32);
            let xSrc = <i32>::from_stack(mem, stack_args + 24u32);
            let ySrc = <i32>::from_stack(mem, stack_args + 28u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 32u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::BitBlt_pos,
                    "gdi32/bitmap_api",
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
                gdi32::bitmap_api::BitBlt(sys, hdcDst, xDst, yDst, w, h, hdcSrc, xSrc, ySrc, rop);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateBitmap(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let nWidth = <u32>::from_stack(mem, stack_args + 0u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 4u32);
            let nPlanes = <u32>::from_stack(mem, stack_args + 8u32);
            let nBitCount = <u32>::from_stack(mem, stack_args + 12u32);
            let lpBits = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::CreateBitmap_pos,
                    "gdi32/bitmap_api",
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
                gdi32::bitmap_api::CreateBitmap(sys, nWidth, nHeight, nPlanes, nBitCount, lpBits);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateCompatibleBitmap(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let cx = <u32>::from_stack(mem, stack_args + 4u32);
            let cy = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::CreateCompatibleBitmap_pos,
                    "gdi32/bitmap_api",
                    "CreateCompatibleBitmap",
                    &[("hdc", &hdc), ("cx", &cx), ("cy", &cy)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::bitmap_api::CreateCompatibleBitmap(sys, hdc, cx, cy);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateCompatibleDC(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::dc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/dc") {
                trace::Record::new(
                    gdi32::dc::CreateCompatibleDC_pos,
                    "gdi32/dc",
                    "CreateCompatibleDC",
                    &[("hdc", &hdc)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::dc::CreateCompatibleDC(sys, hdc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateDIBSection(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
            let usage = <u32>::from_stack(mem, stack_args + 8u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let hSection = <u32>::from_stack(mem, stack_args + 16u32);
            let offset = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::CreateDIBSection_pos,
                    "gdi32/bitmap_api",
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
            let result = gdi32::bitmap_api::CreateDIBSection(
                sys, hdc, pbmi, usage, ppvBits, hSection, offset,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateDIBitmap(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let pbmih = <Option<&mut BITMAPINFOHEADER>>::from_stack(mem, stack_args + 4u32);
            let flInit = <u32>::from_stack(mem, stack_args + 8u32);
            let pjBits = <Option<&mut u8>>::from_stack(mem, stack_args + 12u32);
            let pbmi = <Option<&mut BITMAPINFO>>::from_stack(mem, stack_args + 16u32);
            let iUsage = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::CreateDIBitmap_pos,
                    "gdi32/bitmap_api",
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
                gdi32::bitmap_api::CreateDIBitmap(sys, hdc, pbmih, flInit, pjBits, pbmi, iUsage);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateFontA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
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
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::CreateFontA_pos,
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
            let result = gdi32::text::CreateFontA(
                sys,
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
    }
    pub unsafe fn CreatePalette(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let plpal = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::CreatePalette_pos,
                    "gdi32/palette",
                    "CreatePalette",
                    &[("plpal", &plpal)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::palette::CreatePalette(sys, plpal);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreatePen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, stack_args + 0u32);
            let cWidth = <u32>::from_stack(mem, stack_args + 4u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::CreatePen_pos,
                    "gdi32/draw",
                    "CreatePen",
                    &[("iStyle", &iStyle), ("cWidth", &cWidth), ("color", &color)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::CreatePen(sys, iStyle, cWidth, color);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateSolidBrush(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let color = <COLORREF>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::CreateSolidBrush_pos,
                    "gdi32/draw",
                    "CreateSolidBrush",
                    &[("color", &color)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::CreateSolidBrush(sys, color);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DeleteDC(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::dc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/dc") {
                trace::Record::new(
                    gdi32::dc::DeleteDC_pos,
                    "gdi32/dc",
                    "DeleteDC",
                    &[("hdc", &hdc)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::dc::DeleteDC(sys, hdc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DeleteObject(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::object::*;
        unsafe {
            let mem = sys.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/object") {
                trace::Record::new(
                    gdi32::object::DeleteObject_pos,
                    "gdi32/object",
                    "DeleteObject",
                    &[("handle", &handle)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::object::DeleteObject(sys, handle);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EnumFontFamiliesExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpLogfont = <Option<&mut LOGFONTA>>::from_stack(mem, stack_args + 4u32);
            let lpProc = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <LPARAM>::from_stack(mem, stack_args + 12u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::EnumFontFamiliesExA_pos,
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
                gdi32::text::EnumFontFamiliesExA(sys, hdc, lpLogfont, lpProc, lParam, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDCOrgEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::dc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/dc") {
                trace::Record::new(
                    gdi32::dc::GetDCOrgEx_pos,
                    "gdi32/dc",
                    "GetDCOrgEx",
                    &[("hdc", &hdc), ("lpPoint", &lpPoint)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::dc::GetDCOrgEx(sys, hdc, lpPoint);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDIBits(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hbm = <HBITMAP>::from_stack(mem, stack_args + 4u32);
            let start = <u32>::from_stack(mem, stack_args + 8u32);
            let cLines = <u32>::from_stack(mem, stack_args + 12u32);
            let lpvBits = <Option<&mut u8>>::from_stack(mem, stack_args + 16u32);
            let lpbmi = <Option<&mut BITMAPINFO>>::from_stack(mem, stack_args + 20u32);
            let usage = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::GetDIBits_pos,
                    "gdi32/bitmap_api",
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
                gdi32::bitmap_api::GetDIBits(sys, hdc, hbm, start, cLines, lpvBits, lpbmi, usage);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDeviceCaps(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::dc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/dc") {
                trace::Record::new(
                    gdi32::dc::GetDeviceCaps_pos,
                    "gdi32/dc",
                    "GetDeviceCaps",
                    &[("hdc", &hdc), ("index", &index)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::dc::GetDeviceCaps(sys, hdc, index);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLayout(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::dc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/dc") {
                trace::Record::new(
                    gdi32::dc::GetLayout_pos,
                    "gdi32/dc",
                    "GetLayout",
                    &[("hdc", &hdc)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::dc::GetLayout(sys, hdc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetObjectA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::object::*;
        unsafe {
            let mem = sys.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, stack_args + 0u32);
            let bytes = <u32>::from_stack(mem, stack_args + 4u32);
            let out = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/object") {
                trace::Record::new(
                    gdi32::object::GetObjectA_pos,
                    "gdi32/object",
                    "GetObjectA",
                    &[("handle", &handle), ("bytes", &bytes), ("out", &out)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::object::GetObjectA(sys, handle, bytes, out);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetPaletteEntries(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
            let iStart = <u32>::from_stack(mem, stack_args + 4u32);
            let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
            let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::GetPaletteEntries_pos,
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
            let result =
                gdi32::palette::GetPaletteEntries(sys, hpal, iStart, cEntries, pPalEntries);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetPixel(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::GetPixel_pos,
                    "gdi32/draw",
                    "GetPixel",
                    &[("hdc", &hdc), ("x", &x), ("y", &y)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::GetPixel(sys, hdc, x, y);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetStockObject(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::object::*;
        unsafe {
            let mem = sys.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/object") {
                trace::Record::new(
                    gdi32::object::GetStockObject_pos,
                    "gdi32/object",
                    "GetStockObject",
                    &[("i", &i)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::object::GetStockObject(sys, i);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSystemPaletteEntries(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let iStart = <u32>::from_stack(mem, stack_args + 4u32);
            let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
            let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::GetSystemPaletteEntries_pos,
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
                gdi32::palette::GetSystemPaletteEntries(sys, hdc, iStart, cEntries, pPalEntries);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetTextExtentPoint32A(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let c = <i32>::from_stack(mem, stack_args + 8u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::GetTextExtentPoint32A_pos,
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
            let result = gdi32::text::GetTextExtentPoint32A(sys, hdc, lpString, c, psizl);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetTextExtentPoint32W(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let c = <i32>::from_stack(mem, stack_args + 8u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::GetTextExtentPoint32W_pos,
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
            let result = gdi32::text::GetTextExtentPoint32W(sys, hdc, lpString, c, psizl);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetTextMetricsA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::GetTextMetricsA_pos,
                    "gdi32/text",
                    "GetTextMetricsA",
                    &[("hdc", &hdc), ("lptm", &lptm)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::text::GetTextMetricsA(sys, hdc, lptm);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetTextMetricsW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::GetTextMetricsW_pos,
                    "gdi32/text",
                    "GetTextMetricsW",
                    &[("hdc", &hdc), ("lptm", &lptm)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::text::GetTextMetricsW(sys, hdc, lptm);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LineDDA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let xStart = <i32>::from_stack(mem, stack_args + 0u32);
            let yStart = <i32>::from_stack(mem, stack_args + 4u32);
            let xEnd = <i32>::from_stack(mem, stack_args + 8u32);
            let yEnd = <i32>::from_stack(mem, stack_args + 12u32);
            let lpProc = <u32>::from_stack(mem, stack_args + 16u32);
            let data = <u32>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::LineDDA_pos,
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
            let result = gdi32::draw::LineDDA(sys, xStart, yStart, xEnd, yEnd, lpProc, data);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LineTo(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::LineTo_pos,
                    "gdi32/draw",
                    "LineTo",
                    &[("hdc", &hdc), ("x", &x), ("y", &y)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::LineTo(sys, hdc, x, y);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MoveToEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::MoveToEx_pos,
                    "gdi32/draw",
                    "MoveToEx",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lppt", &lppt)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::MoveToEx(sys, hdc, x, y, lppt);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PatBlt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let w = <i32>::from_stack(mem, stack_args + 12u32);
            let h = <i32>::from_stack(mem, stack_args + 16u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::PatBlt_pos,
                    "gdi32/bitmap_api",
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
            let result = gdi32::bitmap_api::PatBlt(sys, hdc, x, y, w, h, rop);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PtVisible(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::PtVisible_pos,
                    "gdi32/draw",
                    "PtVisible",
                    &[("hdc", &hdc), ("x", &x), ("y", &y)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::PtVisible(sys, hdc, x, y);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RealizePalette(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::RealizePalette_pos,
                    "gdi32/palette",
                    "RealizePalette",
                    &[("hdc", &hdc)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::palette::RealizePalette(sys, hdc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ResizePalette(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
            let n = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::ResizePalette_pos,
                    "gdi32/palette",
                    "ResizePalette",
                    &[("hpal", &hpal), ("n", &n)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::palette::ResizePalette(sys, hpal, n);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SelectObject(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::object::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/object") {
                trace::Record::new(
                    gdi32::object::SelectObject_pos,
                    "gdi32/object",
                    "SelectObject",
                    &[("hdc", &hdc), ("hGdiObj", &hGdiObj)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::object::SelectObject(sys, hdc, hGdiObj);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SelectPalette(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let hPal = <HPALETTE>::from_stack(mem, stack_args + 4u32);
            let bForceBkgd = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::SelectPalette_pos,
                    "gdi32/palette",
                    "SelectPalette",
                    &[("hdc", &hdc), ("hPal", &hPal), ("bForceBkgd", &bForceBkgd)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::palette::SelectPalette(sys, hdc, hPal, bForceBkgd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetBkColor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::SetBkColor_pos,
                    "gdi32/draw",
                    "SetBkColor",
                    &[("hdc", &hdc), ("color", &color)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::SetBkColor(sys, hdc, color);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetBkMode(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let mode = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::SetBkMode_pos,
                    "gdi32/draw",
                    "SetBkMode",
                    &[("hdc", &hdc), ("mode", &mode)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::SetBkMode(sys, hdc, mode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetBrushOrgEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <i32>::from_stack(mem, stack_args + 4u32);
            let y = <i32>::from_stack(mem, stack_args + 8u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::SetBrushOrgEx_pos,
                    "gdi32/draw",
                    "SetBrushOrgEx",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lppt", &lppt)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::SetBrushOrgEx(sys, hdc, x, y, lppt);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetDIBitsToDevice(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
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
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::SetDIBitsToDevice_pos,
                    "gdi32/bitmap_api",
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
            let result = gdi32::bitmap_api::SetDIBitsToDevice(
                sys, hdc, xDst, yDst, w, h, xSrc, ySrc, StartScan, cLines, lpBits, lpBmi, iUsage,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetLayout(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::dc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let l = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/dc") {
                trace::Record::new(
                    gdi32::dc::SetLayout_pos,
                    "gdi32/dc",
                    "SetLayout",
                    &[("hdc", &hdc), ("l", &l)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::dc::SetLayout(sys, hdc, l);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetPaletteEntries(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hpal = <HPALETTE>::from_stack(mem, stack_args + 0u32);
            let iStart = <u32>::from_stack(mem, stack_args + 4u32);
            let cEntries = <u32>::from_stack(mem, stack_args + 8u32);
            let pPalEntries = <Option<&mut PALETTEENTRY>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::SetPaletteEntries_pos,
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
            let result =
                gdi32::palette::SetPaletteEntries(sys, hpal, iStart, cEntries, pPalEntries);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetPixel(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::SetPixel_pos,
                    "gdi32/draw",
                    "SetPixel",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("color", &color)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::SetPixel(sys, hdc, x, y, color);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetROP2(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::draw::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/draw") {
                trace::Record::new(
                    gdi32::draw::SetROP2_pos,
                    "gdi32/draw",
                    "SetROP2",
                    &[("hdc", &hdc), ("rop2", &rop2)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::draw::SetROP2(sys, hdc, rop2);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetSystemPaletteUse(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::palette::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let use_ = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/palette") {
                trace::Record::new(
                    gdi32::palette::SetSystemPaletteUse_pos,
                    "gdi32/palette",
                    "SetSystemPaletteUse",
                    &[("hdc", &hdc), ("use_", &use_)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::palette::SetSystemPaletteUse(sys, hdc, use_);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetTextAlign(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let fMode = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::SetTextAlign_pos,
                    "gdi32/text",
                    "SetTextAlign",
                    &[("hdc", &hdc), ("fMode", &fMode)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::text::SetTextAlign(sys, hdc, fMode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetTextColor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let color = <COLORREF>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::SetTextColor_pos,
                    "gdi32/text",
                    "SetTextColor",
                    &[("hdc", &hdc), ("color", &color)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::text::SetTextColor(sys, hdc, color);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn StretchBlt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
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
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::StretchBlt_pos,
                    "gdi32/bitmap_api",
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
            let result = gdi32::bitmap_api::StretchBlt(
                sys, hdcDst, xDst, yDst, wDst, hDst, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn StretchDIBits(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::bitmap_api::*;
        unsafe {
            let mem = sys.mem().detach();
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
            let __trace_record = if trace::enabled("gdi32/bitmap_api") {
                trace::Record::new(
                    gdi32::bitmap_api::StretchDIBits_pos,
                    "gdi32/bitmap_api",
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
            let result = gdi32::bitmap_api::StretchDIBits(
                sys, hdc, xDst, yDst, wDst, hDst, xSrc, ySrc, wSrc, hSrc, lpBits, lpBmi, iUsage,
                rop,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TextOutA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpString = <Array<u8>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::TextOutA_pos,
                    "gdi32/text",
                    "TextOutA",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::text::TextOutA(sys, hdc, x, y, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TextOutW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use gdi32::text::*;
        unsafe {
            let mem = sys.mem().detach();
            let hdc = <HDC>::from_stack(mem, stack_args + 0u32);
            let x = <u32>::from_stack(mem, stack_args + 4u32);
            let y = <u32>::from_stack(mem, stack_args + 8u32);
            let lpString = <Array<u16>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("gdi32/text") {
                trace::Record::new(
                    gdi32::text::TextOutW_pos,
                    "gdi32/text",
                    "TextOutW",
                    &[("hdc", &hdc), ("x", &x), ("y", &y), ("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = gdi32::text::TextOutW(sys, hdc, x, y, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 50usize] = [
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
        name: "ResizePalette",
        func: Handler::Sync(wrappers::ResizePalette),
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
        name: "SetSystemPaletteUse",
        func: Handler::Sync(wrappers::SetSystemPaletteUse),
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
    raw: std::include_bytes!("../gdi32.dll"),
};
