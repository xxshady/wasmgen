use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use crate::timers::set_timeout;

pub(crate) struct TimerFuture {
    dest: Instant,
    ready_in_next: bool,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        TimerFuture {
            dest: Instant::now() + duration,
            ready_in_next: false,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready_in_next {
            println!("TimerFuture ready");

            Poll::Ready(())
        } else {
            println!("TimerFuture pending");

            self.ready_in_next = true;

            let waker = cx.waker().clone();
            set_timeout(
                Box::new(move || {
                    waker.wake();
                }),
                self.dest,
            );

            Poll::Pending
        }
    }
}
