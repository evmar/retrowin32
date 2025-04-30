use crate::POINT;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
unsafe impl memory::Pod for RECT {}

impl RECT {
    pub fn clip(&self, other: &RECT) -> RECT {
        RECT {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub fn origin(&self) -> POINT {
        POINT {
            x: self.left,
            y: self.top,
        }
    }

    pub fn size(&self) -> POINT {
        POINT {
            x: self.right - self.left,
            y: self.bottom - self.top,
        }
    }

    pub fn contains(&self, point: POINT) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }

    pub fn add(&self, delta: POINT) -> RECT {
        RECT {
            left: self.left + delta.x,
            top: self.top + delta.y,
            right: self.right + delta.x,
            bottom: self.bottom + delta.y,
        }
    }
}
