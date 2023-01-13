use core::{future::Future, task::{Context, Poll}, pin::Pin};
use alloc::boxed::Box;

pub mod keyboard;
pub mod simple_executor;

pub struct Task {
    future: Pin<Box<dyn Future<Output = () >>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task { future: Box::pin(future), }
    }
}

impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
