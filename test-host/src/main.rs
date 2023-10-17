use std::time::Instant;

use wasmtime::*;

use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

wasm_codegen::host!("../wasm.interface");

struct State {
    wasi: WasiCtx,
    memory: Option<wasmtime::Memory>,
    free: Option<wasmtime::TypedFunc<host::FatPtr, ()>>,
    alloc: Option<wasmtime::TypedFunc<host::Size, host::Ptr>>,
    big_call_ptr: host::Ptr,
}

impl host::imports::Imports for State {
    fn get_big_call_ptr(&self) -> u32 {
        self.big_call_ptr
    }

    fn get_memory(&self) -> Option<wasmtime::Memory> {
        self.memory
    }

    fn get_free(&self) -> Option<wasmtime::TypedFunc<host::FatPtr, ()>> {
        self.free
    }

    fn get_alloc(&self) -> Option<wasmtime::TypedFunc<host::Size, host::Ptr>> {
        self.alloc
    }

    fn set_memory(&mut self, memory: wasmtime::Memory) {
        self.memory.replace(memory);
    }

    fn set_free(&mut self, value: wasmtime::TypedFunc<host::FatPtr, ()>) {
        self.free.replace(value);
    }

    fn set_alloc(&mut self, alloc: wasmtime::TypedFunc<host::Size, host::Ptr>) {
        self.alloc.replace(alloc);
    }

    // fn give_string_to_host(&self, string: String) {
    //     // println!("give_string_to_host string: {string:?}");
    //     assert_eq!(string.len(), string.len());
    // }

    // fn return_string_to_guest(&self) -> String {
    //     let string = "ᗢ".to_string();
    //     // println!("return_string_to_guest: {string:?}");
    //     string
    // }

    // fn log(&self, msg: String) {
    //     println!("[guest log] {msg}");
    // }

    fn big_call_test(
        &self,
        a: bool,
        a2: u8,
        a3: String,
        a4: i32,
        a5: i64,
        a6: u64,
        a7: i32,
        a8: i32,
        a9: i32,
        a10: i32,
        a11: i32,
        a12: i32,
        a13: i32,
        a14: i32,
        a15: i32,
        a16: i32,
        a17: i32,
        a18: i32,
        a19: i32,
        a20: i32,
        a21: i32,
        a22: i32,
        a23: i32,
        a24: i32,
        a25: i32,
        a26: i32,
        a27: i32,
        a28: i32,
        a29: i32,
        a30: i32,
        a31: i32,
        a32: i32,
    ) {
        dbg!(
            a, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
            a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30, a31, a32,
        );

        assert_eq!(i32::MAX, a4);
        assert_eq!(i64::MAX, a5);
        assert_eq!(u64::MAX, a6);
    }
}

fn main() -> wasmtime::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let engine = Engine::default();
    let mut linker = Linker::<State>::new(&engine);

    #[cfg(debug_assertions)]
    let path = "../target/wasm32-wasi/debug/test_guest.wasm";
    #[cfg(not(debug_assertions))]
    let path = "../target/wasm32-wasi/release/test_guest.wasm";

    dbg!(path);
    let module = Module::from_file(&engine, path).unwrap();

    let wasi = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();
    let mut store = Store::new(
        &engine,
        State {
            wasi,
            memory: None,
            free: None,
            alloc: None,
            big_call_ptr: 0,
        },
    );

    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi).unwrap();
    host::imports::add_to_linker(&mut linker);

    let instance = linker.instantiate(&mut store, &module).unwrap();

    let mut exports = host::exports::Exports::new(|s| &mut s.big_call_ptr, store, instance);

    exports.call_main().unwrap();

    // exports.call_pre_main().unwrap();

    // let string = "string for guest (●'◡'●)".to_string();
    // for _ in 1..=10_000 {
    //     exports.call_give_string_to_guest(&string).unwrap();
    //     let string = exports.call_return_string_to_host().unwrap();
    //     assert_eq!(string.len(), string.len());
    // }

    // exports.call_main().unwrap();

    // let mut bench = || {
    //     let now = Instant::now();
    //     // main.call(&mut store, ()).unwrap();
    //     exports.call_main().unwrap();
    //     dbg!(now.elapsed());
    // };

    // for _ in 1..=15 {
    //     bench();
    // }

    Ok(())
}
