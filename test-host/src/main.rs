use std::time::Instant;

use wasmtime::*;

use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

// TODO: fix out of bounds memory access
wasm_codegen::host!("../wasm.interface");

struct State {
    wasi: WasiCtx,
    // memory: Option<Memory>,
    // free: Option<TypedFunc<u64, ()>>,
}

impl host::imports::Imports for State {
    fn give_string_to_host(&self, string: String) {
        println!("give_string_to_host string: {string:?}");
        assert_eq!(string.len(), string.len());
    }

    fn return_string_to_guest(&self) -> String {
        "string for guest ᓚᘏᗢ".to_string()
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
            // free: None,
            // memory: None,
        },
    );

    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi).unwrap();
    host::imports::add_to_linker(&mut linker);

    // linker
    //     .func_wrap(
    //         "env",
    //         "send_string_to_host",
    //         |mut caller: Caller<State>, fat_ptr: u64| {
    //             fn get_memory_and<
    //                 U,
    //                 Params: wasmtime::WasmParams,
    //                 Results: wasmtime::WasmResults,
    //             >(
    //                 caller: &mut wasmtime::Caller<U>,
    //                 and: &'static str,
    //             ) -> (wasmtime::Memory, wasmtime::TypedFunc<Params, Results>) {
    //                 let Some(wasmtime::Extern::Memory(memory)) = caller.get_export("memory") else {
    //                     panic!("Failed to get memory export")
    //                 };
    //                 let Some(wasmtime::Extern::Func(func)) = caller.get_export(and) else {
    //                     panic!("Failed to get {and:?} export")
    //                 };

    //                 (memory, func.typed::<Params, Results>(caller).unwrap())
    //             }

    //             let ptr = (fat_ptr >> 32) as u32;
    //             let size = fat_ptr as u32;
    //             // dbg!(ptr, size);

    //             let (memory, free) = if caller.data().memory.is_some() {
    //                 (caller.data().memory.unwrap(), caller.data().free.unwrap())
    //             } else {
    //                 get_memory_and::<State, u64, ()>(&mut caller, "__custom_custom_free")
    //             };
    //             // let (memory, free) =
    //             //     get_memory_and::<State, u64, ()>(&mut caller, "__custom_custom_free");

    //             let mut buffer = vec![0; size as usize];
    //             memory.read(&caller, ptr as usize, &mut buffer).unwrap();

    //             // let string: String = bincode::deserialize(&buffer).unwrap();
    //             let string = String::from_utf8(buffer).unwrap();

    //             assert_eq!(string.len(), string.len());
    //             // dbg!(string);

    //             free.call(&mut caller, fat_ptr).unwrap();

    //             caller.data_mut().memory.replace(memory);
    //             caller.data_mut().free.replace(free);
    //         },
    //     )
    //     .unwrap();

    let instance = linker.instantiate(&mut store, &module).unwrap();

    let mut exports = host::exports::Exports::new(store, instance);

    // let main = instance
    //     .get_typed_func::<(), ()>(&mut store, "main")
    //     .unwrap();

    // let mut bench = || {
    //     let now = Instant::now();
    //     // main.call(&mut store, ()).unwrap();
    //     exports.call_main().unwrap();
    //     dbg!(now.elapsed());
    // };

    // for _ in 1..=15 {
    //     bench();
    // }

    exports.call_main().unwrap();
    exports
        .call_give_string_to_guest("string for guest (●'◡'●)")
        .unwrap();
    dbg!(exports.call_return_string_to_host().unwrap());

    Ok(())
}
