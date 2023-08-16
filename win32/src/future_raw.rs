//! Unimplemented version of future_cpuemu for non-emulated cpu case.
//! Currently bails with todo!().

use crate::Machine;

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
