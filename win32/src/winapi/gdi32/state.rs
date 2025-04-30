use super::{DC, DCTarget, HDC, HGDIOBJ, LOWEST_HGDIOBJ, Object};
use crate::winapi::{Handles, bitmap::Bitmap, user32::Window};
use std::{cell::RefCell, rc::Rc};

pub struct State {
    pub dcs: Handles<HDC, Rc<RefCell<DC>>>,
    pub screen_dc: HDC,
    pub objects: Handles<HGDIOBJ, Object>,
}

pub trait DCHandles {
    fn add_dc(&mut self, dc: DC) -> HDC;
}
impl DCHandles for Handles<HDC, Rc<RefCell<DC>>> {
    fn add_dc(&mut self, dc: DC) -> HDC {
        self.add(Rc::new(RefCell::new(dc)))
    }
}

pub trait GDIHandles {
    fn add_bitmap(&mut self, bmp: Bitmap) -> HGDIOBJ;
    fn get_bitmap(&self, handle: HGDIOBJ) -> Option<&Rc<RefCell<Bitmap>>>;
}
impl GDIHandles for Handles<HGDIOBJ, Object> {
    fn add_bitmap(&mut self, bmp: Bitmap) -> HGDIOBJ {
        self.add(Object::Bitmap(Rc::new(RefCell::new(bmp))))
    }

    fn get_bitmap(&self, handle: HGDIOBJ) -> Option<&Rc<RefCell<Bitmap>>> {
        let object = self.get(handle)?;
        let Object::Bitmap(bmp) = object else {
            return None;
        };
        Some(bmp)
    }
}

impl Default for State {
    fn default() -> Self {
        let mut dcs: Handles<HDC, Rc<RefCell<DC>>> = Default::default();
        let screen_dc = dcs.add_dc(DC::new(DCTarget::DesktopWindow));
        State {
            dcs,
            screen_dc,
            objects: Handles::new(LOWEST_HGDIOBJ),
        }
    }
}

impl State {
    pub fn new_window_dc(&mut self, window: Rc<RefCell<Window>>) -> HDC {
        self.dcs.add_dc(DC::new(DCTarget::Window(window)))
    }
}
