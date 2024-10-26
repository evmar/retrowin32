use super::{DCTarget, Object, DC, HDC, HGDIOBJ};
use crate::winapi::{self, handle::Handles, types::HWND};

pub struct State {
    pub dcs: Handles<HDC, DC>,
    pub screen_dc: HDC,
    pub objects: Handles<HGDIOBJ, Object>,
}

impl Handles<HGDIOBJ, Object> {
    pub fn new_bitmap_rgba32(&mut self, bmp: winapi::bitmap::BitmapRGBA32) -> HGDIOBJ {
        self.add(super::Object::Bitmap(super::Bitmap::RGBA32(
            std::rc::Rc::new(bmp),
        )))
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
