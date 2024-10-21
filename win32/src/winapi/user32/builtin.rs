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
    use winapi::user32::*;
    pub unsafe fn AdjustWindowRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let dwStyle = <Result<WS, u32>>::from_stack(mem, stack_args + 4u32);
        let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::AdjustWindowRect_pos,
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
        let result = winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let dwStyle = <Result<WS, u32>>::from_stack(mem, stack_args + 4u32);
        let bMenu = <bool>::from_stack(mem, stack_args + 8u32);
        let dwExStyle = <Result<WS_EX, u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::AdjustWindowRectEx_pos,
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
        let result = winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn AppendMenuA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
        let uFlags = <u32>::from_stack(mem, stack_args + 4u32);
        let uIDNewItem = <u32>::from_stack(mem, stack_args + 8u32);
        let lpNewItem = <Option<&str>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::AppendMenuA_pos,
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
        let result = winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn BeginPaint(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::BeginPaint_pos,
                "user32/paint",
                "BeginPaint",
                &[("hWnd", &hWnd), ("lpPaint", &lpPaint)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::BeginPaint(machine, hWnd, lpPaint);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CallWindowProcA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPrevWndFunc = <u32>::from_stack(mem, stack_args + 0u32);
        let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
        let Msg = <u32>::from_stack(mem, stack_args + 8u32);
        let wParam = <u32>::from_stack(mem, stack_args + 12u32);
        let lParam = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::CallWindowProcA_pos,
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
            winapi::user32::CallWindowProcA(machine, lpPrevWndFunc, hWnd, Msg, wParam, lParam);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CheckDlgButton(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
        let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::CheckDlgButton_pos,
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
        let result = winapi::user32::CheckDlgButton(machine, hDlg, nIDButton, uCheck);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CheckMenuItem(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
        let uIDCheckItem = <u32>::from_stack(mem, stack_args + 4u32);
        let uCheck = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::CheckMenuItem_pos,
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
        let result = winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CheckRadioButton(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDFirstButton = <i32>::from_stack(mem, stack_args + 4u32);
        let nIDLastButton = <i32>::from_stack(mem, stack_args + 8u32);
        let nIDCheckButton = <i32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::CheckRadioButton_pos,
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
        let result = winapi::user32::CheckRadioButton(
            machine,
            hDlg,
            nIDFirstButton,
            nIDLastButton,
            nIDCheckButton,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn ClientToScreen(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::ClientToScreen_pos,
                "user32/window",
                "ClientToScreen",
                &[("hWnd", &hWnd), ("lpPoint", &lpPoint)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::ClientToScreen(machine, hWnd, lpPoint);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CopyRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let lprcSrc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::CopyRect_pos,
                "user32/rect",
                "CopyRect",
                &[("lprcDst", &lprcDst), ("lprcSrc", &lprcSrc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::CopyRect(machine, lprcDst, lprcSrc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CreateCursor(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInst = <u32>::from_stack(mem, stack_args + 0u32);
        let xHotSpot = <u32>::from_stack(mem, stack_args + 4u32);
        let yHotSpot = <u32>::from_stack(mem, stack_args + 8u32);
        let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
        let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
        let pvANDPlane = <u32>::from_stack(mem, stack_args + 20u32);
        let pvXORPlane = <u32>::from_stack(mem, stack_args + 24u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::CreateCursor_pos,
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
        let result = winapi::user32::CreateCursor(
            machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CreatePopupMenu(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::CreatePopupMenu_pos,
                "user32/menu",
                "CreatePopupMenu",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::CreatePopupMenu(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn CreateWindowExA(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let dwExStyle = <Result<WS_EX, u32>>::from_stack(mem, stack_args + 0u32);
        let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, stack_args + 4u32);
        let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 8u32);
        let dwStyle = <Result<WS, u32>>::from_stack(mem, stack_args + 12u32);
        let X = <u32>::from_stack(mem, stack_args + 16u32);
        let Y = <u32>::from_stack(mem, stack_args + 20u32);
        let nWidth = <u32>::from_stack(mem, stack_args + 24u32);
        let nHeight = <u32>::from_stack(mem, stack_args + 28u32);
        let hWndParent = <HWND>::from_stack(mem, stack_args + 32u32);
        let hMenu = <u32>::from_stack(mem, stack_args + 36u32);
        let hInstance = <u32>::from_stack(mem, stack_args + 40u32);
        let lpParam = <u32>::from_stack(mem, stack_args + 44u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::CreateWindowExA_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
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
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn CreateWindowExW(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let dwExStyle = <Result<WS_EX, u32>>::from_stack(mem, stack_args + 0u32);
        let lpClassName = <CreateWindowClassName<'_, Str16>>::from_stack(mem, stack_args + 4u32);
        let lpWindowName = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
        let dwStyle = <Result<WS, u32>>::from_stack(mem, stack_args + 12u32);
        let X = <u32>::from_stack(mem, stack_args + 16u32);
        let Y = <u32>::from_stack(mem, stack_args + 20u32);
        let nWidth = <u32>::from_stack(mem, stack_args + 24u32);
        let nHeight = <u32>::from_stack(mem, stack_args + 28u32);
        let hWndParent = <HWND>::from_stack(mem, stack_args + 32u32);
        let hMenu = <u32>::from_stack(mem, stack_args + 36u32);
        let hInstance = <u32>::from_stack(mem, stack_args + 40u32);
        let lpParam = <u32>::from_stack(mem, stack_args + 44u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::CreateWindowExW_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
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
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn DefWindowProcA(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::DefWindowProcA_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn DefWindowProcW(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::DefWindowProcW_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn DeleteMenu(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
        let uPosition = <u32>::from_stack(mem, stack_args + 4u32);
        let uFlags = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::DeleteMenu_pos,
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
        let result = winapi::user32::DeleteMenu(machine, hMenu, uPosition, uFlags);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DestroyWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::DestroyWindow_pos,
                "user32/window",
                "DestroyWindow",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::DestroyWindow(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let hDialogTemplate = <u32>::from_stack(mem, stack_args + 4u32);
        let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
        let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
        let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::DialogBoxIndirectParamA_pos,
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
        let result = winapi::user32::DialogBoxIndirectParamA(
            machine,
            hInstance,
            hDialogTemplate,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DialogBoxParamA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
        let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
        let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
        let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::DialogBoxParamA_pos,
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
        let result = winapi::user32::DialogBoxParamA(
            machine,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DialogBoxParamW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpTemplateName = <u32>::from_stack(mem, stack_args + 4u32);
        let hWndParent = <HWND>::from_stack(mem, stack_args + 8u32);
        let lpDialogFunc = <u32>::from_stack(mem, stack_args + 12u32);
        let dwInitParam = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::DialogBoxParamW_pos,
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
        let result = winapi::user32::DialogBoxParamW(
            machine,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DispatchMessageA(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::DispatchMessageA_pos,
                "user32/message",
                "DispatchMessageA",
                &[("lpMsg", &lpMsg)],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn DispatchMessageW(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::DispatchMessageW_pos,
                "user32/message",
                "DispatchMessageW",
                &[("lpMsg", &lpMsg)],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::DispatchMessageW(machine, lpMsg).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn DrawMenuBar(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::DrawMenuBar_pos,
                "user32/menu",
                "DrawMenuBar",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::DrawMenuBar(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn DrawTextW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
        let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let nCount = <i32>::from_stack(mem, stack_args + 8u32);
        let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 12u32);
        let uFormat = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::DrawTextW_pos,
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
        let result = winapi::user32::DrawTextW(machine, hDC, lpString, nCount, lpRect, uFormat);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn EnableMenuItem(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
        let uIDEnableItem = <u32>::from_stack(mem, stack_args + 4u32);
        let uEnable = <Result<MF, u32>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::EnableMenuItem_pos,
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
        let result = winapi::user32::EnableMenuItem(machine, hMenu, uIDEnableItem, uEnable);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn EnableWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let bEnable = <bool>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::EnableWindow_pos,
                "user32/window",
                "EnableWindow",
                &[("hWnd", &hWnd), ("bEnable", &bEnable)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::EnableWindow(machine, hWnd, bEnable);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn EndDialog(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nResult = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::EndDialog_pos,
                "user32/dialog",
                "EndDialog",
                &[("hDlg", &hDlg), ("nResult", &nResult)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::EndDialog(machine, hDlg, nResult);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn EndPaint(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::EndPaint_pos,
                "user32/paint",
                "EndPaint",
                &[("hWnd", &hWnd), ("lpPaint", &lpPaint)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::EndPaint(machine, hWnd, lpPaint);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn FillRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
        let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let hbr = <BrushOrColor>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::FillRect_pos,
                "user32/paint",
                "FillRect",
                &[("hDC", &hDC), ("lprc", &lprc), ("hbr", &hbr)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::FillRect(machine, hDC, lprc, hbr);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn FindWindowA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpClassName = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpWindowName = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::FindWindowA_pos,
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
        let result = winapi::user32::FindWindowA(machine, lpClassName, lpWindowName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn FrameRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
        let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let hbr = <HBRUSH>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::FrameRect_pos,
                "user32/paint",
                "FrameRect",
                &[("hDC", &hDC), ("lprc", &lprc), ("hbr", &hbr)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::FrameRect(machine, hDC, lprc, hbr);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetActiveWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetActiveWindow_pos,
                "user32/window",
                "GetActiveWindow",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetActiveWindow(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetCapture(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetCapture_pos,
                "user32/window",
                "GetCapture",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetCapture(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetClassLongA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIndex = <Result<GCL, u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/wndclass") {
            crate::trace::Record::new(
                winapi::user32::GetClassLongA_pos,
                "user32/wndclass",
                "GetClassLongA",
                &[("hWnd", &hWnd), ("nIndex", &nIndex)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetClassLongA(machine, hWnd, nIndex);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetClientRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetClientRect_pos,
                "user32/window",
                "GetClientRect",
                &[("hWnd", &hWnd), ("lpRect", &lpRect)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetClientRect(machine, hWnd, lpRect);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetCursorPos(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpPoint = <Option<&mut POINT>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetCursorPos_pos,
                "user32/misc",
                "GetCursorPos",
                &[("lpPoint", &lpPoint)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetCursorPos(machine, lpPoint);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetDC(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetDC_pos,
                "user32/window",
                "GetDC",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetDC(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetDesktopWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetDesktopWindow_pos,
                "user32/window",
                "GetDesktopWindow",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetDesktopWindow(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetDlgItem(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::GetDlgItem_pos,
                "user32/dialog",
                "GetDlgItem",
                &[("hDlg", &hDlg), ("nIDDlgItem", &nIDDlgItem)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetDlgItem(machine, hDlg, nIDDlgItem);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetDlgItemInt(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let lpTranslated = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let bSigned = <bool>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::GetDlgItemInt_pos,
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
            winapi::user32::GetDlgItemInt(machine, hDlg, nIDDlgItem, lpTranslated, bSigned);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetDlgItemTextW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let lpString = <ArrayWithSizeMut<u16>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::GetDlgItemTextW_pos,
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
        let result = winapi::user32::GetDlgItemTextW(machine, hDlg, nIDDlgItem, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetFocus(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetFocus_pos,
                "user32/window",
                "GetFocus",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetFocus(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetForegroundWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetForegroundWindow_pos,
                "user32/window",
                "GetForegroundWindow",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetForegroundWindow(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetKeyState(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nVirtKey = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetKeyState_pos,
                "user32/misc",
                "GetKeyState",
                &[("nVirtKey", &nVirtKey)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetKeyState(machine, nVirtKey);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetKeyboardLayout(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let idThread = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetKeyboardLayout_pos,
                "user32/misc",
                "GetKeyboardLayout",
                &[("idThread", &idThread)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetKeyboardLayout(machine, idThread);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetKeyboardState(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpKeyState = <Option<&mut u8>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetKeyboardState_pos,
                "user32/misc",
                "GetKeyboardState",
                &[("lpKeyState", &lpKeyState)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetKeyboardState(machine, lpKeyState);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetKeyboardType(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nTypeFlag = <i32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetKeyboardType_pos,
                "user32/misc",
                "GetKeyboardType",
                &[("nTypeFlag", &nTypeFlag)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetKeyboardType(machine, nTypeFlag);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetLastActivePopup(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetLastActivePopup_pos,
                "user32/window",
                "GetLastActivePopup",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetLastActivePopup(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetMenu(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::GetMenu_pos,
                "user32/menu",
                "GetMenu",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetMenu(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetMenuItemRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
        let uItem = <u32>::from_stack(mem, stack_args + 8u32);
        let lprcItem = <Option<&mut RECT>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::GetMenuItemRect_pos,
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
        let result = winapi::user32::GetMenuItemRect(machine, hWnd, hMenu, uItem, lprcItem);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetMessageA(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
        let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
        let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
        let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::GetMessageA_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn GetMessageW(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
        let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
        let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
        let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::GetMessageW_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::user32::GetMessageW(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn GetQueueStatus(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let flags = <Result<QS, u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::GetQueueStatus_pos,
                "user32/message",
                "GetQueueStatus",
                &[("flags", &flags)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetQueueStatus(machine, flags);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetSubMenu(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
        let nPos = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::GetSubMenu_pos,
                "user32/menu",
                "GetSubMenu",
                &[("hMenu", &hMenu), ("nPos", &nPos)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetSubMenu(machine, hMenu, nPos);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetSysColor(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nIndex = <i32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetSysColor_pos,
                "user32/misc",
                "GetSysColor",
                &[("nIndex", &nIndex)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetSysColor(machine, nIndex);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetSystemMenu(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let bRevert = <bool>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::GetSystemMenu_pos,
                "user32/menu",
                "GetSystemMenu",
                &[("hWnd", &hWnd), ("bRevert", &bRevert)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetSystemMenu(machine, hWnd, bRevert);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetSystemMetrics(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nIndex = <Result<SM, u32>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::GetSystemMetrics_pos,
                "user32/misc",
                "GetSystemMetrics",
                &[("nIndex", &nIndex)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetSystemMetrics(machine, nIndex);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetUpdateRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
        let bErase = <bool>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::GetUpdateRect_pos,
                "user32/paint",
                "GetUpdateRect",
                &[("hWnd", &hWnd), ("lpRect", &lpRect), ("bErase", &bErase)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetUpdateRect(machine, hWnd, lpRect, bErase);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetWindowDC(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetWindowDC_pos,
                "user32/window",
                "GetWindowDC",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetWindowDC(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetWindowLongA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIndex = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetWindowLongA_pos,
                "user32/window",
                "GetWindowLongA",
                &[("hWnd", &hWnd), ("nIndex", &nIndex)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetWindowLongA(machine, hWnd, nIndex);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetWindowPlacement(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpwndpl = <Option<&mut WINDOWPLACEMENT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetWindowPlacement_pos,
                "user32/window",
                "GetWindowPlacement",
                &[("hWnd", &hWnd), ("lpwndpl", &lpwndpl)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetWindowPlacement(machine, hWnd, lpwndpl);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn GetWindowRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpRect = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::GetWindowRect_pos,
                "user32/window",
                "GetWindowRect",
                &[("hWnd", &hWnd), ("lpRect", &lpRect)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::GetWindowRect(machine, hWnd, lpRect);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn InflateRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let dx = <i32>::from_stack(mem, stack_args + 4u32);
        let dy = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::InflateRect_pos,
                "user32/rect",
                "InflateRect",
                &[("lprc", &lprc), ("dx", &dx), ("dy", &dy)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::InflateRect(machine, lprc, dx, dy);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IntersectRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprcDst = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let lprcSrc1 = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let lprcSrc2 = <Option<&RECT>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::IntersectRect_pos,
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
        let result = winapi::user32::IntersectRect(machine, lprcDst, lprcSrc1, lprcSrc2);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn InvalidateRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let bErase = <bool>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::InvalidateRect_pos,
                "user32/paint",
                "InvalidateRect",
                &[("hWnd", &hWnd), ("lpRect", &lpRect), ("bErase", &bErase)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn InvalidateRgn(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let hRgn = <HRGN>::from_stack(mem, stack_args + 4u32);
        let bErase = <bool>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::InvalidateRgn_pos,
                "user32/paint",
                "InvalidateRgn",
                &[("hWnd", &hWnd), ("hRgn", &hRgn), ("bErase", &bErase)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn InvertRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDC = <HDC>::from_stack(mem, stack_args + 0u32);
        let lpr = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::InvertRect_pos,
                "user32/paint",
                "InvertRect",
                &[("hDC", &hDC), ("lpr", &lpr)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::InvertRect(machine, hDC, lpr);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IsDlgButtonChecked(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDButton = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::IsDlgButtonChecked_pos,
                "user32/dialog",
                "IsDlgButtonChecked",
                &[("hDlg", &hDlg), ("nIDButton", &nIDButton)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::IsDlgButtonChecked(machine, hDlg, nIDButton);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IsIconic(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::IsIconic_pos,
                "user32/misc",
                "IsIconic",
                &[("hwnd", &hwnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::IsIconic(machine, hwnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IsRectEmpty(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::IsRectEmpty_pos,
                "user32/rect",
                "IsRectEmpty",
                &[("lprc", &lprc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::IsRectEmpty(machine, lprc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn KillTimer(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let uIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/timer") {
            crate::trace::Record::new(
                winapi::user32::KillTimer_pos,
                "user32/timer",
                "KillTimer",
                &[("hWnd", &hWnd), ("uIDEvent", &uIDEvent)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::KillTimer(machine, hWnd, uIDEvent);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpTableName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadAcceleratorsW_pos,
                "user32/resource",
                "LoadAcceleratorsW",
                &[("hInstance", &hInstance), ("lpTableName", &lpTableName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadBitmapA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <HINSTANCE>::from_stack(mem, stack_args + 0u32);
        let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadBitmapA_pos,
                "user32/resource",
                "LoadBitmapA",
                &[("hInstance", &hInstance), ("lpBitmapName", &lpBitmapName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadCursorA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadCursorA_pos,
                "user32/resource",
                "LoadCursorA",
                &[("hInstance", &hInstance), ("lpCursorName", &lpCursorName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadCursorA(machine, hInstance, lpCursorName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadCursorW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpCursorName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadCursorW_pos,
                "user32/resource",
                "LoadCursorW",
                &[("hInstance", &hInstance), ("lpCursorName", &lpCursorName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadCursorW(machine, hInstance, lpCursorName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadIconA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadIconA_pos,
                "user32/resource",
                "LoadIconA",
                &[("hInstance", &hInstance), ("lpIconName", &lpIconName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadIconA(machine, hInstance, lpIconName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadIconW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpIconName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadIconW_pos,
                "user32/resource",
                "LoadIconW",
                &[("hInstance", &hInstance), ("lpIconName", &lpIconName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadIconW(machine, hInstance, lpIconName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadImageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let name = <ResourceKey<&str>>::from_stack(mem, stack_args + 4u32);
        let typ = <u32>::from_stack(mem, stack_args + 8u32);
        let cx = <u32>::from_stack(mem, stack_args + 12u32);
        let cy = <u32>::from_stack(mem, stack_args + 16u32);
        let fuLoad = <u32>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadImageA_pos,
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
        let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadImageW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let name = <ResourceKey<&Str16>>::from_stack(mem, stack_args + 4u32);
        let typ = <u32>::from_stack(mem, stack_args + 8u32);
        let cx = <u32>::from_stack(mem, stack_args + 12u32);
        let cy = <u32>::from_stack(mem, stack_args + 16u32);
        let fuLoad = <u32>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadImageW_pos,
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
        let result = winapi::user32::LoadImageW(machine, hInstance, name, typ, cx, cy, fuLoad);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadMenuA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::LoadMenuA_pos,
                "user32/menu",
                "LoadMenuA",
                &[("hInstance", &hInstance), ("lpMenuName", &lpMenuName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadMenuA(machine, hInstance, lpMenuName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadMenuW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let lpMenuName = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadMenuW_pos,
                "user32/resource",
                "LoadMenuW",
                &[("hInstance", &hInstance), ("lpMenuName", &lpMenuName)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::LoadMenuW(machine, hInstance, lpMenuName);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadStringA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let uID = <u32>::from_stack(mem, stack_args + 4u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
        let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadStringA_pos,
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
        let result = winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn LoadStringW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hInstance = <u32>::from_stack(mem, stack_args + 0u32);
        let uID = <u32>::from_stack(mem, stack_args + 4u32);
        let lpBuffer = <u32>::from_stack(mem, stack_args + 8u32);
        let cchBufferMax = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::LoadStringW_pos,
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
        let result = winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn MapWindowPoints(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWndFrom = <HWND>::from_stack(mem, stack_args + 0u32);
        let hWndTo = <HWND>::from_stack(mem, stack_args + 4u32);
        let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::MapWindowPoints_pos,
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
        let result = winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn MessageBoxA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpText = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let lpCaption = <Option<&str>>::from_stack(mem, stack_args + 8u32);
        let uType = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::MessageBoxA_pos,
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
        let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn MessageBoxW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpText = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let lpCaption = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
        let uType = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::MessageBoxW_pos,
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
        let result = winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn MoveWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let X = <u32>::from_stack(mem, stack_args + 4u32);
        let Y = <u32>::from_stack(mem, stack_args + 8u32);
        let nWidth = <u32>::from_stack(mem, stack_args + 12u32);
        let nHeight = <u32>::from_stack(mem, stack_args + 16u32);
        let bRepaint = <bool>::from_stack(mem, stack_args + 20u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::MoveWindow_pos,
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
        let result = winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn MsgWaitForMultipleObjects(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let nCount = <u32>::from_stack(mem, stack_args + 0u32);
        let pHandles = <u32>::from_stack(mem, stack_args + 4u32);
        let fWaitAll = <bool>::from_stack(mem, stack_args + 8u32);
        let dwMilliseconds = <u32>::from_stack(mem, stack_args + 12u32);
        let dwWakeMask = <Result<QS, u32>>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::MsgWaitForMultipleObjects_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::MsgWaitForMultipleObjects(
                machine,
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
            result.to_raw()
        })
    }
    pub unsafe fn PeekMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
        let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
        let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
        let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
        let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::PeekMessageA_pos,
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
        let result = winapi::user32::PeekMessageA(
            machine,
            lpMsg,
            hWnd,
            wMsgFilterMin,
            wMsgFilterMax,
            wRemoveMsg,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn PeekMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&mut MSG>>::from_stack(mem, stack_args + 0u32);
        let hWnd = <HWND>::from_stack(mem, stack_args + 4u32);
        let wMsgFilterMin = <u32>::from_stack(mem, stack_args + 8u32);
        let wMsgFilterMax = <u32>::from_stack(mem, stack_args + 12u32);
        let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::PeekMessageW_pos,
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
        let result = winapi::user32::PeekMessageW(
            machine,
            lpMsg,
            hWnd,
            wMsgFilterMin,
            wMsgFilterMax,
            wRemoveMsg,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn PostMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let Msg = <u32>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::PostMessageA_pos,
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
        let result = winapi::user32::PostMessageA(machine, hWnd, Msg, wParam, lParam);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn PostMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let Msg = <u32>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::PostMessageW_pos,
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
        let result = winapi::user32::PostMessageW(machine, hWnd, Msg, wParam, lParam);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn PostQuitMessage(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let nExitCode = <i32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::PostQuitMessage_pos,
                "user32/message",
                "PostQuitMessage",
                &[("nExitCode", &nExitCode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::PostQuitMessage(machine, nExitCode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn PostThreadMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let idThread = <u32>::from_stack(mem, stack_args + 0u32);
        let Msg = <u32>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::PostThreadMessageA_pos,
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
        let result = winapi::user32::PostThreadMessageA(machine, idThread, Msg, wParam, lParam);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn PtInRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprc = <Option<&RECT>>::from_stack(mem, stack_args + 0u32);
        let pt = <POINT>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::PtInRect_pos,
                "user32/rect",
                "PtInRect",
                &[("lprc", &lprc), ("pt", &pt)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::PtInRect(machine, lprc, pt);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn RedrawWindow(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lprcUpdate = <Option<&mut RECT>>::from_stack(mem, stack_args + 4u32);
        let hrgnUpdate = <HRGN>::from_stack(mem, stack_args + 8u32);
        let flags = <Result<RDW, u32>>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::RedrawWindow_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::user32::RedrawWindow(machine, hWnd, lprcUpdate, hrgnUpdate, flags).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn RegisterClassA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/wndclass") {
            crate::trace::Record::new(
                winapi::user32::RegisterClassA_pos,
                "user32/wndclass",
                "RegisterClassA",
                &[("lpWndClass", &lpWndClass)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::RegisterClassA(machine, lpWndClass);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn RegisterClassExA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/wndclass") {
            crate::trace::Record::new(
                winapi::user32::RegisterClassExA_pos,
                "user32/wndclass",
                "RegisterClassExA",
                &[("lpWndClassEx", &lpWndClassEx)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn RegisterClassExW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/wndclass") {
            crate::trace::Record::new(
                winapi::user32::RegisterClassExW_pos,
                "user32/wndclass",
                "RegisterClassExW",
                &[("lpWndClassEx", &lpWndClassEx)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::RegisterClassExW(machine, lpWndClassEx);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn RegisterClassW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/wndclass") {
            crate::trace::Record::new(
                winapi::user32::RegisterClassW_pos,
                "user32/wndclass",
                "RegisterClassW",
                &[("lpWndClass", &lpWndClass)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::RegisterClassW(machine, lpWndClass);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn RegisterWindowMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::RegisterWindowMessageA_pos,
                "user32/window",
                "RegisterWindowMessageA",
                &[("lpString", &lpString)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::RegisterWindowMessageA(machine, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn RegisterWindowMessageW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::RegisterWindowMessageW_pos,
                "user32/window",
                "RegisterWindowMessageW",
                &[("lpString", &lpString)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::RegisterWindowMessageW(machine, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn ReleaseCapture(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::ReleaseCapture_pos,
                "user32/window",
                "ReleaseCapture",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::ReleaseCapture(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn ReleaseDC(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let hdc = <HDC>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::ReleaseDC_pos,
                "user32/window",
                "ReleaseDC",
                &[("hwnd", &hwnd), ("hdc", &hdc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::ReleaseDC(machine, hwnd, hdc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SendDlgItemMessageA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let Msg = <u32>::from_stack(mem, stack_args + 8u32);
        let wParam = <u32>::from_stack(mem, stack_args + 12u32);
        let lParam = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::SendDlgItemMessageA_pos,
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
            winapi::user32::SendDlgItemMessageA(machine, hDlg, nIDDlgItem, Msg, wParam, lParam);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SendMessageA(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let Msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::SendMessageA_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn SendMessageW(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let Msg = <Result<WM, u32>>::from_stack(mem, stack_args + 4u32);
        let wParam = <u32>::from_stack(mem, stack_args + 8u32);
        let lParam = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::SendMessageW_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::SendMessageW(machine, hWnd, Msg, wParam, lParam).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn SetCapture(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hwnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::SetCapture_pos,
                "user32/window",
                "SetCapture",
                &[("hwnd", &hwnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetCapture(machine, hwnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetClassLongA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIndex = <Result<GCL, u32>>::from_stack(mem, stack_args + 4u32);
        let dwNewLong = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/wndclass") {
            crate::trace::Record::new(
                winapi::user32::SetClassLongA_pos,
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
        let result = winapi::user32::SetClassLongA(machine, hWnd, nIndex, dwNewLong);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetCursor(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hCursor = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::SetCursor_pos,
                "user32/resource",
                "SetCursor",
                &[("hCursor", &hCursor)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetCursor(machine, hCursor);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetCursorPos(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let x = <i32>::from_stack(mem, stack_args + 0u32);
        let y = <i32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::SetCursorPos_pos,
                "user32/misc",
                "SetCursorPos",
                &[("x", &x), ("y", &y)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetCursorPos(machine, x, y);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetDlgItemInt(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let uValue = <u32>::from_stack(mem, stack_args + 8u32);
        let _bSigned = <bool>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::SetDlgItemInt_pos,
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
        let result = winapi::user32::SetDlgItemInt(machine, hDlg, nIDDlgItem, uValue, _bSigned);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetDlgItemTextA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::SetDlgItemTextA_pos,
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
        let result = winapi::user32::SetDlgItemTextA(machine, hDlg, nIDDlgItem, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetDlgItemTextW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hDlg = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDDlgItem = <i32>::from_stack(mem, stack_args + 4u32);
        let lpString = <Option<&Str16>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/dialog") {
            crate::trace::Record::new(
                winapi::user32::SetDlgItemTextW_pos,
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
        let result = winapi::user32::SetDlgItemTextW(machine, hDlg, nIDDlgItem, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetFocus(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::SetFocus_pos,
                "user32/window",
                "SetFocus",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetFocus(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetForegroundWindow(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::SetForegroundWindow_pos,
                "user32/window",
                "SetForegroundWindow",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetForegroundWindow(machine, hWnd);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetMenu(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let hMenu = <HMENU>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::SetMenu_pos,
                "user32/menu",
                "SetMenu",
                &[("hWnd", &hWnd), ("hMenu", &hMenu)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetMenu(machine, hWnd, hMenu);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetMenuItemInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hMenu = <HMENU>::from_stack(mem, stack_args + 0u32);
        let item = <u32>::from_stack(mem, stack_args + 4u32);
        let fByPosition = <bool>::from_stack(mem, stack_args + 8u32);
        let lpmii = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/menu") {
            crate::trace::Record::new(
                winapi::user32::SetMenuItemInfoA_pos,
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
        let result = winapi::user32::SetMenuItemInfoA(machine, hMenu, item, fByPosition, lpmii);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let xLeft = <i32>::from_stack(mem, stack_args + 4u32);
        let yTop = <i32>::from_stack(mem, stack_args + 8u32);
        let xRight = <i32>::from_stack(mem, stack_args + 12u32);
        let yBottom = <i32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::SetRect_pos,
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
        let result = winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetRectEmpty(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lprc = <Option<&mut RECT>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/rect") {
            crate::trace::Record::new(
                winapi::user32::SetRectEmpty_pos,
                "user32/rect",
                "SetRectEmpty",
                &[("lprc", &lprc)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetRectEmpty(machine, lprc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetTimer(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIDEvent = <u32>::from_stack(mem, stack_args + 4u32);
        let uElapse = <u32>::from_stack(mem, stack_args + 8u32);
        let lpTimerFunc = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/timer") {
            crate::trace::Record::new(
                winapi::user32::SetTimer_pos,
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
        let result = winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetWindowLongA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let nIndex = <u32>::from_stack(mem, stack_args + 4u32);
        let dwNewLong = <i32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::SetWindowLongA_pos,
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
        let result = winapi::user32::SetWindowLongA(machine, hWnd, nIndex, dwNewLong);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn SetWindowPos(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let hWndInsertAfter = <HWND>::from_stack(mem, stack_args + 4u32);
        let X = <i32>::from_stack(mem, stack_args + 8u32);
        let Y = <i32>::from_stack(mem, stack_args + 12u32);
        let cx = <i32>::from_stack(mem, stack_args + 16u32);
        let cy = <i32>::from_stack(mem, stack_args + 20u32);
        let uFlags = <Result<SWP, u32>>::from_stack(mem, stack_args + 24u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::SetWindowPos_pos,
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
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::user32::SetWindowPos(machine, hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags)
                    .await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn SetWindowTextA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpString = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::SetWindowTextA_pos,
                "user32/window",
                "SetWindowTextA",
                &[("hWnd", &hWnd), ("lpString", &lpString)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::SetWindowTextA(machine, hWnd, lpString);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn ShowCursor(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let bShow = <bool>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/resource") {
            crate::trace::Record::new(
                winapi::user32::ShowCursor_pos,
                "user32/resource",
                "ShowCursor",
                &[("bShow", &bShow)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::ShowCursor(machine, bShow);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn ShowWindow(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let nCmdShow = <Result<SW, u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::ShowWindow_pos,
                "user32/window",
                "ShowWindow",
                &[("hWnd", &hWnd), ("nCmdShow", &nCmdShow)],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::ShowWindow(machine, hWnd, nCmdShow).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn SystemParametersInfoA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let uiAction = <u32>::from_stack(mem, stack_args + 0u32);
        let uiParam = <u32>::from_stack(mem, stack_args + 4u32);
        let pvParam = <Option<&mut u8>>::from_stack(mem, stack_args + 8u32);
        let fWinIni = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::SystemParametersInfoA_pos,
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
            winapi::user32::SystemParametersInfoA(machine, uiAction, uiParam, pvParam, fWinIni);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let hAccTable = <u32>::from_stack(mem, stack_args + 4u32);
        let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::TranslateAcceleratorW_pos,
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
        let result = winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn TranslateMessage(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpMsg = <Option<&MSG>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::TranslateMessage_pos,
                "user32/message",
                "TranslateMessage",
                &[("lpMsg", &lpMsg)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::TranslateMessage(machine, lpMsg);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn UpdateWindow(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("user32/window") {
            crate::trace::Record::new(
                winapi::user32::UpdateWindow_pos,
                "user32/window",
                "UpdateWindow",
                &[("hWnd", &hWnd)],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::UpdateWindow(machine, hWnd).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn ValidateRect(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWnd = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpRect = <Option<&RECT>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("user32/paint") {
            crate::trace::Record::new(
                winapi::user32::ValidateRect_pos,
                "user32/paint",
                "ValidateRect",
                &[("hWnd", &hWnd), ("lpRect", &lpRect)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::ValidateRect(machine, hWnd, lpRect);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn WaitMessage(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let __trace_record = if crate::trace::enabled("user32/message") {
            crate::trace::Record::new(
                winapi::user32::WaitMessage_pos,
                "user32/message",
                "WaitMessage",
                &[],
            )
            .enter()
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result = winapi::user32::WaitMessage(machine).await;
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.to_raw()
        })
    }
    pub unsafe fn WinHelpW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hWndMain = <HWND>::from_stack(mem, stack_args + 0u32);
        let lpszHelp = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let uCommand = <u32>::from_stack(mem, stack_args + 8u32);
        let dwData = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::WinHelpW_pos,
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
        let result = winapi::user32::WinHelpW(machine, hWndMain, lpszHelp, uCommand, dwData);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn keybd_event(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let bVk = <u8>::from_stack(mem, stack_args + 0u32);
        let bScan = <u8>::from_stack(mem, stack_args + 4u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
        let dwExtraInfo = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::keybd_event_pos,
                "user32/misc",
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
        let result = winapi::user32::keybd_event(machine, bVk, bScan, dwFlags, dwExtraInfo);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn wsprintfA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let buf = <u32>::from_stack(mem, stack_args + 0u32);
        let fmt = <Option<&str>>::from_stack(mem, stack_args + 4u32);
        let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::wsprintfA_pos,
                "user32/misc",
                "wsprintfA",
                &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::wsprintfA(machine, buf, fmt, args);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn wsprintfW(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let buf = <u32>::from_stack(mem, stack_args + 0u32);
        let fmt = <Option<&Str16>>::from_stack(mem, stack_args + 4u32);
        let args = <VarArgs>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("user32/misc") {
            crate::trace::Record::new(
                winapi::user32::wsprintfW_pos,
                "user32/misc",
                "wsprintfW",
                &[("buf", &buf), ("fmt", &fmt), ("args", &args)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::user32::wsprintfW(machine, buf, fmt, args);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 137usize] = [
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
        name: "GetKeyState",
        func: Handler::Sync(wrappers::GetKeyState),
    },
    Shim {
        name: "GetKeyboardLayout",
        func: Handler::Sync(wrappers::GetKeyboardLayout),
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
        func: Handler::Sync(wrappers::SetFocus),
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
        name: "TranslateAcceleratorW",
        func: Handler::Sync(wrappers::TranslateAcceleratorW),
    },
    Shim {
        name: "TranslateMessage",
        func: Handler::Sync(wrappers::TranslateMessage),
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
    raw: std::include_bytes!("../../../dll/user32.dll"),
};
