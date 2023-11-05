use futures::executor::{LocalPool, LocalSpawner};

mod timer_future;
// The timer we wrote in the previous section:
pub(crate) use timer_future::TimerFuture;

pub(crate) fn new_executor() -> (LocalPool, LocalSpawner) {
    let pool = LocalPool::new();
    let spawner = pool.spawner();
    (pool, spawner)
}
