
    // AUTO-GENERATED
    // All manual changes will be overwritten

mod host {
    mod __shared {
        pub type FatPtr = u64;
        pub type Size = u32;
        pub type Ptr = u32;
        pub fn from_fat_ptr(fat_ptr: FatPtr) -> (Ptr, Size) {
            let ptr = (fat_ptr >> 32) as Ptr;
            let size = fat_ptr as Size;
            (ptr, size)
        }
        pub fn to_fat_ptr(ptr: Ptr, size: Size) -> FatPtr {
            ((ptr as u64) << 32) | (size as u64)
        }
        const U64_SIZE: usize = std::mem::size_of::<u64>();
        pub const BYTES_TO_STORE_U64_32_TIMES: usize = 32 * U64_SIZE;
        type U64AsBytes = [u8; U64_SIZE];
        pub trait NumAsU64Arr: Copy {
            fn from_bytes(bytes: U64AsBytes) -> Self;
            fn into_bytes(self) -> U64AsBytes;
        }
        macro_rules! copy_to_full_arr {
            ($part:expr) => {
                { let mut bytes = [0u8; U64_SIZE]; bytes[.. $part .len()]
                .clone_from_slice(& $part); bytes }
            };
        }
        impl NumAsU64Arr for f32 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                f32::from_le_bytes(bytes[..4].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for f64 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                f64::from_le_bytes(bytes)
            }
            fn into_bytes(self) -> U64AsBytes {
                self.to_le_bytes()
            }
        }
        impl NumAsU64Arr for u8 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u8::from_le_bytes(bytes[..1].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for u16 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u16::from_le_bytes(bytes[..2].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for u32 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u32::from_le_bytes(bytes[..4].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for u64 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u64::from_le_bytes(bytes)
            }
            fn into_bytes(self) -> U64AsBytes {
                self.to_le_bytes()
            }
        }
        impl NumAsU64Arr for i8 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i8::from_le_bytes(bytes[..1].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for i16 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i16::from_le_bytes(bytes[..2].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for i32 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i32::from_le_bytes(bytes[..4].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for i64 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i64::from_le_bytes(bytes)
            }
            fn into_bytes(self) -> U64AsBytes {
                self.to_le_bytes()
            }
        }
    }
    pub use __shared::{FatPtr, Ptr, Size};
    pub type FreeFunc = wasmtime::TypedFunc<FatPtr, ()>;
    pub type AllocFunc = wasmtime::TypedFunc<Size, Ptr>;
    pub mod exports {
        pub struct Exports<S> {
            prop_return_string_to_host: wasmtime::TypedFunc<(), super::__shared::FatPtr>,
            prop_give_string_to_guest: wasmtime::TypedFunc<
                (super::__shared::FatPtr,),
                (),
            >,
            memory: wasmtime::Memory,
            store: wasmtime::Store<S>,
            alloc: super::AllocFunc,
            free: super::FreeFunc,
            pre_main: wasmtime::TypedFunc<(), ()>,
            main: wasmtime::TypedFunc<(), ()>,
        }
        impl<S> Exports<S> {
            pub fn new(
                mutate_big_call_ptr: impl FnOnce(&mut S) -> &mut super::Ptr,
                mut store: wasmtime::Store<S>,
                instance: wasmtime::Instance,
            ) -> Self {
                let mut exports = Self {
                    prop_return_string_to_host: instance
                        .get_typed_func(
                            &mut store,
                            stringify!(__custom_exports_return_string_to_host),
                        )
                        .unwrap(),
                    prop_give_string_to_guest: instance
                        .get_typed_func(
                            &mut store,
                            stringify!(__custom_exports_give_string_to_guest),
                        )
                        .unwrap(),
                    memory: instance.get_memory(&mut store, "memory").unwrap(),
                    alloc: instance
                        .get_typed_func(&mut store, "__custom_alloc")
                        .unwrap(),
                    free: instance.get_typed_func(&mut store, "__custom_free").unwrap(),
                    pre_main: instance.get_typed_func(&mut store, "__pre_main").unwrap(),
                    main: instance.get_typed_func(&mut store, "main").unwrap(),
                    store,
                };
                {
                    let (ptr, size) = exports
                        .alloc_bytes(
                            &[1_u8; super::__shared::BYTES_TO_STORE_U64_32_TIMES],
                        )
                        .unwrap();
                    *mutate_big_call_ptr(exports.store.data_mut()) = ptr;
                    let init_big_call: wasmtime::TypedFunc<
                        (super::__shared::Ptr,),
                        (),
                    > = instance
                        .get_typed_func(&mut exports.store, "__init_big_call")
                        .unwrap();
                    init_big_call.call(&mut exports.store, (ptr,)).unwrap();
                }
                exports
            }
            pub fn store_mut(&mut self) -> &mut wasmtime::Store<S> {
                &mut self.store
            }
            pub fn alloc_bytes(
                &mut self,
                bytes: &[u8],
            ) -> wasmtime::Result<(super::__shared::Ptr, super::__shared::Size)> {
                let size = bytes.len().try_into()?;
                let ptr = self.alloc.call(&mut self.store, size)?;
                self.memory.write(&mut self.store, ptr as usize, bytes)?;
                Ok((ptr, size))
            }
            fn clone_bytes_to_guest(
                &mut self,
                bytes: &[u8],
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let (ptr, size) = self.alloc_bytes(bytes)?;
                Ok(super::__shared::to_fat_ptr(ptr, size))
            }
            fn send_to_guest<T: ?Sized + serde::Serialize>(
                &mut self,
                data: &T,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let encoded: Vec<u8> = bincode::serialize(&data)?;
                self.clone_bytes_to_guest(&encoded)
            }
            fn send_str_to_guest(
                &mut self,
                str: &str,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                self.clone_bytes_to_guest(str.as_bytes())
            }
            fn read_to_buffer(
                &mut self,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<Vec<u8>> {
                let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                let mut buffer = vec![0; size as usize];
                self.memory.read(&self.store, ptr as usize, &mut buffer)?;
                self.free.call(&mut self.store, fat_ptr).unwrap();
                Ok(buffer)
            }
            fn read_from_guest<T: serde::de::DeserializeOwned>(
                &mut self,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<T> {
                let buffer = self.read_to_buffer(fat_ptr)?;
                Ok(bincode::deserialize(&buffer)?)
            }
            fn read_string_from_guest(
                &mut self,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<String> {
                let buffer = self.read_to_buffer(fat_ptr)?;
                Ok(String::from_utf8(buffer)?)
            }
            pub fn call_pre_main(&mut self) -> wasmtime::Result<()> {
                self.pre_main.call(&mut self.store, ())
            }
            pub fn call_main(&mut self) -> wasmtime::Result<()> {
                self.main.call(&mut self.store, ())
            }
            pub fn call_return_string_to_host(&mut self) -> wasmtime::Result<String> {
                #[allow(clippy::unnecessary_cast)]
                {
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self
                        .prop_return_string_to_host
                        .call(&mut self.store, ())?;
                    self.read_string_from_guest(call_return)
                }
            }
            pub fn call_give_string_to_guest(
                &mut self,
                string: &String,
            ) -> wasmtime::Result<()> {
                #[allow(clippy::unnecessary_cast)]
                {
                    let string = self.send_str_to_guest(&string)?;
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self
                        .prop_give_string_to_guest
                        .call(&mut self.store, (string,))?;
                    Ok(())
                }
            }
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\wasmgen\\test-host\\../wasm.interface"#
    );
    pub mod imports {
        pub trait Imports {
            fn get_memory(&self) -> Option<wasmtime::Memory>;
            fn set_memory(&mut self, memory: wasmtime::Memory);
            fn get_free(&self) -> Option<super::FreeFunc>;
            fn set_free(&mut self, free: super::FreeFunc);
            fn get_alloc(&self) -> Option<super::AllocFunc>;
            fn set_alloc(&mut self, alloc: super::AllocFunc);
            fn get_big_call_ptr(&self) -> super::Ptr;
            fn multi_test_a(&self, a: i32);
            fn multi_test_b(&self, b: bool);
        }
        pub fn add_to_linker<U: Imports>(linker: &mut wasmtime::Linker<U>) {
            fn get_memory<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
            ) -> wasmtime::Memory {
                let Some(wasmtime::Extern::Memory(memory)) = caller.get_export("memory")
                else { panic!("Failed to get memory export") };
                memory
            }
            fn read_big_call_args<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
            ) -> &'static std::thread::LocalKey<std::cell::RefCell<Vec<u8>>> {
                thread_local! {
                    static ARGS : std::cell::RefCell < Vec < u8 >> =
                    std::cell::RefCell::new(vec![0u8;
                    super::__shared::BYTES_TO_STORE_U64_32_TIMES]);
                }
                let big_call_ptr = caller.data().get_big_call_ptr();
                ARGS.with_borrow_mut(|args| {
                    get_memory(caller)
                        .read(caller, big_call_ptr as usize, args)
                        .unwrap();
                });
                &ARGS
            }
            fn get_memory_and<
                U: Imports,
                Params: wasmtime::WasmParams,
                Results: wasmtime::WasmResults,
            >(
                caller: &mut wasmtime::Caller<U>,
                and: &'static str,
            ) -> (wasmtime::Memory, wasmtime::TypedFunc<Params, Results>) {
                let memory = get_memory(caller);
                let Some(wasmtime::Extern::Func(func)) = caller.get_export(and) else {
                    panic!("Failed to get {and:?} export")
                };
                (memory, func.typed::<Params, Results>(caller).unwrap())
            }
            fn read_to_buffer<U: Imports>(
                mut caller: &mut wasmtime::Caller<U>,
                fat_ptr: super::__shared::FatPtr,
                call_free: bool,
            ) -> wasmtime::Result<Vec<u8>> {
                let memory = caller.data().get_memory();
                let free = caller.data().get_free();
                let (memory, free) = if free.is_some() {
                    (memory.unwrap(), free.unwrap())
                } else {
                    get_memory_and(caller, "__custom_free")
                };
                let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                let mut buffer = vec![0; size as usize];
                memory.read(&caller, ptr as usize, &mut buffer)?;
                if call_free {
                    free.call(&mut caller, fat_ptr)?;
                }
                let data = caller.data_mut();
                data.set_memory(memory);
                data.set_free(free);
                Ok(buffer)
            }
            fn read_from_guest<U: Imports, T: serde::de::DeserializeOwned>(
                caller: &mut wasmtime::Caller<U>,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<T> {
                let buffer = read_to_buffer(caller, fat_ptr, true)?;
                Ok(bincode::deserialize(&buffer)?)
            }
            fn read_string_ref_from_guest<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<String> {
                let buffer = read_to_buffer(caller, fat_ptr, false)?;
                Ok(String::from_utf8(buffer)?)
            }
            fn clone_bytes_to_guest<U: Imports>(
                mut caller: &mut wasmtime::Caller<U>,
                bytes: &[u8],
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let (memory, alloc) = {
                    let data = caller.data();
                    (data.get_memory(), data.get_alloc())
                };
                let (memory, alloc) = if alloc.is_some() {
                    (memory.unwrap(), alloc.unwrap())
                } else {
                    get_memory_and(caller, "__custom_alloc")
                };
                let size = bytes.len().try_into()?;
                let ptr = alloc.call(&mut caller, size)?;
                memory.write(&mut caller, ptr as usize, bytes)?;
                let data = caller.data_mut();
                data.set_memory(memory);
                data.set_alloc(alloc);
                Ok(super::__shared::to_fat_ptr(ptr, size))
            }
            fn send_to_guest<U: Imports, T: ?Sized + serde::Serialize>(
                caller: &mut wasmtime::Caller<U>,
                data: &T,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let bytes = bincode::serialize(&data)?;
                clone_bytes_to_guest(caller, &bytes)
            }
            fn send_string_to_guest<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
                string: String,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                clone_bytes_to_guest(caller, string.as_bytes())
            }
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(multi_test),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, func_index: u32| -> u64 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            match func_index {
                                0u32 => {
                                    let (a,) = read_big_call_args(&mut caller)
                                        .with_borrow(|big_call_args| {
                                            (
                                                <i32 as super::__shared::NumAsU64Arr>::from_bytes(
                                                    big_call_args[0usize..8usize].try_into().unwrap(),
                                                ) as i32,
                                            )
                                        });
                                    #[allow(unused_variables, clippy::let_unit_value)]
                                    let call_return = caller.data().multi_test_a(a);
                                    0
                                }
                                1u32 => {
                                    let (b,) = read_big_call_args(&mut caller)
                                        .with_borrow(|big_call_args| {
                                            (
                                                <i32 as super::__shared::NumAsU64Arr>::from_bytes(
                                                    big_call_args[0usize..8usize].try_into().unwrap(),
                                                ) == 1,
                                            )
                                        });
                                    #[allow(unused_variables, clippy::let_unit_value)]
                                    let call_return = caller.data().multi_test_b(b);
                                    0
                                }
                                _ => {
                                    panic!(
                                        "Unknown multi func index: {func_index} in func: {}",
                                        stringify!(multi_test)
                                    );
                                }
                            }
                        }
                    },
                )
                .unwrap();
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\wasmgen\\test-host\\../wasm.interface"#
    );
}
pub use host::*;
