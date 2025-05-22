/// This trait is implemented for u32/u16/u8 and lets us write operations generically
/// over all those bit sizes.
pub(crate) trait Int: num_traits::PrimInt {
    fn as_usize(self) -> usize;
    fn bits() -> usize;
    fn high_bit(&self) -> Self {
        *self >> (Self::bits() - 1)
    }
}
impl Int for u64 {
    fn as_usize(self) -> usize {
        unimplemented!()
    }
    fn bits() -> usize {
        64
    }
}
impl Int for u32 {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn bits() -> usize {
        32
    }
}
impl Int for u16 {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn bits() -> usize {
        16
    }
}
impl Int for u8 {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn bits() -> usize {
        8
    }
}
