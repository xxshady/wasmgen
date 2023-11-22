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
            ($part:expr) => {{
                let mut bytes = [0u8; U64_SIZE];
                bytes[..$part.len()].clone_from_slice(&$part);
                bytes
            }};
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
            prop_give_string_to_guest: wasmtime::TypedFunc<(super::__shared::FatPtr,), ()>,
            prop_give_custom_to_guest: wasmtime::TypedFunc<(super::__shared::FatPtr,), ()>,
            prop_option_bool: wasmtime::TypedFunc<(super::__shared::FatPtr,), ()>,
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
                    prop_give_custom_to_guest: instance
                        .get_typed_func(
                            &mut store,
                            stringify!(__custom_exports_give_custom_to_guest),
                        )
                        .unwrap(),
                    prop_option_bool: instance
                        .get_typed_func(&mut store, stringify!(__custom_exports_option_bool))
                        .unwrap(),
                    memory: instance.get_memory(&mut store, "memory").unwrap(),
                    alloc: instance
                        .get_typed_func(&mut store, "__custom_alloc")
                        .unwrap(),
                    free: instance
                        .get_typed_func(&mut store, "__custom_free")
                        .unwrap(),
                    pre_main: instance.get_typed_func(&mut store, "__pre_main").unwrap(),
                    main: instance.get_typed_func(&mut store, "main").unwrap(),
                    store,
                };
                {
                    let (ptr, size) = exports
                        .alloc_bytes(&[1_u8; super::__shared::BYTES_TO_STORE_U64_32_TIMES])
                        .unwrap();
                    *mutate_big_call_ptr(exports.store.data_mut()) = ptr;
                    let init_big_call: wasmtime::TypedFunc<(super::__shared::Ptr,), ()> = instance
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
                    let call_return = self.prop_return_string_to_host.call(&mut self.store, ())?;
                    self.read_string_from_guest(call_return)
                }
            }
            pub fn call_give_string_to_guest(&mut self, string: &String) -> wasmtime::Result<()> {
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
            pub fn call_give_custom_to_guest(
                &mut self,
                custom: shared::Custom,
            ) -> wasmtime::Result<()> {
                #[allow(clippy::unnecessary_cast)]
                {
                    let custom = self.send_to_guest(&custom)?;
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self
                        .prop_give_custom_to_guest
                        .call(&mut self.store, (custom,))?;
                    Ok(())
                }
            }
            pub fn call_option_bool(&mut self, option_bool: Option<bool>) -> wasmtime::Result<()> {
                #[allow(clippy::unnecessary_cast)]
                {
                    let option_bool = self.send_to_guest(&option_bool)?;
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self
                        .prop_option_bool
                        .call(&mut self.store, (option_bool,))?;
                    Ok(())
                }
            }
        }
    }
    pub mod imports {
        use crate::{
            host_extra::extra_linker_func,
            host_shared::{self, GetBigCallPtr},
        };

        pub fn add_to_linker<
            U: host_shared::Imports + host_shared::GetExtra + host_shared::GetBigCallPtr,
        >(
            linker: &mut wasmtime::Linker<U>,
        ) {
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
                                    let (a,) = host_shared::read_big_call_args(&mut caller)
                                        .with_borrow(|big_call_args| {
                                            (<i32 as super::__shared::NumAsU64Arr>::from_bytes(
                                                big_call_args[0usize..8usize].try_into().unwrap(),
                                            ) as i32,)
                                        });
                                    #[allow(unused_variables, clippy::let_unit_value)]
                                    let call_return = caller.data().multi_test_a(a);
                                    0
                                }
                                1u32 => {
                                    let (b,) = host_shared::read_big_call_args(&mut caller)
                                        .with_borrow(|big_call_args| {
                                            (<i32 as super::__shared::NumAsU64Arr>::from_bytes(
                                                big_call_args[0usize..8usize].try_into().unwrap(),
                                            ) == 1,)
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
            extra_linker_func(linker);
        }
    }
}
pub use host::*;
