use std::{
    backtrace::Backtrace,
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
};

use crate::timers::set_timeout;

pub(crate) struct TimerFuture {
    dest: Instant,
    timer_was_set: bool,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        TimerFuture {
            dest: Instant::now() + duration,
            timer_was_set: false,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.dest {
            println!("TimerFuture ready");
            return Poll::Ready(());
        }

        println!("TimerFuture poll");

        // waker could be shared between different tasks (futures)
        if !self.timer_was_set {
            self.timer_was_set = true;

            println!("TimerFuture set timer");

            let waker = cx.waker().clone();
            set_timeout(
                Box::new(move || {
                    waker.wake();
                }),
                self.dest,
            );
        }

        Poll::Pending
    }
}
