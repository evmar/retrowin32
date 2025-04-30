use super::{BrushOrColor, HBRUSH, HCURSOR, HICON, HINSTANCE};
use crate::{
    Machine, System,
    calling_convention::FromArg,
    winapi::{HWND, Str16},
};
use bitflags::bitflags;
use memory::Extensions;
use std::{cell::RefCell, rc::Rc};

bitflags! {
    /// CS_ class style flags for window classes.
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct CS: u32 {
        const VREDRAW         = 0x0001;
        const HREDRAW         = 0x0002;
        const DBLCLKS         = 0x0008;
        const OWNDC           = 0x0020;
        const CLASSDC         = 0x0040;
        const PARENTDC        = 0x0080;
        const NOCLOSE         = 0x0200;
        const SAVEBITS        = 0x0800;
        const BYTEALIGNCLIENT = 0x1000;
        const BYTEALIGNWINDOW = 0x2000;
        const GLOBALCLASS     = 0x4000;
        const DROPSHADOW  = 0x00020000;
    }
}

/// Our internal representation of a window class, as created by RegisterClass etc.
pub struct WndClass {
    pub name: String,
    pub style: CS,
    pub wndproc: u32,
    pub background: HBRUSH,
}

/// The collection of known window classes.
// These generally don't change, but SetWindowLong lets you poke at most of their fields,
// so RefCell it is.
#[derive(Default)]
pub struct WndClasses {
    vec: Vec<Rc<RefCell<WndClass>>>,
}

impl WndClasses {
    pub fn register(&mut self, class: WndClass) -> u32 {
        let atom = self.vec.len() as u32 + 1;
        self.vec.push(Rc::new(RefCell::new(class)));
        atom
    }

    fn by_name(&self, name: &str) -> Option<Rc<RefCell<WndClass>>> {
        self.vec
            .iter()
            .find(|class| class.borrow().name == name)
            .cloned()
    }

    pub fn get(&mut self, name: &str) -> Option<Rc<RefCell<WndClass>>> {
        if let Some(class) = self.by_name(name) {
            return Some(class);
        }
        match name {
            "BUTTON" => {
                self.register(button_class());
                return self.by_name(name);
            }
            _ => None,
        }
    }
}

fn button_class() -> WndClass {
    WndClass {
        name: "BUTTON".to_string(),
        style: CS::VREDRAW | CS::HREDRAW,
        wndproc: 0,
        background: HBRUSH::null(),
    }
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WNDCLASSA {
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: u32,
    lpszMenuName: u32,
    lpszClassName: u32,
}
unsafe impl memory::Pod for WNDCLASSA {}

#[win32_derive::dllexport]
pub fn RegisterClassA(machine: &mut Machine, lpWndClass: Option<&WNDCLASSA>) -> u32 {
    let wndclass = lpWndClass.unwrap();
    let ex = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        style: wndclass.style,
        lpfnWndProc: wndclass.lpfnWndProc,
        cbClsExtra: wndclass.cbClsExtra,
        cbWndExtra: wndclass.cbWndExtra,
        hInstance: wndclass.hInstance,
        hIcon: wndclass.hIcon,
        hCursor: wndclass.hCursor,
        hbrBackground: wndclass.hbrBackground,
        lpszMenuName: wndclass.lpszMenuName,
        lpszClassName: wndclass.lpszClassName,
        hIconSm: 0,
    };
    RegisterClassExA(machine, Some(&ex))
}

#[win32_derive::dllexport]
pub fn RegisterClassW(machine: &mut Machine, lpWndClass: Option<&WNDCLASSA>) -> u32 {
    // TODO: calling the *W variants tags the windows as expecting wide messages(!).
    let lpWndClass = lpWndClass.unwrap();
    let name = Str16::from_nul_term_ptr(machine.mem(), lpWndClass.lpszClassName).unwrap();
    let background = BrushOrColor::from_arg(machine.mem(), lpWndClass.hbrBackground);
    let wndclass = WndClass {
        name: name.to_string(),
        style: CS::from_bits(lpWndClass.style).unwrap(),
        wndproc: lpWndClass.lpfnWndProc,
        background: background.to_brush(machine),
    };
    machine.state.user32.wndclasses.register(wndclass)
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WNDCLASSEXA {
    cbSize: u32,
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: u32,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl memory::Pod for WNDCLASSEXA {}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WNDCLASSEXW {
    cbSize: u32,
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: u32,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl memory::Pod for WNDCLASSEXW {}

#[win32_derive::dllexport]
pub fn RegisterClassExA(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXA>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let name = machine
        .mem()
        .slicez(lpWndClassEx.lpszClassName)
        .try_ascii()
        .unwrap()
        .to_string();
    let wndclass = WndClass {
        name,
        style: CS::from_bits(lpWndClassEx.style).unwrap(),
        wndproc: lpWndClassEx.lpfnWndProc,
        background: BrushOrColor::from_arg(machine.mem(), lpWndClassEx.hbrBackground)
            .to_brush(machine),
    };
    machine.state.user32.wndclasses.register(wndclass)
}

#[win32_derive::dllexport]
pub fn RegisterClassExW(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXW>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let name = Str16::from_nul_term_ptr(machine.mem(), lpWndClassEx.lpszClassName)
        .unwrap()
        .to_string();
    let wndclass = WndClass {
        name,
        style: CS::from_bits(lpWndClassEx.style).unwrap(),
        wndproc: lpWndClassEx.lpfnWndProc,
        background: BrushOrColor::from_arg(machine.mem(), lpWndClassEx.hbrBackground)
            .to_brush(machine),
    };
    machine.state.user32.wndclasses.register(wndclass)
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum GCL {
    HICONSM = -34,
    STYLE = -26,
    WNDPROC = -24,
    CBCLSEXTRA = -20,
    CBWNDEXTRA = -18,
    HMODULE = -16,
    HICON = -14,
    HCURSOR = -12,
    HBRBACKGROUND = -10,
    MENUNAME = -8,
}

#[win32_derive::dllexport]
pub fn GetClassLongA(machine: &mut Machine, hWnd: HWND, nIndex: Result<GCL, u32>) -> u32 {
    let window = machine.state.user32.windows.get(hWnd).unwrap().borrow();
    let class = window.wndclass.borrow();
    match nIndex.unwrap() {
        GCL::STYLE => class.style.bits(),
        f => todo!("GetClassLongA({f:?})"),
    }
}

#[win32_derive::dllexport]
pub fn SetClassLongA(
    machine: &mut Machine,
    hWnd: HWND,
    nIndex: Result<GCL, u32>,
    dwNewLong: i32,
) -> u32 {
    let window = machine.state.user32.windows.get(hWnd).unwrap().borrow();
    let class = &window.wndclass;
    match nIndex.unwrap() {
        GCL::STYLE => std::mem::replace(
            &mut class.borrow_mut().style,
            CS::try_from(dwNewLong as u32).unwrap(),
        )
        .bits(),
        f @ GCL::HICON => {
            log::warn!("TODO: SetClassLongA({f:?})");
            0
        }
        f => todo!("SetClassLongA({f:?})"),
    }
}

#[win32_derive::dllexport]
pub fn UnregisterClassA(sys: &dyn System, lpClassName: Option<&str>, hInstance: HINSTANCE) -> bool {
    todo!()
}
