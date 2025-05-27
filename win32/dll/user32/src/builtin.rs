#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as user32;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn AdjustWindowRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dwStyle = <Result<WS, u32>>::from_stack(mem, stack_args + 4u32);
            let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::AdjustWindowRect_pos,
                    "user32/window",
                    "AdjustWindowRect",
                    &[
                        ("lpRect", &lpRect),
                        ("dwStyle", &dwStyle),
                        ("bMenu", &bMenu),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::AdjustWindowRect(sys, lpRect, dwStyle, bMenu);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn AdjustWindowRectEx(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dwStyle = <Result<WS, u32>>::from_stack(mem, stack_args + 4u32);
            let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
            let dwExStyle = <Result<WS_EX, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::AdjustWindowRectEx_pos,
                    "user32/window",
                    "AdjustWindowRectEx",
                    &[
                        ("lpRect", &lpRect),
                        ("dwStyle", &dwStyle),
                        ("bMenu", &bMenu),
                        ("dwExStyle", &dwExStyle),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::AdjustWindowRectEx(sys, lpRect, dwStyle, bMenu, dwExStyle);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn AppendMenuA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uFlags = <u32>::from_stack(mem, stack_args + 4u32);
            let uIDNewItem = <u32>::from_stack(mem, stack_args + 8u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::AppendMenuA_pos,
                    "user32/menu",
                    "AppendMenuA",
                    &[
                        ("hMenu", &hMenu),
                        ("uFlags", &uFlags),
                        ("uIDNewItem", &uIDNewItem),
                        ("lpNewItem", &lpNewItem),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::AppendMenuA(sys, hMenu, uFlags, uIDNewItem, lpNewItem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn BeginPaint(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::BeginPaint_pos,
                    "user32/paint",
                    "BeginPaint",
                    &[("hWnd", &hWnd), ("lpPaint", &lpPaint)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::BeginPaint(sys, hWnd, lpPaint);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CallWindowProcA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpPrevWndFunc = <u32>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let Msg = <u32>::from_stack(mem, stack_args + 8u32);
            let wParam = <u32>::from_stack(mem, stack_args + 12u32);
            let lParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::CallWindowProcA_pos,
                    "user32/message",
                    "CallWindowProcA",
                    &[
                        ("lpPrevWndFunc", &lpPrevWndFunc),
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                user32::message::CallWindowProcA(sys, lpPrevWndFunc, hWnd, Msg, wParam, lParam);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CharLowerA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpsz = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::CharLowerA_pos,
                    "user32/misc",
                    "CharLowerA",
                    &[("lpsz", &lpsz)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::CharLowerA(sys, lpsz);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CharLowerBuffA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpsz = <u32>::from_stack(mem, stack_args + 0u32);
            let cchLength = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::CharLowerBuffA_pos,
                    "user32/misc",
                    "CharLowerBuffA",
                    &[("lpsz", &lpsz), ("cchLength", &cchLength)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::CharLowerBuffA(sys, lpsz, cchLength);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CheckDlgButton(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
            let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::CheckDlgButton_pos,
                    "user32/dialog",
                    "CheckDlgButton",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDButton", &nIDButton),
                        ("uCheck", &uCheck),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::CheckDlgButton(sys, hDlg, nIDButton, uCheck);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CheckMenuItem(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uIDCheckItem = <u32>::from_stack(mem, stack_args + 4u32);
            let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::CheckMenuItem_pos,
                    "user32/menu",
                    "CheckMenuItem",
                    &[
                        ("hMenu", &hMenu),
                        ("uIDCheckItem", &uIDCheckItem),
                        ("uCheck", &uCheck),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::CheckMenuItem(sys, hMenu, uIDCheckItem, uCheck);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CheckRadioButton(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDFirstButton = <i32>::from_stack(mem, stack_args + 4u32);
            let nIDLastButton = <i32>::from_stack(mem, stack_args + 8u32);
            let nIDCheckButton = <i32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::CheckRadioButton_pos,
                    "user32/dialog",
                    "CheckRadioButton",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDFirstButton", &nIDFirstButton),
                        ("nIDLastButton", &nIDLastButton),
                        ("nIDCheckButton", &nIDCheckButton),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::CheckRadioButton(
                sys,
                hDlg,
                nIDFirstButton,
                nIDLastButton,
                nIDCheckButton,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ClientToScreen(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::ClientToScreen_pos,
                    "user32/window",
                    "ClientToScreen",
                    &[("hWnd", &hWnd), ("lpPoint", &lpPoint)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::ClientToScreen(sys, hWnd, lpPoint);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CopyRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let lprcSrc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::CopyRect_pos,
                    "user32/rect",
                    "CopyRect",
                    &[("lprcDst", &lprcDst), ("lprcSrc", &lprcSrc)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::CopyRect(sys, lprcDst, lprcSrc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateCursor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInst = <u32>::from_stack(mem, stack_args + 0u32);
            let xHotSpot = <u32>::from_stack(mem, stack_args + 4u32);
            let yHotSpot = <u32>::from_stack(mem, stack_args + 8u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
            let pvANDPlane = <u32>::from_stack(mem, stack_args + 20u32);
            let pvXORPlane = <u32>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::CreateCursor_pos,
                    "user32/resource",
                    "CreateCursor",
                    &[
                        ("hInst", &hInst),
                        ("xHotSpot", &xHotSpot),
                        ("yHotSpot", &yHotSpot),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("pvANDPlane", &pvANDPlane),
                        ("pvXORPlane", &pvXORPlane),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::CreateCursor(
                sys, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreatePopupMenu(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::CreatePopupMenu_pos,
                    "user32/menu",
                    "CreatePopupMenu",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::CreatePopupMenu(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn CreateWindowExA(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let dwExStyle = <Result<WS_EX, u32>>::from_stack(mem, stack_args + 0u32);
            let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, stack_args + 4u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let dwStyle = <CreateWindowStyle>::from_stack(mem, stack_args + 12u32);
            let X = <u32>::from_stack(mem, stack_args + 16u32);
            let Y = <u32>::from_stack(mem, stack_args + 20u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 24u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 28u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 32u32);
            let hMenu = <u32>::from_stack(mem, stack_args + 36u32);
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 40u32);
            let lpParam = <u32>::from_stack(mem, stack_args + 44u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::CreateWindowExA_pos,
                    "user32/window",
                    "CreateWindowExA",
                    &[
                        ("dwExStyle", &dwExStyle),
                        ("lpClassName", &lpClassName),
                        ("lpWindowName", &lpWindowName),
                        ("dwStyle", &dwStyle),
                        ("X", &X),
                        ("Y", &Y),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("hWndParent", &hWndParent),
                        ("hMenu", &hMenu),
                        ("hInstance", &hInstance),
                        ("lpParam", &lpParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::CreateWindowExA(
                    sys,
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
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn CreateWindowExW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let dwExStyle = <Result<WS_EX, u32>>::from_stack(mem, stack_args + 0u32);
            let lpClassName =
                <CreateWindowClassName<'_, Str16>>::from_stack(mem, stack_args + 4u32);
            let lpWindowName = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let dwStyle = <CreateWindowStyle>::from_stack(mem, stack_args + 12u32);
            let X = <u32>::from_stack(mem, stack_args + 16u32);
            let Y = <u32>::from_stack(mem, stack_args + 20u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 24u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 28u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 32u32);
            let hMenu = <u32>::from_stack(mem, stack_args + 36u32);
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 40u32);
            let lpParam = <u32>::from_stack(mem, stack_args + 44u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::CreateWindowExW_pos,
                    "user32/window",
                    "CreateWindowExW",
                    &[
                        ("dwExStyle", &dwExStyle),
                        ("lpClassName", &lpClassName),
                        ("lpWindowName", &lpWindowName),
                        ("dwStyle", &dwStyle),
                        ("X", &X),
                        ("Y", &Y),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("hWndParent", &hWndParent),
                        ("hMenu", &hMenu),
                        ("hInstance", &hInstance),
                        ("lpParam", &lpParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::CreateWindowExW(
                    sys,
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
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn DefWindowProcA(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::DefWindowProcA_pos,
                    "user32/window",
                    "DefWindowProcA",
                    &[
                        ("hWnd", &hWnd),
                        ("msg", &msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::DefWindowProcA(sys, hWnd, msg, wParam, lParam).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn DefWindowProcW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::DefWindowProcW_pos,
                    "user32/window",
                    "DefWindowProcW",
                    &[
                        ("hWnd", &hWnd),
                        ("msg", &msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::DefWindowProcW(sys, hWnd, msg, wParam, lParam).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn DeleteMenu(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uPosition = <u32>::from_stack(mem, stack_args + 4u32);
            let uFlags = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::DeleteMenu_pos,
                    "user32/menu",
                    "DeleteMenu",
                    &[
                        ("hMenu", &hMenu),
                        ("uPosition", &uPosition),
                        ("uFlags", &uFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::DeleteMenu(sys, hMenu, uPosition, uFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DestroyWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::DestroyWindow_pos,
                    "user32/window",
                    "DestroyWindow",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::DestroyWindow(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DialogBoxIndirectParamA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let hDialogTemplate = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::DialogBoxIndirectParamA_pos,
                    "user32/dialog",
                    "DialogBoxIndirectParamA",
                    &[
                        ("hInstance", &hInstance),
                        ("hDialogTemplate", &hDialogTemplate),
                        ("hWndParent", &hWndParent),
                        ("lpDialogFunc", &lpDialogFunc),
                        ("dwInitParam", &dwInitParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::DialogBoxIndirectParamA(
                sys,
                hInstance,
                hDialogTemplate,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DialogBoxParamA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::DialogBoxParamA_pos,
                    "user32/dialog",
                    "DialogBoxParamA",
                    &[
                        ("hInstance", &hInstance),
                        ("lpTemplateName", &lpTemplateName),
                        ("hWndParent", &hWndParent),
                        ("lpDialogFunc", &lpDialogFunc),
                        ("dwInitParam", &dwInitParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::DialogBoxParamA(
                sys,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DialogBoxParamW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
            let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
            let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::DialogBoxParamW_pos,
                    "user32/dialog",
                    "DialogBoxParamW",
                    &[
                        ("hInstance", &hInstance),
                        ("lpTemplateName", &lpTemplateName),
                        ("hWndParent", &hWndParent),
                        ("lpDialogFunc", &lpDialogFunc),
                        ("dwInitParam", &dwInitParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::DialogBoxParamW(
                sys,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DispatchMessageA(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::DispatchMessageA_pos,
                    "user32/message",
                    "DispatchMessageA",
                    &[("lpMsg", &lpMsg)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::message::DispatchMessageA(sys, lpMsg).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn DispatchMessageW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::DispatchMessageW_pos,
                    "user32/message",
                    "DispatchMessageW",
                    &[("lpMsg", &lpMsg)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::message::DispatchMessageW(sys, lpMsg).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn DrawMenuBar(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::DrawMenuBar_pos,
                    "user32/menu",
                    "DrawMenuBar",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::DrawMenuBar(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DrawTextW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let nCount = <i32>::from_stack(mem, stack_args + 8u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
            let uFormat = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::DrawTextW_pos,
                    "user32/paint",
                    "DrawTextW",
                    &[
                        ("hDC", &hDC),
                        ("lpString", &lpString),
                        ("nCount", &nCount),
                        ("lpRect", &lpRect),
                        ("uFormat", &uFormat),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::DrawTextW(sys, hDC, lpString, nCount, lpRect, uFormat);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EnableMenuItem(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let uIDEnableItem = <u32>::from_stack(mem, stack_args + 4u32);
            let uEnable = <Result<MF, u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::EnableMenuItem_pos,
                    "user32/menu",
                    "EnableMenuItem",
                    &[
                        ("hMenu", &hMenu),
                        ("uIDEnableItem", &uIDEnableItem),
                        ("uEnable", &uEnable),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::EnableMenuItem(sys, hMenu, uIDEnableItem, uEnable);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EnableWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let bEnable = <bool>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::EnableWindow_pos,
                    "user32/window",
                    "EnableWindow",
                    &[("hWnd", &hWnd), ("bEnable", &bEnable)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::EnableWindow(sys, hWnd, bEnable);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EndDialog(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nResult = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::EndDialog_pos,
                    "user32/dialog",
                    "EndDialog",
                    &[("hDlg", &hDlg), ("nResult", &nResult)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::EndDialog(sys, hDlg, nResult);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn EndPaint(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::EndPaint_pos,
                    "user32/paint",
                    "EndPaint",
                    &[("hWnd", &hWnd), ("lpPaint", &lpPaint)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::EndPaint(sys, hWnd, lpPaint);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FillRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let hbr = <BrushOrColor>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::FillRect_pos,
                    "user32/paint",
                    "FillRect",
                    &[("hDC", &hDC), ("lprc", &lprc), ("hbr", &hbr)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::FillRect(sys, hDC, lprc, hbr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FindWindowA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::FindWindowA_pos,
                    "user32/window",
                    "FindWindowA",
                    &[
                        ("lpClassName", &lpClassName),
                        ("lpWindowName", &lpWindowName),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::FindWindowA(sys, lpClassName, lpWindowName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn FrameRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let hbr = <HBRUSH>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::FrameRect_pos,
                    "user32/paint",
                    "FrameRect",
                    &[("hDC", &hDC), ("lprc", &lprc), ("hbr", &hbr)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::FrameRect(sys, hDC, lprc, hbr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetActiveWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetActiveWindow_pos,
                    "user32/window",
                    "GetActiveWindow",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetActiveWindow(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCapture(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetCapture_pos,
                    "user32/window",
                    "GetCapture",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetCapture(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetClassLongA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIndex = <Result<GCL, i32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::GetClassLongA_pos,
                    "user32/wndclass",
                    "GetClassLongA",
                    &[("hWnd", &hWnd), ("nIndex", &nIndex)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::GetClassLongA(sys, hWnd, nIndex);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetClientRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetClientRect_pos,
                    "user32/window",
                    "GetClientRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetClientRect(sys, hWnd, lpRect);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetCursorPos(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::GetCursorPos_pos,
                    "user32/misc",
                    "GetCursorPos",
                    &[("lpPoint", &lpPoint)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::GetCursorPos(sys, lpPoint);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDC(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetDC_pos,
                    "user32/window",
                    "GetDC",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetDC(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDesktopWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetDesktopWindow_pos,
                    "user32/window",
                    "GetDesktopWindow",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetDesktopWindow(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDlgItem(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::GetDlgItem_pos,
                    "user32/dialog",
                    "GetDlgItem",
                    &[("hDlg", &hDlg), ("nIDDlgItem", &nIDDlgItem)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::GetDlgItem(sys, hDlg, nIDDlgItem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDlgItemInt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpTranslated = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let bSigned = <bool>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::GetDlgItemInt_pos,
                    "user32/dialog",
                    "GetDlgItemInt",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpTranslated", &lpTranslated),
                        ("bSigned", &bSigned),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                user32::dialog::GetDlgItemInt(sys, hDlg, nIDDlgItem, lpTranslated, bSigned);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetDlgItemTextW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <ArrayOut<u16>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::GetDlgItemTextW_pos,
                    "user32/dialog",
                    "GetDlgItemTextW",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpString", &lpString),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::GetDlgItemTextW(sys, hDlg, nIDDlgItem, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetFocus(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetFocus_pos,
                    "user32/window",
                    "GetFocus",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetFocus(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetForegroundWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetForegroundWindow_pos,
                    "user32/window",
                    "GetForegroundWindow",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetForegroundWindow(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetKeyNameTextA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let lParam = <i32>::from_stack(mem, stack_args + 0u32);
            let lpString = <ArrayOut<u8>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::GetKeyNameTextA_pos,
                    "user32/keyboard",
                    "GetKeyNameTextA",
                    &[("lParam", &lParam), ("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::GetKeyNameTextA(sys, lParam, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetKeyState(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let nVirtKey = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::GetKeyState_pos,
                    "user32/keyboard",
                    "GetKeyState",
                    &[("nVirtKey", &nVirtKey)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::GetKeyState(sys, nVirtKey);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetKeyboardLayout(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let idThread = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::GetKeyboardLayout_pos,
                    "user32/keyboard",
                    "GetKeyboardLayout",
                    &[("idThread", &idThread)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::GetKeyboardLayout(sys, idThread);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetKeyboardLayoutList(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let nBuff = <i32>::from_stack(mem, stack_args + 0u32);
            let lpList = <Option<&mut HKL>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::GetKeyboardLayoutList_pos,
                    "user32/keyboard",
                    "GetKeyboardLayoutList",
                    &[("nBuff", &nBuff), ("lpList", &lpList)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::GetKeyboardLayoutList(sys, nBuff, lpList);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetKeyboardState(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpKeyState = <Option<&mut u8>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::GetKeyboardState_pos,
                    "user32/keyboard",
                    "GetKeyboardState",
                    &[("lpKeyState", &lpKeyState)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::GetKeyboardState(sys, lpKeyState);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetKeyboardType(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let nTypeFlag = <i32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::GetKeyboardType_pos,
                    "user32/keyboard",
                    "GetKeyboardType",
                    &[("nTypeFlag", &nTypeFlag)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::GetKeyboardType(sys, nTypeFlag);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetLastActivePopup(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetLastActivePopup_pos,
                    "user32/window",
                    "GetLastActivePopup",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetLastActivePopup(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetMenu(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::GetMenu_pos,
                    "user32/menu",
                    "GetMenu",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::GetMenu(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetMenuItemRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
            let uItem = <u32>::from_stack(mem, stack_args + 8u32);
            let lprcItem = <Option<&mut RECT>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::GetMenuItemRect_pos,
                    "user32/menu",
                    "GetMenuItemRect",
                    &[
                        ("hWnd", &hWnd),
                        ("hMenu", &hMenu),
                        ("uItem", &uItem),
                        ("lprcItem", &lprcItem),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::GetMenuItemRect(sys, hWnd, hMenu, uItem, lprcItem);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetMessageA(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::GetMessageA_pos,
                    "user32/message",
                    "GetMessageA",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    user32::message::GetMessageA(sys, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                        .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn GetMessageW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::GetMessageW_pos,
                    "user32/message",
                    "GetMessageW",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    user32::message::GetMessageW(sys, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                        .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn GetMonitorInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMonitor = <HMONITOR>::from_stack(mem, stack_args + 0u32);
            let lpmi = <Option<&mut MONITORINFO>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::GetMonitorInfoA_pos,
                    "user32/misc",
                    "GetMonitorInfoA",
                    &[("hMonitor", &hMonitor), ("lpmi", &lpmi)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::GetMonitorInfoA(sys, hMonitor, lpmi);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetProcessWindowStation(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::GetProcessWindowStation_pos,
                    "user32/misc",
                    "GetProcessWindowStation",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::GetProcessWindowStation(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetQueueStatus(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let flags = <Result<QS, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::GetQueueStatus_pos,
                    "user32/message",
                    "GetQueueStatus",
                    &[("flags", &flags)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::GetQueueStatus(sys, flags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSubMenu(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let nPos = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::GetSubMenu_pos,
                    "user32/menu",
                    "GetSubMenu",
                    &[("hMenu", &hMenu), ("nPos", &nPos)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::GetSubMenu(sys, hMenu, nPos);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSysColor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let nIndex = <i32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::GetSysColor_pos,
                    "user32/misc",
                    "GetSysColor",
                    &[("nIndex", &nIndex)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::GetSysColor(sys, nIndex);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSystemMenu(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let bRevert = <bool>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::GetSystemMenu_pos,
                    "user32/menu",
                    "GetSystemMenu",
                    &[("hWnd", &hWnd), ("bRevert", &bRevert)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::GetSystemMenu(sys, hWnd, bRevert);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetSystemMetrics(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let nIndex = <Result<SM, u32>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::GetSystemMetrics_pos,
                    "user32/misc",
                    "GetSystemMetrics",
                    &[("nIndex", &nIndex)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::GetSystemMetrics(sys, nIndex);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetUpdateRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::GetUpdateRect_pos,
                    "user32/paint",
                    "GetUpdateRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect), ("bErase", &bErase)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::GetUpdateRect(sys, hWnd, lpRect, bErase);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetUserObjectInformationW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hObj = <u32>::from_stack(mem, stack_args + 0u32);
            let nIndex = <u32>::from_stack(mem, stack_args + 4u32);
            let pvInfo = <u32>::from_stack(mem, stack_args + 8u32);
            let nLength = <u32>::from_stack(mem, stack_args + 12u32);
            let lpnLengthNeeded = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::GetUserObjectInformationW_pos,
                    "user32/misc",
                    "GetUserObjectInformationW",
                    &[
                        ("hObj", &hObj),
                        ("nIndex", &nIndex),
                        ("pvInfo", &pvInfo),
                        ("nLength", &nLength),
                        ("lpnLengthNeeded", &lpnLengthNeeded),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::GetUserObjectInformationW(
                sys,
                hObj,
                nIndex,
                pvInfo,
                nLength,
                lpnLengthNeeded,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetWindowDC(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetWindowDC_pos,
                    "user32/window",
                    "GetWindowDC",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetWindowDC(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetWindowLongA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIndex = <Result<GWL, i32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetWindowLongA_pos,
                    "user32/window",
                    "GetWindowLongA",
                    &[("hWnd", &hWnd), ("nIndex", &nIndex)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetWindowLongA(sys, hWnd, nIndex);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetWindowPlacement(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpwndpl = <Option<&mut WINDOWPLACEMENT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetWindowPlacement_pos,
                    "user32/window",
                    "GetWindowPlacement",
                    &[("hWnd", &hWnd), ("lpwndpl", &lpwndpl)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetWindowPlacement(sys, hWnd, lpwndpl);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn GetWindowRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::GetWindowRect_pos,
                    "user32/window",
                    "GetWindowRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::GetWindowRect(sys, hWnd, lpRect);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InflateRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let dx = <i32>::from_stack(mem, stack_args + 4u32);
            let dy = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::InflateRect_pos,
                    "user32/rect",
                    "InflateRect",
                    &[("lprc", &lprc), ("dx", &dx), ("dy", &dy)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::InflateRect(sys, lprc, dx, dy);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IntersectRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let lprcSrc1 = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let lprcSrc2 = <Option<&RECT>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::IntersectRect_pos,
                    "user32/rect",
                    "IntersectRect",
                    &[
                        ("lprcDst", &lprcDst),
                        ("lprcSrc1", &lprcSrc1),
                        ("lprcSrc2", &lprcSrc2),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::IntersectRect(sys, lprcDst, lprcSrc1, lprcSrc2);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InvalidateRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::InvalidateRect_pos,
                    "user32/paint",
                    "InvalidateRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect), ("bErase", &bErase)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::InvalidateRect(sys, hWnd, lpRect, bErase);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InvalidateRgn(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hRgn = <HRGN>::from_stack(mem, stack_args + 4u32);
            let bErase = <bool>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::InvalidateRgn_pos,
                    "user32/paint",
                    "InvalidateRgn",
                    &[("hWnd", &hWnd), ("hRgn", &hRgn), ("bErase", &bErase)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::InvalidateRgn(sys, hWnd, hRgn, bErase);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InvertRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
            let lpr = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::InvertRect_pos,
                    "user32/paint",
                    "InvertRect",
                    &[("hDC", &hDC), ("lpr", &lpr)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::InvertRect(sys, hDC, lpr);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsDlgButtonChecked(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::IsDlgButtonChecked_pos,
                    "user32/dialog",
                    "IsDlgButtonChecked",
                    &[("hDlg", &hDlg), ("nIDButton", &nIDButton)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::IsDlgButtonChecked(sys, hDlg, nIDButton);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsIconic(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::IsIconic_pos,
                    "user32/misc",
                    "IsIconic",
                    &[("hwnd", &hwnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::IsIconic(sys, hwnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsRectEmpty(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::IsRectEmpty_pos,
                    "user32/rect",
                    "IsRectEmpty",
                    &[("lprc", &lprc)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::IsRectEmpty(sys, lprc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::IsWindow_pos,
                    "user32/window",
                    "IsWindow",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::IsWindow(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IsWindowVisible(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::IsWindowVisible_pos,
                    "user32/window",
                    "IsWindowVisible",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::IsWindowVisible(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn KillTimer(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::timer::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let uIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/timer") {
                trace::Record::new(
                    user32::timer::KillTimer_pos,
                    "user32/timer",
                    "KillTimer",
                    &[("hWnd", &hWnd), ("uIDEvent", &uIDEvent)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::timer::KillTimer(sys, hWnd, uIDEvent);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadAcceleratorsW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpTableName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadAcceleratorsW_pos,
                    "user32/resource",
                    "LoadAcceleratorsW",
                    &[("hInstance", &hInstance), ("lpTableName", &lpTableName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadAcceleratorsW(sys, hInstance, lpTableName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadBitmapA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadBitmapA_pos,
                    "user32/resource",
                    "LoadBitmapA",
                    &[("hInstance", &hInstance), ("lpBitmapName", &lpBitmapName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadBitmapA(sys, hInstance, lpBitmapName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadCursorA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadCursorA_pos,
                    "user32/resource",
                    "LoadCursorA",
                    &[("hInstance", &hInstance), ("lpCursorName", &lpCursorName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadCursorA(sys, hInstance, lpCursorName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadCursorW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadCursorW_pos,
                    "user32/resource",
                    "LoadCursorW",
                    &[("hInstance", &hInstance), ("lpCursorName", &lpCursorName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadCursorW(sys, hInstance, lpCursorName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadIconA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadIconA_pos,
                    "user32/resource",
                    "LoadIconA",
                    &[("hInstance", &hInstance), ("lpIconName", &lpIconName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadIconA(sys, hInstance, lpIconName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadIconW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadIconW_pos,
                    "user32/resource",
                    "LoadIconW",
                    &[("hInstance", &hInstance), ("lpIconName", &lpIconName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadIconW(sys, hInstance, lpIconName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadImageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let name = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
            let typ = <Result<IMAGE, u32>>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let fuLoad = <Result<LR, u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadImageA_pos,
                    "user32/resource",
                    "LoadImageA",
                    &[
                        ("hInstance", &hInstance),
                        ("name", &name),
                        ("typ", &typ),
                        ("cx", &cx),
                        ("cy", &cy),
                        ("fuLoad", &fuLoad),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadImageA(sys, hInstance, name, typ, cx, cy, fuLoad);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadImageW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let name = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
            let typ = <Result<IMAGE, u32>>::from_stack(mem, stack_args + 8u32);
            let cx = <u32>::from_stack(mem, stack_args + 12u32);
            let cy = <u32>::from_stack(mem, stack_args + 16u32);
            let fuLoad = <Result<LR, u32>>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadImageW_pos,
                    "user32/resource",
                    "LoadImageW",
                    &[
                        ("hInstance", &hInstance),
                        ("name", &name),
                        ("typ", &typ),
                        ("cx", &cx),
                        ("cy", &cy),
                        ("fuLoad", &fuLoad),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadImageW(sys, hInstance, name, typ, cx, cy, fuLoad);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadMenuA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::LoadMenuA_pos,
                    "user32/menu",
                    "LoadMenuA",
                    &[("hInstance", &hInstance), ("lpMenuName", &lpMenuName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::LoadMenuA(sys, hInstance, lpMenuName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadMenuW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
            let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadMenuW_pos,
                    "user32/resource",
                    "LoadMenuW",
                    &[("hInstance", &hInstance), ("lpMenuName", &lpMenuName)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadMenuW(sys, hInstance, lpMenuName);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadStringA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let uID = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadStringA_pos,
                    "user32/resource",
                    "LoadStringA",
                    &[
                        ("hInstance", &hInstance),
                        ("uID", &uID),
                        ("lpBuffer", &lpBuffer),
                        ("cchBufferMax", &cchBufferMax),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadStringA(sys, hInstance, uID, lpBuffer, cchBufferMax);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn LoadStringW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
            let uID = <u32>::from_stack(mem, stack_args + 4u32);
            let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
            let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::LoadStringW_pos,
                    "user32/resource",
                    "LoadStringW",
                    &[
                        ("hInstance", &hInstance),
                        ("uID", &uID),
                        ("lpBuffer", &lpBuffer),
                        ("cchBufferMax", &cchBufferMax),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::LoadStringW(sys, hInstance, uID, lpBuffer, cchBufferMax);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MapVirtualKeyA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let uCode = <u32>::from_stack(mem, stack_args + 0u32);
            let uMapType = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::MapVirtualKeyA_pos,
                    "user32/keyboard",
                    "MapVirtualKeyA",
                    &[("uCode", &uCode), ("uMapType", &uMapType)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::MapVirtualKeyA(sys, uCode, uMapType);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MapWindowPoints(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, stack_args + 0u32);
            let hWndTo = <HWND>::from_stack(mem, stack_args + 4u32);
            let lpPoints = <Array<POINT>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::MapWindowPoints_pos,
                    "user32/window",
                    "MapWindowPoints",
                    &[
                        ("hWndFrom", &hWndFrom),
                        ("hWndTo", &hWndTo),
                        ("lpPoints", &lpPoints),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::MapWindowPoints(sys, hWndFrom, hWndTo, lpPoints);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MessageBoxA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpText = <Option<&CStr>>::from_stack(mem, stack_args + 4u32);
            let lpCaption = <Option<&CStr>>::from_stack(mem, stack_args + 8u32);
            let uType = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::MessageBoxA_pos,
                    "user32/dialog",
                    "MessageBoxA",
                    &[
                        ("hWnd", &hWnd),
                        ("lpText", &lpText),
                        ("lpCaption", &lpCaption),
                        ("uType", &uType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::MessageBoxA(sys, hWnd, lpText, lpCaption, uType);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MessageBoxW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpText = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let uType = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::MessageBoxW_pos,
                    "user32/dialog",
                    "MessageBoxW",
                    &[
                        ("hWnd", &hWnd),
                        ("lpText", &lpText),
                        ("lpCaption", &lpCaption),
                        ("uType", &uType),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::MessageBoxW(sys, hWnd, lpText, lpCaption, uType);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MoveWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let X = <u32>::from_stack(mem, stack_args + 4u32);
            let Y = <u32>::from_stack(mem, stack_args + 8u32);
            let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
            let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
            let bRepaint = <bool>::from_stack(mem, stack_args + 20u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::MoveWindow_pos,
                    "user32/window",
                    "MoveWindow",
                    &[
                        ("hWnd", &hWnd),
                        ("X", &X),
                        ("Y", &Y),
                        ("nWidth", &nWidth),
                        ("nHeight", &nHeight),
                        ("bRepaint", &bRepaint),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::MoveWindow(sys, hWnd, X, Y, nWidth, nHeight, bRepaint);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn MsgWaitForMultipleObjects(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let nCount = <u32>::from_stack(mem, stack_args + 0u32);
            let pHandles = <u32>::from_stack(mem, stack_args + 4u32);
            let fWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
            let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
            let dwWakeMask = <Result<QS, u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::MsgWaitForMultipleObjects_pos,
                    "user32/message",
                    "MsgWaitForMultipleObjects",
                    &[
                        ("nCount", &nCount),
                        ("pHandles", &pHandles),
                        ("fWaitAll", &fWaitAll),
                        ("dwMilliseconds", &dwMilliseconds),
                        ("dwWakeMask", &dwWakeMask),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::message::MsgWaitForMultipleObjects(
                    sys,
                    nCount,
                    pHandles,
                    fWaitAll,
                    dwMilliseconds,
                    dwWakeMask,
                )
                .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn OemToCharA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let pSrc = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let pDst = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::OemToCharA_pos,
                    "user32/misc",
                    "OemToCharA",
                    &[("pSrc", &pSrc), ("pDst", &pDst)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::OemToCharA(sys, pSrc, pDst);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PeekMessageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::PeekMessageA_pos,
                    "user32/message",
                    "PeekMessageA",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                        ("wRemoveMsg", &wRemoveMsg),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::PeekMessageA(
                sys,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PeekMessageW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
            let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
            let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::PeekMessageW_pos,
                    "user32/message",
                    "PeekMessageW",
                    &[
                        ("lpMsg", &lpMsg),
                        ("hWnd", &hWnd),
                        ("wMsgFilterMin", &wMsgFilterMin),
                        ("wMsgFilterMax", &wMsgFilterMax),
                        ("wRemoveMsg", &wRemoveMsg),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::PeekMessageW(
                sys,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PostMessageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <u32>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::PostMessageA_pos,
                    "user32/message",
                    "PostMessageA",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::PostMessageA(sys, hWnd, Msg, wParam, lParam);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PostMessageW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <u32>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::PostMessageW_pos,
                    "user32/message",
                    "PostMessageW",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::PostMessageW(sys, hWnd, Msg, wParam, lParam);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PostQuitMessage(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let nExitCode = <i32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::PostQuitMessage_pos,
                    "user32/message",
                    "PostQuitMessage",
                    &[("nExitCode", &nExitCode)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::PostQuitMessage(sys, nExitCode);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PostThreadMessageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let idThread = <u32>::from_stack(mem, stack_args + 0u32);
            let Msg = <u32>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::PostThreadMessageA_pos,
                    "user32/message",
                    "PostThreadMessageA",
                    &[
                        ("idThread", &idThread),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::PostThreadMessageA(sys, idThread, Msg, wParam, lParam);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn PtInRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
            let pt = <POINT>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::PtInRect_pos,
                    "user32/rect",
                    "PtInRect",
                    &[("lprc", &lprc), ("pt", &pt)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::PtInRect(sys, lprc, pt);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RedrawWindow(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lprcUpdate = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
            let hrgnUpdate = <HRGN>::from_stack(mem, stack_args + 8u32);
            let flags = <Result<RDW, u32>>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::RedrawWindow_pos,
                    "user32/window",
                    "RedrawWindow",
                    &[
                        ("hWnd", &hWnd),
                        ("lprcUpdate", &lprcUpdate),
                        ("hrgnUpdate", &hrgnUpdate),
                        ("flags", &flags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    user32::window::RedrawWindow(sys, hWnd, lprcUpdate, hrgnUpdate, flags).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn RegisterClassA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::RegisterClassA_pos,
                    "user32/wndclass",
                    "RegisterClassA",
                    &[("lpWndClass", &lpWndClass)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::RegisterClassA(sys, lpWndClass);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegisterClassExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::RegisterClassExA_pos,
                    "user32/wndclass",
                    "RegisterClassExA",
                    &[("lpWndClassEx", &lpWndClassEx)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::RegisterClassExA(sys, lpWndClassEx);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegisterClassExW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::RegisterClassExW_pos,
                    "user32/wndclass",
                    "RegisterClassExW",
                    &[("lpWndClassEx", &lpWndClassEx)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::RegisterClassExW(sys, lpWndClassEx);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegisterClassW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::RegisterClassW_pos,
                    "user32/wndclass",
                    "RegisterClassW",
                    &[("lpWndClass", &lpWndClass)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::RegisterClassW(sys, lpWndClass);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegisterClipboardFormatA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpszFormat = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::RegisterClipboardFormatA_pos,
                    "user32/misc",
                    "RegisterClipboardFormatA",
                    &[("lpszFormat", &lpszFormat)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::RegisterClipboardFormatA(sys, lpszFormat);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegisterWindowMessageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::RegisterWindowMessageA_pos,
                    "user32/window",
                    "RegisterWindowMessageA",
                    &[("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::RegisterWindowMessageA(sys, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn RegisterWindowMessageW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::RegisterWindowMessageW_pos,
                    "user32/window",
                    "RegisterWindowMessageW",
                    &[("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::RegisterWindowMessageW(sys, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReleaseCapture(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::ReleaseCapture_pos,
                    "user32/window",
                    "ReleaseCapture",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::ReleaseCapture(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ReleaseDC(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hdc = <HDC>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::ReleaseDC_pos,
                    "user32/window",
                    "ReleaseDC",
                    &[("hwnd", &hwnd), ("hdc", &hdc)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::ReleaseDC(sys, hwnd, hdc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SendDlgItemMessageA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let Msg = <u32>::from_stack(mem, stack_args + 8u32);
            let wParam = <u32>::from_stack(mem, stack_args + 12u32);
            let lParam = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::SendDlgItemMessageA_pos,
                    "user32/message",
                    "SendDlgItemMessageA",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                user32::message::SendDlgItemMessageA(sys, hDlg, nIDDlgItem, Msg, wParam, lParam);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SendMessageA(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::SendMessageA_pos,
                    "user32/message",
                    "SendMessageA",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::message::SendMessageA(sys, hWnd, Msg, wParam, lParam).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SendMessageW(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
            let wParam = <u32>::from_stack(mem, stack_args + 8u32);
            let lParam = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::SendMessageW_pos,
                    "user32/message",
                    "SendMessageW",
                    &[
                        ("hWnd", &hWnd),
                        ("Msg", &Msg),
                        ("wParam", &wParam),
                        ("lParam", &lParam),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::message::SendMessageW(sys, hWnd, Msg, wParam, lParam).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SetCapture(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::SetCapture_pos,
                    "user32/window",
                    "SetCapture",
                    &[("hwnd", &hwnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::SetCapture(sys, hwnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetClassLongA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIndex = <Result<GCL, i32>>::from_stack(mem, stack_args + 4u32);
            let dwNewLong = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::SetClassLongA_pos,
                    "user32/wndclass",
                    "SetClassLongA",
                    &[
                        ("hWnd", &hWnd),
                        ("nIndex", &nIndex),
                        ("dwNewLong", &dwNewLong),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::SetClassLongA(sys, hWnd, nIndex, dwNewLong);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetCursor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let hCursor = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::SetCursor_pos,
                    "user32/resource",
                    "SetCursor",
                    &[("hCursor", &hCursor)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::SetCursor(sys, hCursor);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetCursorPos(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let x = <i32>::from_stack(mem, stack_args + 0u32);
            let y = <i32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::SetCursorPos_pos,
                    "user32/misc",
                    "SetCursorPos",
                    &[("x", &x), ("y", &y)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::SetCursorPos(sys, x, y);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetDlgItemInt(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let uValue = <u32>::from_stack(mem, stack_args + 8u32);
            let _bSigned = <bool>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::SetDlgItemInt_pos,
                    "user32/dialog",
                    "SetDlgItemInt",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("uValue", &uValue),
                        ("bSigned", &_bSigned),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::SetDlgItemInt(sys, hDlg, nIDDlgItem, uValue, _bSigned);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetDlgItemTextA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::SetDlgItemTextA_pos,
                    "user32/dialog",
                    "SetDlgItemTextA",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpString", &lpString),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::SetDlgItemTextA(sys, hDlg, nIDDlgItem, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetDlgItemTextW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::dialog::*;
        unsafe {
            let mem = sys.mem().detach();
            let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/dialog") {
                trace::Record::new(
                    user32::dialog::SetDlgItemTextW_pos,
                    "user32/dialog",
                    "SetDlgItemTextW",
                    &[
                        ("hDlg", &hDlg),
                        ("nIDDlgItem", &nIDDlgItem),
                        ("lpString", &lpString),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::dialog::SetDlgItemTextW(sys, hDlg, nIDDlgItem, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetFocus(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::SetFocus_pos,
                    "user32/window",
                    "SetFocus",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::SetFocus(sys, hWnd).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SetForegroundWindow(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::SetForegroundWindow_pos,
                    "user32/window",
                    "SetForegroundWindow",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::SetForegroundWindow(sys, hWnd);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetMenu(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::SetMenu_pos,
                    "user32/menu",
                    "SetMenu",
                    &[("hWnd", &hWnd), ("hMenu", &hMenu)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::SetMenu(sys, hWnd, hMenu);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetMenuItemInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::menu::*;
        unsafe {
            let mem = sys.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
            let item = <u32>::from_stack(mem, stack_args + 4u32);
            let fByPosition = <bool>::from_stack(mem, stack_args + 8u32);
            let lpmii = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/menu") {
                trace::Record::new(
                    user32::menu::SetMenuItemInfoA_pos,
                    "user32/menu",
                    "SetMenuItemInfoA",
                    &[
                        ("hMenu", &hMenu),
                        ("item", &item),
                        ("fByPosition", &fByPosition),
                        ("lpmii", &lpmii),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::menu::SetMenuItemInfoA(sys, hMenu, item, fByPosition, lpmii);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let xLeft = <i32>::from_stack(mem, stack_args + 4u32);
            let yTop = <i32>::from_stack(mem, stack_args + 8u32);
            let xRight = <i32>::from_stack(mem, stack_args + 12u32);
            let yBottom = <i32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::SetRect_pos,
                    "user32/rect",
                    "SetRect",
                    &[
                        ("lprc", &lprc),
                        ("xLeft", &xLeft),
                        ("yTop", &yTop),
                        ("xRight", &xRight),
                        ("yBottom", &yBottom),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::SetRect(sys, lprc, xLeft, yTop, xRight, yBottom);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetRectEmpty(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::rect::*;
        unsafe {
            let mem = sys.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/rect") {
                trace::Record::new(
                    user32::rect::SetRectEmpty_pos,
                    "user32/rect",
                    "SetRectEmpty",
                    &[("lprc", &lprc)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::rect::SetRectEmpty(sys, lprc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetTimer(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::timer::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
            let uElapse = <u32>::from_stack(mem, stack_args + 8u32);
            let lpTimerFunc = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/timer") {
                trace::Record::new(
                    user32::timer::SetTimer_pos,
                    "user32/timer",
                    "SetTimer",
                    &[
                        ("hWnd", &hWnd),
                        ("nIDEvent", &nIDEvent),
                        ("uElapse", &uElapse),
                        ("lpTimerFunc", &lpTimerFunc),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::timer::SetTimer(sys, hWnd, nIDEvent, uElapse, lpTimerFunc);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetWindowLongA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nIndex = <Result<GWL, i32>>::from_stack(mem, stack_args + 4u32);
            let dwNewLong = <i32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::SetWindowLongA_pos,
                    "user32/window",
                    "SetWindowLongA",
                    &[
                        ("hWnd", &hWnd),
                        ("nIndex", &nIndex),
                        ("dwNewLong", &dwNewLong),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::SetWindowLongA(sys, hWnd, nIndex, dwNewLong);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetWindowPos(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hWndInsertAfter = <HWND>::from_stack(mem, stack_args + 4u32);
            let X = <i32>::from_stack(mem, stack_args + 8u32);
            let Y = <i32>::from_stack(mem, stack_args + 12u32);
            let cx = <i32>::from_stack(mem, stack_args + 16u32);
            let cy = <i32>::from_stack(mem, stack_args + 20u32);
            let uFlags = <Result<SWP, u32>>::from_stack(mem, stack_args + 24u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::SetWindowPos_pos,
                    "user32/window",
                    "SetWindowPos",
                    &[
                        ("hWnd", &hWnd),
                        ("hWndInsertAfter", &hWndInsertAfter),
                        ("X", &X),
                        ("Y", &Y),
                        ("cx", &cx),
                        ("cy", &cy),
                        ("uFlags", &uFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result =
                    user32::window::SetWindowPos(sys, hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags)
                        .await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SetWindowTextA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::SetWindowTextA_pos,
                    "user32/window",
                    "SetWindowTextA",
                    &[("hWnd", &hWnd), ("lpString", &lpString)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::window::SetWindowTextA(sys, hWnd, lpString);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn SetWindowsHookExA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let idHook = <u32>::from_stack(mem, stack_args + 0u32);
            let lpfn = <u32>::from_stack(mem, stack_args + 4u32);
            let hmod = <HINSTANCE>::from_stack(mem, stack_args + 8u32);
            let dwThreadId = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::SetWindowsHookExA_pos,
                    "user32/misc",
                    "SetWindowsHookExA",
                    &[
                        ("idHook", &idHook),
                        ("lpfn", &lpfn),
                        ("hmod", &hmod),
                        ("dwThreadId", &dwThreadId),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::SetWindowsHookExA(sys, idHook, lpfn, hmod, dwThreadId);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ShowCursor(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::resource::*;
        unsafe {
            let mem = sys.mem().detach();
            let bShow = <bool>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/resource") {
                trace::Record::new(
                    user32::resource::ShowCursor_pos,
                    "user32/resource",
                    "ShowCursor",
                    &[("bShow", &bShow)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::resource::ShowCursor(sys, bShow);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn ShowWindow(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::ShowWindow_pos,
                    "user32/window",
                    "ShowWindow",
                    &[("hWnd", &hWnd), ("nCmdShow", &nCmdShow)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::ShowWindow(sys, hWnd, nCmdShow).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn SystemParametersInfoA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let uiAction = <Result<SPI, u32>>::from_stack(mem, stack_args + 0u32);
            let uiParam = <u32>::from_stack(mem, stack_args + 4u32);
            let pvParam = <u32>::from_stack(mem, stack_args + 8u32);
            let fWinIni = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::SystemParametersInfoA_pos,
                    "user32/misc",
                    "SystemParametersInfoA",
                    &[
                        ("uiAction", &uiAction),
                        ("uiParam", &uiParam),
                        ("pvParam", &pvParam),
                        ("fWinIni", &fWinIni),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                user32::misc::SystemParametersInfoA(sys, uiAction, uiParam, pvParam, fWinIni);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TrackMouseEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpEventTrack = <Option<&mut TRACKMOUSEEVENT>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::TrackMouseEvent_pos,
                    "user32/misc",
                    "TrackMouseEvent",
                    &[("lpEventTrack", &lpEventTrack)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::TrackMouseEvent(sys, lpEventTrack);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TranslateAcceleratorW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let hAccTable = <u32>::from_stack(mem, stack_args + 4u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::TranslateAcceleratorW_pos,
                    "user32/message",
                    "TranslateAcceleratorW",
                    &[
                        ("hWnd", &hWnd),
                        ("hAccTable", &hAccTable),
                        ("lpMsg", &lpMsg),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::TranslateAcceleratorW(sys, hWnd, hAccTable, lpMsg);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn TranslateMessage(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::TranslateMessage_pos,
                    "user32/message",
                    "TranslateMessage",
                    &[("lpMsg", &lpMsg)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::message::TranslateMessage(sys, lpMsg);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn UnregisterClassA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::wndclass::*;
        unsafe {
            let mem = sys.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/wndclass") {
                trace::Record::new(
                    user32::wndclass::UnregisterClassA_pos,
                    "user32/wndclass",
                    "UnregisterClassA",
                    &[("lpClassName", &lpClassName), ("hInstance", &hInstance)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::wndclass::UnregisterClassA(sys, lpClassName, hInstance);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn UpdateWindow(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::window::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("user32/window") {
                trace::Record::new(
                    user32::window::UpdateWindow_pos,
                    "user32/window",
                    "UpdateWindow",
                    &[("hWnd", &hWnd)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::window::UpdateWindow(sys, hWnd).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn ValidateRect(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::paint::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("user32/paint") {
                trace::Record::new(
                    user32::paint::ValidateRect_pos,
                    "user32/paint",
                    "ValidateRect",
                    &[("hWnd", &hWnd), ("lpRect", &lpRect)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::paint::ValidateRect(sys, hWnd, lpRect);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn WaitMessage(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use user32::message::*;
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("user32/message") {
                trace::Record::new(
                    user32::message::WaitMessage_pos,
                    "user32/message",
                    "WaitMessage",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = user32::message::WaitMessage(sys).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
    }
    pub unsafe fn WinHelpW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let hWndMain = <HWND>::from_stack(mem, stack_args + 0u32);
            let lpszHelp = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let uCommand = <u32>::from_stack(mem, stack_args + 8u32);
            let dwData = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::WinHelpW_pos,
                    "user32/misc",
                    "WinHelpW",
                    &[
                        ("hWndMain", &hWndMain),
                        ("lpszHelp", &lpszHelp),
                        ("uCommand", &uCommand),
                        ("dwData", &dwData),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::WinHelpW(sys, hWndMain, lpszHelp, uCommand, dwData);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn keybd_event(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::keyboard::*;
        unsafe {
            let mem = sys.mem().detach();
            let bVk = <u8>::from_stack(mem, stack_args + 0u32);
            let bScan = <u8>::from_stack(mem, stack_args + 4u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
            let dwExtraInfo = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("user32/keyboard") {
                trace::Record::new(
                    user32::keyboard::keybd_event_pos,
                    "user32/keyboard",
                    "keybd_event",
                    &[
                        ("bVk", &bVk),
                        ("bScan", &bScan),
                        ("dwFlags", &dwFlags),
                        ("dwExtraInfo", &dwExtraInfo),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = user32::keyboard::keybd_event(sys, bVk, bScan, dwFlags, dwExtraInfo);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn wsprintfA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&str>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::wsprintfA_pos,
                    "user32/misc",
                    "wsprintfA",
                    &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::wsprintfA(sys, buf, fmt, args);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn wsprintfW(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        use user32::misc::*;
        unsafe {
            let mem = sys.mem().detach();
            let buf = <u32>::from_stack(mem, stack_args + 0u32);
            let fmt = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
            let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("user32/misc") {
                trace::Record::new(
                    user32::misc::wsprintfW_pos,
                    "user32/misc",
                    "wsprintfW",
                    &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
                )
                .enter()
            } else {
                None
            };
            let result = user32::misc::wsprintfW(sys, buf, fmt, args);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 152usize] = [
    Shim {
        name: "AdjustWindowRect",
        func: Handler::Sync(wrappers::AdjustWindowRect),
    },
    Shim {
        name: "AdjustWindowRectEx",
        func: Handler::Sync(wrappers::AdjustWindowRectEx),
    },
    Shim {
        name: "AppendMenuA",
        func: Handler::Sync(wrappers::AppendMenuA),
    },
    Shim {
        name: "BeginPaint",
        func: Handler::Sync(wrappers::BeginPaint),
    },
    Shim {
        name: "CallWindowProcA",
        func: Handler::Sync(wrappers::CallWindowProcA),
    },
    Shim {
        name: "CharLowerA",
        func: Handler::Sync(wrappers::CharLowerA),
    },
    Shim {
        name: "CharLowerBuffA",
        func: Handler::Sync(wrappers::CharLowerBuffA),
    },
    Shim {
        name: "CheckDlgButton",
        func: Handler::Sync(wrappers::CheckDlgButton),
    },
    Shim {
        name: "CheckMenuItem",
        func: Handler::Sync(wrappers::CheckMenuItem),
    },
    Shim {
        name: "CheckRadioButton",
        func: Handler::Sync(wrappers::CheckRadioButton),
    },
    Shim {
        name: "ClientToScreen",
        func: Handler::Sync(wrappers::ClientToScreen),
    },
    Shim {
        name: "CopyRect",
        func: Handler::Sync(wrappers::CopyRect),
    },
    Shim {
        name: "CreateCursor",
        func: Handler::Sync(wrappers::CreateCursor),
    },
    Shim {
        name: "CreatePopupMenu",
        func: Handler::Sync(wrappers::CreatePopupMenu),
    },
    Shim {
        name: "CreateWindowExA",
        func: Handler::Async(wrappers::CreateWindowExA),
    },
    Shim {
        name: "CreateWindowExW",
        func: Handler::Async(wrappers::CreateWindowExW),
    },
    Shim {
        name: "DefWindowProcA",
        func: Handler::Async(wrappers::DefWindowProcA),
    },
    Shim {
        name: "DefWindowProcW",
        func: Handler::Async(wrappers::DefWindowProcW),
    },
    Shim {
        name: "DeleteMenu",
        func: Handler::Sync(wrappers::DeleteMenu),
    },
    Shim {
        name: "DestroyWindow",
        func: Handler::Sync(wrappers::DestroyWindow),
    },
    Shim {
        name: "DialogBoxIndirectParamA",
        func: Handler::Sync(wrappers::DialogBoxIndirectParamA),
    },
    Shim {
        name: "DialogBoxParamA",
        func: Handler::Sync(wrappers::DialogBoxParamA),
    },
    Shim {
        name: "DialogBoxParamW",
        func: Handler::Sync(wrappers::DialogBoxParamW),
    },
    Shim {
        name: "DispatchMessageA",
        func: Handler::Async(wrappers::DispatchMessageA),
    },
    Shim {
        name: "DispatchMessageW",
        func: Handler::Async(wrappers::DispatchMessageW),
    },
    Shim {
        name: "DrawMenuBar",
        func: Handler::Sync(wrappers::DrawMenuBar),
    },
    Shim {
        name: "DrawTextW",
        func: Handler::Sync(wrappers::DrawTextW),
    },
    Shim {
        name: "EnableMenuItem",
        func: Handler::Sync(wrappers::EnableMenuItem),
    },
    Shim {
        name: "EnableWindow",
        func: Handler::Sync(wrappers::EnableWindow),
    },
    Shim {
        name: "EndDialog",
        func: Handler::Sync(wrappers::EndDialog),
    },
    Shim {
        name: "EndPaint",
        func: Handler::Sync(wrappers::EndPaint),
    },
    Shim {
        name: "FillRect",
        func: Handler::Sync(wrappers::FillRect),
    },
    Shim {
        name: "FindWindowA",
        func: Handler::Sync(wrappers::FindWindowA),
    },
    Shim {
        name: "FrameRect",
        func: Handler::Sync(wrappers::FrameRect),
    },
    Shim {
        name: "GetActiveWindow",
        func: Handler::Sync(wrappers::GetActiveWindow),
    },
    Shim {
        name: "GetCapture",
        func: Handler::Sync(wrappers::GetCapture),
    },
    Shim {
        name: "GetClassLongA",
        func: Handler::Sync(wrappers::GetClassLongA),
    },
    Shim {
        name: "GetClientRect",
        func: Handler::Sync(wrappers::GetClientRect),
    },
    Shim {
        name: "GetCursorPos",
        func: Handler::Sync(wrappers::GetCursorPos),
    },
    Shim {
        name: "GetDC",
        func: Handler::Sync(wrappers::GetDC),
    },
    Shim {
        name: "GetDesktopWindow",
        func: Handler::Sync(wrappers::GetDesktopWindow),
    },
    Shim {
        name: "GetDlgItem",
        func: Handler::Sync(wrappers::GetDlgItem),
    },
    Shim {
        name: "GetDlgItemInt",
        func: Handler::Sync(wrappers::GetDlgItemInt),
    },
    Shim {
        name: "GetDlgItemTextW",
        func: Handler::Sync(wrappers::GetDlgItemTextW),
    },
    Shim {
        name: "GetFocus",
        func: Handler::Sync(wrappers::GetFocus),
    },
    Shim {
        name: "GetForegroundWindow",
        func: Handler::Sync(wrappers::GetForegroundWindow),
    },
    Shim {
        name: "GetKeyNameTextA",
        func: Handler::Sync(wrappers::GetKeyNameTextA),
    },
    Shim {
        name: "GetKeyState",
        func: Handler::Sync(wrappers::GetKeyState),
    },
    Shim {
        name: "GetKeyboardLayout",
        func: Handler::Sync(wrappers::GetKeyboardLayout),
    },
    Shim {
        name: "GetKeyboardLayoutList",
        func: Handler::Sync(wrappers::GetKeyboardLayoutList),
    },
    Shim {
        name: "GetKeyboardState",
        func: Handler::Sync(wrappers::GetKeyboardState),
    },
    Shim {
        name: "GetKeyboardType",
        func: Handler::Sync(wrappers::GetKeyboardType),
    },
    Shim {
        name: "GetLastActivePopup",
        func: Handler::Sync(wrappers::GetLastActivePopup),
    },
    Shim {
        name: "GetMenu",
        func: Handler::Sync(wrappers::GetMenu),
    },
    Shim {
        name: "GetMenuItemRect",
        func: Handler::Sync(wrappers::GetMenuItemRect),
    },
    Shim {
        name: "GetMessageA",
        func: Handler::Async(wrappers::GetMessageA),
    },
    Shim {
        name: "GetMessageW",
        func: Handler::Async(wrappers::GetMessageW),
    },
    Shim {
        name: "GetMonitorInfoA",
        func: Handler::Sync(wrappers::GetMonitorInfoA),
    },
    Shim {
        name: "GetProcessWindowStation",
        func: Handler::Sync(wrappers::GetProcessWindowStation),
    },
    Shim {
        name: "GetQueueStatus",
        func: Handler::Sync(wrappers::GetQueueStatus),
    },
    Shim {
        name: "GetSubMenu",
        func: Handler::Sync(wrappers::GetSubMenu),
    },
    Shim {
        name: "GetSysColor",
        func: Handler::Sync(wrappers::GetSysColor),
    },
    Shim {
        name: "GetSystemMenu",
        func: Handler::Sync(wrappers::GetSystemMenu),
    },
    Shim {
        name: "GetSystemMetrics",
        func: Handler::Sync(wrappers::GetSystemMetrics),
    },
    Shim {
        name: "GetUpdateRect",
        func: Handler::Sync(wrappers::GetUpdateRect),
    },
    Shim {
        name: "GetUserObjectInformationW",
        func: Handler::Sync(wrappers::GetUserObjectInformationW),
    },
    Shim {
        name: "GetWindowDC",
        func: Handler::Sync(wrappers::GetWindowDC),
    },
    Shim {
        name: "GetWindowLongA",
        func: Handler::Sync(wrappers::GetWindowLongA),
    },
    Shim {
        name: "GetWindowPlacement",
        func: Handler::Sync(wrappers::GetWindowPlacement),
    },
    Shim {
        name: "GetWindowRect",
        func: Handler::Sync(wrappers::GetWindowRect),
    },
    Shim {
        name: "InflateRect",
        func: Handler::Sync(wrappers::InflateRect),
    },
    Shim {
        name: "IntersectRect",
        func: Handler::Sync(wrappers::IntersectRect),
    },
    Shim {
        name: "InvalidateRect",
        func: Handler::Sync(wrappers::InvalidateRect),
    },
    Shim {
        name: "InvalidateRgn",
        func: Handler::Sync(wrappers::InvalidateRgn),
    },
    Shim {
        name: "InvertRect",
        func: Handler::Sync(wrappers::InvertRect),
    },
    Shim {
        name: "IsDlgButtonChecked",
        func: Handler::Sync(wrappers::IsDlgButtonChecked),
    },
    Shim {
        name: "IsIconic",
        func: Handler::Sync(wrappers::IsIconic),
    },
    Shim {
        name: "IsRectEmpty",
        func: Handler::Sync(wrappers::IsRectEmpty),
    },
    Shim {
        name: "IsWindow",
        func: Handler::Sync(wrappers::IsWindow),
    },
    Shim {
        name: "IsWindowVisible",
        func: Handler::Sync(wrappers::IsWindowVisible),
    },
    Shim {
        name: "KillTimer",
        func: Handler::Sync(wrappers::KillTimer),
    },
    Shim {
        name: "LoadAcceleratorsW",
        func: Handler::Sync(wrappers::LoadAcceleratorsW),
    },
    Shim {
        name: "LoadBitmapA",
        func: Handler::Sync(wrappers::LoadBitmapA),
    },
    Shim {
        name: "LoadCursorA",
        func: Handler::Sync(wrappers::LoadCursorA),
    },
    Shim {
        name: "LoadCursorW",
        func: Handler::Sync(wrappers::LoadCursorW),
    },
    Shim {
        name: "LoadIconA",
        func: Handler::Sync(wrappers::LoadIconA),
    },
    Shim {
        name: "LoadIconW",
        func: Handler::Sync(wrappers::LoadIconW),
    },
    Shim {
        name: "LoadImageA",
        func: Handler::Sync(wrappers::LoadImageA),
    },
    Shim {
        name: "LoadImageW",
        func: Handler::Sync(wrappers::LoadImageW),
    },
    Shim {
        name: "LoadMenuA",
        func: Handler::Sync(wrappers::LoadMenuA),
    },
    Shim {
        name: "LoadMenuW",
        func: Handler::Sync(wrappers::LoadMenuW),
    },
    Shim {
        name: "LoadStringA",
        func: Handler::Sync(wrappers::LoadStringA),
    },
    Shim {
        name: "LoadStringW",
        func: Handler::Sync(wrappers::LoadStringW),
    },
    Shim {
        name: "MapVirtualKeyA",
        func: Handler::Sync(wrappers::MapVirtualKeyA),
    },
    Shim {
        name: "MapWindowPoints",
        func: Handler::Sync(wrappers::MapWindowPoints),
    },
    Shim {
        name: "MessageBoxA",
        func: Handler::Sync(wrappers::MessageBoxA),
    },
    Shim {
        name: "MessageBoxW",
        func: Handler::Sync(wrappers::MessageBoxW),
    },
    Shim {
        name: "MoveWindow",
        func: Handler::Sync(wrappers::MoveWindow),
    },
    Shim {
        name: "MsgWaitForMultipleObjects",
        func: Handler::Async(wrappers::MsgWaitForMultipleObjects),
    },
    Shim {
        name: "OemToCharA",
        func: Handler::Sync(wrappers::OemToCharA),
    },
    Shim {
        name: "PeekMessageA",
        func: Handler::Sync(wrappers::PeekMessageA),
    },
    Shim {
        name: "PeekMessageW",
        func: Handler::Sync(wrappers::PeekMessageW),
    },
    Shim {
        name: "PostMessageA",
        func: Handler::Sync(wrappers::PostMessageA),
    },
    Shim {
        name: "PostMessageW",
        func: Handler::Sync(wrappers::PostMessageW),
    },
    Shim {
        name: "PostQuitMessage",
        func: Handler::Sync(wrappers::PostQuitMessage),
    },
    Shim {
        name: "PostThreadMessageA",
        func: Handler::Sync(wrappers::PostThreadMessageA),
    },
    Shim {
        name: "PtInRect",
        func: Handler::Sync(wrappers::PtInRect),
    },
    Shim {
        name: "RedrawWindow",
        func: Handler::Async(wrappers::RedrawWindow),
    },
    Shim {
        name: "RegisterClassA",
        func: Handler::Sync(wrappers::RegisterClassA),
    },
    Shim {
        name: "RegisterClassExA",
        func: Handler::Sync(wrappers::RegisterClassExA),
    },
    Shim {
        name: "RegisterClassExW",
        func: Handler::Sync(wrappers::RegisterClassExW),
    },
    Shim {
        name: "RegisterClassW",
        func: Handler::Sync(wrappers::RegisterClassW),
    },
    Shim {
        name: "RegisterClipboardFormatA",
        func: Handler::Sync(wrappers::RegisterClipboardFormatA),
    },
    Shim {
        name: "RegisterWindowMessageA",
        func: Handler::Sync(wrappers::RegisterWindowMessageA),
    },
    Shim {
        name: "RegisterWindowMessageW",
        func: Handler::Sync(wrappers::RegisterWindowMessageW),
    },
    Shim {
        name: "ReleaseCapture",
        func: Handler::Sync(wrappers::ReleaseCapture),
    },
    Shim {
        name: "ReleaseDC",
        func: Handler::Sync(wrappers::ReleaseDC),
    },
    Shim {
        name: "SendDlgItemMessageA",
        func: Handler::Sync(wrappers::SendDlgItemMessageA),
    },
    Shim {
        name: "SendMessageA",
        func: Handler::Async(wrappers::SendMessageA),
    },
    Shim {
        name: "SendMessageW",
        func: Handler::Async(wrappers::SendMessageW),
    },
    Shim {
        name: "SetCapture",
        func: Handler::Sync(wrappers::SetCapture),
    },
    Shim {
        name: "SetClassLongA",
        func: Handler::Sync(wrappers::SetClassLongA),
    },
    Shim {
        name: "SetCursor",
        func: Handler::Sync(wrappers::SetCursor),
    },
    Shim {
        name: "SetCursorPos",
        func: Handler::Sync(wrappers::SetCursorPos),
    },
    Shim {
        name: "SetDlgItemInt",
        func: Handler::Sync(wrappers::SetDlgItemInt),
    },
    Shim {
        name: "SetDlgItemTextA",
        func: Handler::Sync(wrappers::SetDlgItemTextA),
    },
    Shim {
        name: "SetDlgItemTextW",
        func: Handler::Sync(wrappers::SetDlgItemTextW),
    },
    Shim {
        name: "SetFocus",
        func: Handler::Async(wrappers::SetFocus),
    },
    Shim {
        name: "SetForegroundWindow",
        func: Handler::Sync(wrappers::SetForegroundWindow),
    },
    Shim {
        name: "SetMenu",
        func: Handler::Sync(wrappers::SetMenu),
    },
    Shim {
        name: "SetMenuItemInfoA",
        func: Handler::Sync(wrappers::SetMenuItemInfoA),
    },
    Shim {
        name: "SetRect",
        func: Handler::Sync(wrappers::SetRect),
    },
    Shim {
        name: "SetRectEmpty",
        func: Handler::Sync(wrappers::SetRectEmpty),
    },
    Shim {
        name: "SetTimer",
        func: Handler::Sync(wrappers::SetTimer),
    },
    Shim {
        name: "SetWindowLongA",
        func: Handler::Sync(wrappers::SetWindowLongA),
    },
    Shim {
        name: "SetWindowPos",
        func: Handler::Async(wrappers::SetWindowPos),
    },
    Shim {
        name: "SetWindowTextA",
        func: Handler::Sync(wrappers::SetWindowTextA),
    },
    Shim {
        name: "SetWindowsHookExA",
        func: Handler::Sync(wrappers::SetWindowsHookExA),
    },
    Shim {
        name: "ShowCursor",
        func: Handler::Sync(wrappers::ShowCursor),
    },
    Shim {
        name: "ShowWindow",
        func: Handler::Async(wrappers::ShowWindow),
    },
    Shim {
        name: "SystemParametersInfoA",
        func: Handler::Sync(wrappers::SystemParametersInfoA),
    },
    Shim {
        name: "TrackMouseEvent",
        func: Handler::Sync(wrappers::TrackMouseEvent),
    },
    Shim {
        name: "TranslateAcceleratorW",
        func: Handler::Sync(wrappers::TranslateAcceleratorW),
    },
    Shim {
        name: "TranslateMessage",
        func: Handler::Sync(wrappers::TranslateMessage),
    },
    Shim {
        name: "UnregisterClassA",
        func: Handler::Sync(wrappers::UnregisterClassA),
    },
    Shim {
        name: "UpdateWindow",
        func: Handler::Async(wrappers::UpdateWindow),
    },
    Shim {
        name: "ValidateRect",
        func: Handler::Sync(wrappers::ValidateRect),
    },
    Shim {
        name: "WaitMessage",
        func: Handler::Async(wrappers::WaitMessage),
    },
    Shim {
        name: "WinHelpW",
        func: Handler::Sync(wrappers::WinHelpW),
    },
    Shim {
        name: "keybd_event",
        func: Handler::Sync(wrappers::keybd_event),
    },
    Shim {
        name: "wsprintfA",
        func: Handler::Sync(wrappers::wsprintfA),
    },
    Shim {
        name: "wsprintfW",
        func: Handler::Sync(wrappers::wsprintfW),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "user32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../user32.dll"),
};
