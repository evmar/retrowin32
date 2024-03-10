use super::{Object, DC, HDC, HGDIOBJ};
use crate::winapi::handle::Handles;

pub struct State {
    pub dcs: Handles<HDC, DC>,
    pub desktop_dc: HDC,
    pub objects: Handles<HGDIOBJ, Object>,
}

impl Default for State {
    fn default() -> Self {
        let mut dcs: Handles<HDC, DC> = Default::default();
        let desktop_dc = dcs.reserve();
        State {
            dcs,
            desktop_dc,
            objects: Handles::new(HGDIOBJ::lowest_value()),
        }
    }
}
