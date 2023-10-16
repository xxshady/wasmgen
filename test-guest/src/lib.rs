use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;

// #[global_allocator]
// static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

// thread_local! {
//     static REGION: Region<'static, System> = Region::new(&GLOBAL);
// }

wasm_codegen::guest!("../wasm.interface");

impl guest::exports::Exports for guest::exports::ExportsImpl {
    fn give_string_to_guest(string: String) {
        assert_eq!(string.len(), string.len());
        // guest::imports::log(&format!("string from host: {string:?}"));
    }

    fn return_string_to_host() -> String {
        "string for host ಥ_ಥ".to_string()
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
pub fn main() {
    let string = "llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@wdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwdddddddddd".to_string();

    // let string = "dada123_(❁´◡`❁)_ёклмн_end".to_string();

    // REGION.with(|v| {
    //     println!("stats: {:#?}", v.change());
    // });
    for _ in 1..=25_000 {
        guest::imports::give_string_to_host(&string);
    }
}
