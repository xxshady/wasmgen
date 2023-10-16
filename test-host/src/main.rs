use std::time::Instant;

use wasmtime::*;

use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

wasm_codegen::host!("../wasm.interface");

struct State {
    wasi: WasiCtx,
    memory: Option<wasmtime::Memory>,
    free: Option<wasmtime::TypedFunc<host::FatPtr, ()>>,
    alloc: Option<wasmtime::TypedFunc<host::Size, host::Ptr>>,
}

impl host::imports::Imports for State {
    fn give_string_to_host(&self, string: String) {
        // println!("give_string_to_host string: {string:?}");
        assert_eq!(string.len(), string.len());
    }

    fn return_string_to_guest(&self) -> String {
        let string = "ᗢ".to_string();
        // println!("return_string_to_guest: {string:?}");
        string
    }

    fn log(&self, msg: String) {
        println!("[guest log] {msg}");
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
}

fn main() -> wasmtime::Result<()> {
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
        },
    );

    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi).unwrap();
    host::imports::add_to_linker(&mut linker);

    let instance = linker.instantiate(&mut store, &module).unwrap();

    let mut exports = host::exports::Exports::new(store, instance);

    // exports.call_pre_main().unwrap();

    // let string = "string for guest (●'◡'●)".to_string();
    // for _ in 1..=10_000 {
    //     exports.call_give_string_to_guest(&string).unwrap();
    //     let string = exports.call_return_string_to_host().unwrap();
    //     assert_eq!(string.len(), string.len());
    // }

    // exports.call_main().unwrap();

    let mut bench = || {
        let now = Instant::now();
        // main.call(&mut store, ()).unwrap();
        exports.call_main().unwrap();
        dbg!(now.elapsed());
    };

    for _ in 1..=15 {
        bench();
    }

    Ok(())
}
