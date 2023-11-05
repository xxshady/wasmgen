use std::{cell::RefCell, time::Instant};

thread_local! {
    pub(crate) static TIMERS: RefCell<Vec<Timer>> = Default::default();
}

pub(crate) struct Timer {
    callback: Option<Box<dyn FnOnce()>>,
    dest: Instant,
}

pub(crate) fn run_tick() {
    TIMERS.with_borrow_mut(|timers| {
        let mut to_remove = vec![];

        timers.iter_mut().enumerate().for_each(|(idx, timer)| {
            if timer.dest >= Instant::now() {
                return;
            }

            (timer.callback.take().unwrap())();
            to_remove.push(idx);
        });

        to_remove.into_iter().for_each(|idx| {
            timers.swap_remove(idx);
        });
    })
}

pub(crate) fn set_timeout(callback: Box<dyn FnOnce()>, dest: Instant) {
    TIMERS.with_borrow_mut(|timers| {
        timers.push(Timer {
            callback: Some(callback),
            dest,
        })
    });
}