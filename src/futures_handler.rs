use std::{future::Future, task::Poll};

use tracing::info;

pub struct FutureHandler {}

impl Future for FutureHandler {
    type Output = ();
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // panic!("Oh heck no");

        // unsafe {
        //     *(0xF00D as *mut u64) = 0x0;
        // }

        info!("Future Handler!!");
        Poll::Ready(())
    }
}
