use crate::{
    System,
    winapi::{POINT, RECT},
};

#[win32_derive::dllexport]
pub fn SetRect(
    sys: &dyn System,
    lprc: Option<&mut RECT>,
    xLeft: i32,
    yTop: i32,
    xRight: i32,
    yBottom: i32,
) -> bool {
    let rect = lprc.unwrap();
    *rect = RECT {
        left: xLeft,
        top: yTop,
        right: xRight,
        bottom: yBottom,
    };
    true
}

#[win32_derive::dllexport]
pub fn PtInRect(sys: &dyn System, lprc: Option<&RECT>, pt: POINT) -> bool {
    let rect = lprc.unwrap();
    pt.x >= rect.left && pt.x < rect.right && pt.y >= rect.top && pt.y < rect.bottom
}

#[win32_derive::dllexport]
pub fn IsRectEmpty(sys: &dyn System, lprc: Option<&RECT>) -> bool {
    let rect = lprc.unwrap();
    rect.left >= rect.right || rect.top >= rect.bottom
}

#[win32_derive::dllexport]
pub fn SetRectEmpty(sys: &dyn System, lprc: Option<&mut RECT>) -> bool {
    if lprc.is_none() {
        return false;
    }
    let rect = lprc.unwrap();
    rect.left = 0;
    rect.top = 0;
    rect.right = 0;
    rect.bottom = 0;
    true
}

#[win32_derive::dllexport]
pub fn IntersectRect(
    sys: &dyn System,
    lprcDst: Option<&mut RECT>,
    lprcSrc1: Option<&RECT>,
    lprcSrc2: Option<&RECT>,
) -> bool {
    if lprcDst.is_none() || lprcSrc1.is_none() || lprcSrc2.is_none() {
        return false;
    }
    let dst = lprcDst.unwrap();
    let src1 = lprcSrc1.unwrap();
    let src2 = lprcSrc2.unwrap();
    if IsRectEmpty(sys, lprcSrc1)
        || IsRectEmpty(sys, lprcSrc2)
        || src1.left >= src2.right
        || src1.right <= src2.left
        || src1.top >= src2.bottom
        || src1.bottom <= src2.top
    {
        return false;
    }
    dst.left = src1.left.max(src2.left);
    dst.right = src1.right.min(src2.right);
    dst.top = src1.top.max(src2.top);
    dst.bottom = src1.bottom.min(src2.bottom);
    dst.left < dst.right && dst.top < dst.bottom
}

#[win32_derive::dllexport]
pub fn CopyRect(sys: &dyn System, lprcDst: Option<&mut RECT>, lprcSrc: Option<&RECT>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn InflateRect(sys: &dyn System, lprc: Option<&mut RECT>, dx: i32, dy: i32) -> bool {
    todo!()
}
