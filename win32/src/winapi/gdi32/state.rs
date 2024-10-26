use super::{DCTarget, Object, DC, HDC, HGDIOBJ};
use crate::winapi::{bitmap::Bitmap, handle::Handles, types::HWND};
use std::{cell::RefCell, rc::Rc};

pub struct State {
    pub dcs: Handles<HDC, DC>,
    pub screen_dc: HDC,
    pub objects: Handles<HGDIOBJ, Object>,
}

impl Handles<HGDIOBJ, Object> {
    pub fn add_bitmap(&mut self, bmp: Bitmap) -> HGDIOBJ {
        self.add(Object::Bitmap(Rc::new(RefCell::new(bmp))))
    }

    pub fn get_bitmap(&self, handle: HGDIOBJ) -> Option<&Rc<RefCell<Bitmap>>> {
        let object = self.get(handle)?;
        let Object::Bitmap(bmp) = object else {
            return None;
        };
        Some(bmp)
    }
}

impl Default for State {
    fn default() -> Self {
        let mut dcs: Handles<HDC, DC> = Default::default();
        let screen_dc = dcs.add(DC::new(DCTarget::Window(HWND::null())));
        State {
            dcs,
            screen_dc,
            objects: Handles::new(HGDIOBJ::lowest_value()),
        }
    }
}

impl State {
    pub fn new_window_dc(&mut self, hwnd: HWND) -> HDC {
        self.dcs.add(DC::new(DCTarget::Window(hwnd)))
    }
}
