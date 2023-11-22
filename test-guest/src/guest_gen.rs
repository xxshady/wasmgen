
    // AUTO-GENERATED
    // All manual changes will be overwritten

mod guest {
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
    mod __internal {
        #[cfg(target_family = "wasm")]
        const _: () = assert!(
            std::mem::size_of:: < usize > () == std::mem::size_of:: < u32 > ()
        );
        #[no_mangle]
        pub fn __custom_free(fat_ptr: super::__shared::FatPtr) {
            let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
            unsafe { std::alloc::dealloc(ptr as *mut u8, array_layout(size)) }
        }
        #[no_mangle]
        pub fn __custom_alloc(len: u32) -> super::__shared::Ptr {
            let ptr = unsafe { std::alloc::alloc(array_layout(len)) };
            if ptr.is_null() {
                panic!("Failed to allocate");
            }
            ptr as super::__shared::Ptr
        }
        #[no_mangle]
        pub fn __init_big_call(ptr: super::__shared::Ptr) {
            super::imports::BIG_CALL_PTR.set(ptr);
        }
        fn array_layout(len: u32) -> std::alloc::Layout {
            std::alloc::Layout::array::<u8>(len as usize).unwrap()
        }
        fn buffer_from_fat_ptr(fat_ptr: super::__shared::FatPtr) -> Vec<u8> {
            let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
            unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) }
        }
        pub(super) fn send_to_host<T: ?Sized + serde::Serialize>(
            data: &T,
        ) -> super::__shared::FatPtr {
            let encoded = bincode::serialize(data).unwrap();
            let ptr = encoded.as_ptr();
            let size = encoded.len();
            std::mem::forget(encoded);
            super::__shared::to_fat_ptr(ptr as u32, size as u32)
        }
        pub(super) fn send_str_to_host(str: &str) -> super::__shared::FatPtr {
            let ptr = str.as_ptr();
            let size = str.len();
            super::__shared::to_fat_ptr(ptr as u32, size as u32)
        }
        pub(super) fn send_string_to_host(string: String) -> super::__shared::FatPtr {
            let ptr = string.as_ptr();
            let size = string.len();
            std::mem::forget(string);
            super::__shared::to_fat_ptr(ptr as u32, size as u32)
        }
        pub(super) fn read_from_host<T: serde::de::DeserializeOwned>(
            fat_ptr: super::__shared::FatPtr,
        ) -> T {
            let buffer = buffer_from_fat_ptr(fat_ptr);
            bincode::deserialize(&buffer).unwrap()
        }
        pub(super) fn read_string_from_host(fat_ptr: super::__shared::FatPtr) -> String {
            let buffer = buffer_from_fat_ptr(fat_ptr);
            String::from_utf8(buffer).unwrap()
        }
    }
    pub mod imports {
        thread_local! {
            pub (super) static BIG_CALL_PTR : std::cell::Cell < super::__shared::Ptr > =
            std::cell::Cell::new(0);
        }
        #[link(wasm_import_module = "__custom_imports")]
        extern "C" {
            #[link_name = "multi_test"]
            fn __custom_imports_multi_test(func_index: u32) -> u64;
        }
        pub fn multi_test_a(a: i32) {
            #[allow(clippy::unnecessary_cast)]
            {
                let mut big_call_args = unsafe {
                    let mut args = Vec::from_raw_parts(
                        BIG_CALL_PTR.get() as *mut u8,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                    );
                    args.set_len(0);
                    args
                };
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(a as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_multi_test(0u32) };
            }
        }
        pub fn multi_test_b(b: bool) {
            #[allow(clippy::unnecessary_cast)]
            {
                let mut big_call_args = unsafe {
                    let mut args = Vec::from_raw_parts(
                        BIG_CALL_PTR.get() as *mut u8,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                    );
                    args.set_len(0);
                    args
                };
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(b as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_multi_test(1u32) };
            }
        }
        pub fn alloc_memory_buffer(size: u16) -> u8 {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(alloc_memory_buffer)]
                fn __custom_imports_alloc_memory_buffer(size: u32) -> u32;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let size = size as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_alloc_memory_buffer(size) };
                call_return as u8
            }
        }
        pub fn dealloc_memory_buffer(id: u8) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(dealloc_memory_buffer)]
                fn __custom_imports_dealloc_memory_buffer(id: u32);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let id = id as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_dealloc_memory_buffer(id) };
            }
        }
        pub fn random_shit_to_memory_buffer(id: u8) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(random_shit_to_memory_buffer)]
                fn __custom_imports_random_shit_to_memory_buffer(id: u32);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let id = id as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_random_shit_to_memory_buffer(id)
                };
            }
        }
        pub fn read_memory_buffer(id: u8) -> Vec<u8> {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(read_memory_buffer)]
                fn __custom_imports_read_memory_buffer(
                    id: u32,
                ) -> super::__shared::FatPtr;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let id = id as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_read_memory_buffer(id) };
                super::__internal::read_from_host(call_return)
            }
        }
        #[link(wasm_import_module = "__custom_imports")]
        extern "C" {
            #[link_name = "extra"]
            fn __custom_imports_extra(func_index: u32) -> u64;
        }
        pub fn extra_a(a: i32) {
            #[allow(clippy::unnecessary_cast)]
            {
                let mut big_call_args = unsafe {
                    let mut args = Vec::from_raw_parts(
                        BIG_CALL_PTR.get() as *mut u8,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                    );
                    args.set_len(0);
                    args
                };
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(a as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_extra(0u32) };
            }
        }
        pub fn extra_b(b: bool) {
            #[allow(clippy::unnecessary_cast)]
            {
                let mut big_call_args = unsafe {
                    let mut args = Vec::from_raw_parts(
                        BIG_CALL_PTR.get() as *mut u8,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                    );
                    args.set_len(0);
                    args
                };
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(b as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_extra(1u32) };
            }
        }
        pub fn extra_option_i32(option_i32: Option<i32>) {
            #[allow(clippy::unnecessary_cast)]
            {
                let mut big_call_args = unsafe {
                    let mut args = Vec::from_raw_parts(
                        BIG_CALL_PTR.get() as *mut u8,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                    );
                    args.set_len(0);
                    args
                };
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&option_i32),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_extra(2u32) };
            }
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\wasmgen\\test-guest\\../wasm.interface"#
    );
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\wasmgen\\test-guest\\../extra_wasm.interface"#
    );
    pub mod exports {
        pub trait Exports {
            fn return_string_to_host() -> String;
            fn give_string_to_guest(string: String);
            fn give_custom_to_guest(custom: shared::Custom);
            fn option_bool(option_bool: Option<bool>);
        }
        pub struct ExportsImpl;
        #[no_mangle]
        extern "C" fn __custom_exports_return_string_to_host() -> super::__shared::FatPtr {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::return_string_to_host();
                super::__internal::send_string_to_host(call_return)
            }
        }
        #[no_mangle]
        extern "C" fn __custom_exports_give_string_to_guest(
            string: super::__shared::FatPtr,
        ) {
            #[allow(clippy::unnecessary_cast)]
            {
                let string = super::__internal::read_string_from_host(string);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::give_string_to_guest(string);
            }
        }
        #[no_mangle]
        extern "C" fn __custom_exports_give_custom_to_guest(
            custom: super::__shared::FatPtr,
        ) {
            #[allow(clippy::unnecessary_cast)]
            {
                let custom = super::__internal::read_from_host(custom);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::give_custom_to_guest(custom);
            }
        }
        #[no_mangle]
        extern "C" fn __custom_exports_option_bool(
            option_bool: super::__shared::FatPtr,
        ) {
            #[allow(clippy::unnecessary_cast)]
            {
                let option_bool = super::__internal::read_from_host(option_bool);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::option_bool(option_bool);
            }
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\wasmgen\\test-guest\\../wasm.interface"#
    );
}
pub use guest::*;
