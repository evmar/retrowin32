use memory::Extensions;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
unsafe impl memory::Pod for POINT {}

impl POINT {
    pub fn add(&self, delta: POINT) -> POINT {
        POINT {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }

    pub fn sub(&self, delta: POINT) -> POINT {
        POINT {
            x: self.x - delta.x,
            y: self.y - delta.y,
        }
    }

    pub fn mul(&self, o: POINT) -> POINT {
        POINT {
            x: self.x * o.x,
            y: self.y * o.y,
        }
    }

    pub fn div(&self, o: POINT) -> POINT {
        POINT {
            x: self.x / o.x,
            y: self.y / o.y,
        }
    }
}

impl<'a> crate::calling_convention::FromStack<'a> for POINT {
    fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let x = mem.get_pod::<i32>(sp);
        let y = mem.get_pod::<i32>(sp + 4);
        POINT { x, y }
    }
}
