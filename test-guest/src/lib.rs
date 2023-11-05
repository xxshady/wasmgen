use futures::{
    executor::{LocalPool, LocalSpawner},
    join,
    task::LocalSpawnExt,
};
use r#async::TimerFuture;
use std::{cell::RefCell, time::Duration};

// #[global_allocator]
// static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

// thread_local! {
//     static REGION: Region<'static, System> = Region::new(&GLOBAL);
// }

mod guest;

mod r#async;
use crate::r#async::new_executor;

mod timers;

thread_local! {
    static EXECUTOR: (RefCell<LocalPool>, LocalSpawner) = {
        let (pool, spawner) = new_executor();
        (RefCell::new(pool), spawner)
    };
}

impl guest::exports::Exports for guest::exports::ExportsImpl {
    fn run_tick() {
        timers::run_tick();
        EXECUTOR.with(|(pool, _)| {
            pool.borrow_mut().run_until_stalled();
        })
    }
}

#[no_mangle]
pub fn __pre_main() {
    // REGION.with(|v| {
    //     v.change();
    //     // println!("stats 1: {:#?}", v.change());
    // });

    // for _ in 0..10_000 {
    //     let string = "d".to_string().repeat(10_000);
    //     assert_eq!(string, string);
    // }

    // let mut string = "(❁´◡`❁) ᓚᘏᗢ".to_string();
    // for _ in 1..=10_000 {
    //     string += "k";
    //     guest::imports::give_string_to_host(&string);
    //     let return_string_to_guest = guest::imports::return_string_to_guest();
    //     // guest::imports::log(&format!("return_string_to_guest: {return_string_to_guest}"));
    //     assert_eq!(return_string_to_guest.len(), return_string_to_guest.len());
    // }
}

#[no_mangle]
fn main() {
    // let string = "llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@wdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwdddddddddd".to_string();

    // let string = "dada123_(❁´◡`❁)_ёклмн_end".to_string();

    // REGION.with(|v| {
    //     println!("stats: {:#?}", v.change());
    // });
    // for _ in 1..=25_000 {
    //     guest::imports::give_string_to_host(&string);
    // }

    // guest::imports::big_call_test(
    //     true,
    //     !0,
    //     &"".to_string(),
    //     i32::MAX,
    //     i64::MAX,
    //     u64::MAX,
    //     7,
    //     8,
    //     9,
    //     10,
    //     11,
    //     12,
    //     13,
    //     14,
    //     15,
    //     16,
    //     17,
    //     18,
    //     19,
    //     20,
    //     21,
    //     22,
    //     23,
    //     24,
    //     25,
    //     26,
    //     27,
    //     28,
    //     29,
    //     30,
    //     31,
    //     32,
    // );

    EXECUTOR.with(|(_, spawner)| {
        println!("spawning");

        spawner
            .spawn_local(async {
                println!("before");
                // TimerFuture::new(Duration::from_millis(10)).await;
                // println!("after 1");
                // TimerFuture::new(Duration::from_millis(10)).await;
                // println!("after 2");
                // TimerFuture::new(Duration::from_millis(10)).await;
                // println!("after 3");

                join!(
                    TimerFuture::new(Duration::from_millis(50)),
                    TimerFuture::new(Duration::from_millis(200)),
                    TimerFuture::new(Duration::from_millis(450))
                );
                println!("after");
            })
            .unwrap();
    })

    // println!("before");
    // set_timeout(
    //     Box::new(|| {
    //         println!("after");
    //     }),
    //     Duration::from_millis(36),
    // );
}
