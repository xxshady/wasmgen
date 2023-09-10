use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

thread_local! {
    static REGION: Region<'static, System> = Region::new(&GLOBAL);
}

wasm_codegen::guest!("../wasm.interface");

impl guest::exports::Exports for guest::exports::ExportsImpl {
    fn give_string_to_guest(string: String) {
        guest::imports::log(&format!("string from host: {string:?}"));
    }

    fn return_string_to_host() -> String {
        "string for host ಥ_ಥ".to_string()
    }
}

#[no_mangle]
pub fn __pre_main() {
    REGION.with(|v| {
        println!("stats 1: {:#?}", v.change());
    });

    let string = "string for host (❁´◡`❁) and: ᓚᘏᗢ".to_string();
    guest::imports::give_string_to_host(&string);
    // let return_string_to_guest = guest::imports::return_string_to_guest();
    // guest::imports::log(&format!("return_string_to_guest: {return_string_to_guest}"));
}

#[no_mangle]
pub fn main() {
    // let string = "llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@wdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwdddddddddd".to_string();

    // let string = "dada123_(❁´◡`❁)_ёклмн_end".to_string();

    // for _ in 1..=25_000 {

    println!("main fn");

    REGION.with(|v| {
        println!("stats 2: {:#?}", v.change());
    });

    // let fat_ptr = send_to_host(&string);
    // unsafe {
    //     extern "C" {
    //         fn send_string_to_host(fat_ptr: u64);
    //     }
    //     let ptr = string.as_ptr();
    //     let len = string.len();
    //     let fat_ptr = to_fat_ptr(ptr as u32, len as u32);
    //     send_string_to_host(fat_ptr);
    // }
    // }
}
