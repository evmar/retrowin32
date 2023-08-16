//! Unimplemented version of shims for non-emulated cpu case.
//! Currently bails with todo!().

use crate::{shims::Shim, Machine};

pub struct Shims {}

impl Shims {
    pub fn new() -> Self {
        Shims {}
    }

    pub fn add(&mut self, shim: Shim) -> u32 {
        todo!()
    }
}

pub fn become_async(
    machine: &mut Machine,
    future: std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
) {
    todo!()
}

pub struct UnimplFuture {}
impl std::future::Future for UnimplFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub fn async_call(machine: &mut Machine, func: u32, args: Vec<u32>) -> UnimplFuture {
    todo!()
}
