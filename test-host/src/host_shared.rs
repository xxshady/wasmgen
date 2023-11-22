pub mod extra_interfaces {
    pub trait extra_wasm: Sized {
        fn extra_a(&self, a: i32);
        fn extra_b(&self, b: bool);
        fn extra_option_i32(&self, option_i32: Option<i32>);
    }
}

pub(crate) trait GetExtra {
    type ExtraInterface_extra_wasm: extra_interfaces::extra_wasm;
    fn get_extra_wasm(&self) -> &Self::ExtraInterface_extra_wasm;
}

pub(crate) mod __shared {
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

pub(crate) trait GetBigCallPtr {
    fn get_big_call_ptr(&self) -> __shared::Ptr;
}

pub(crate) fn get_memory<U: Imports>(caller: &mut wasmtime::Caller<U>) -> wasmtime::Memory {
    let Some(wasmtime::Extern::Memory(memory)) = caller.get_export("memory") else {
        panic!("Failed to get memory export")
    };
    memory
}
pub(crate) fn read_big_call_args<U: Imports + GetBigCallPtr>(
    caller: &mut wasmtime::Caller<U>,
) -> &'static std::thread::LocalKey<std::cell::RefCell<Vec<u8>>> {
    thread_local! {
        static ARGS : std::cell::RefCell < Vec < u8 >> =
        std::cell::RefCell::new(vec![0u8;
        __shared::BYTES_TO_STORE_U64_32_TIMES]);
    }
    let big_call_ptr = caller.data().get_big_call_ptr();
    ARGS.with_borrow_mut(|args| {
        get_memory(caller)
            .read(caller, big_call_ptr as usize, args)
            .unwrap();
    });
    &ARGS
}
pub(crate) fn get_memory_and<
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
pub(crate) fn read_to_buffer<U: Imports>(
    mut caller: &mut wasmtime::Caller<U>,
    fat_ptr: __shared::FatPtr,
    call_free: bool,
) -> wasmtime::Result<Vec<u8>> {
    let memory = caller.data().get_memory();
    let free = caller.data().get_free();
    let (memory, free) = if free.is_some() {
        (memory.unwrap(), free.unwrap())
    } else {
        get_memory_and(caller, "__custom_free")
    };
    let (ptr, size) = __shared::from_fat_ptr(fat_ptr);
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
pub(crate) fn read_from_guest<U: Imports, T: serde::de::DeserializeOwned>(
    caller: &mut wasmtime::Caller<U>,
    fat_ptr: __shared::FatPtr,
) -> wasmtime::Result<T> {
    let buffer = read_to_buffer(caller, fat_ptr, true)?;
    Ok(bincode::deserialize(&buffer)?)
}
pub(crate) fn read_string_ref_from_guest<U: Imports>(
    caller: &mut wasmtime::Caller<U>,
    fat_ptr: __shared::FatPtr,
) -> wasmtime::Result<String> {
    let buffer = read_to_buffer(caller, fat_ptr, false)?;
    Ok(String::from_utf8(buffer)?)
}
pub(crate) fn clone_bytes_to_guest<U: Imports>(
    mut caller: &mut wasmtime::Caller<U>,
    bytes: &[u8],
) -> wasmtime::Result<__shared::FatPtr> {
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
    Ok(__shared::to_fat_ptr(ptr, size))
}
pub(crate) fn send_to_guest<U: Imports, T: ?Sized + serde::Serialize>(
    caller: &mut wasmtime::Caller<U>,
    data: &T,
) -> wasmtime::Result<__shared::FatPtr> {
    let bytes = bincode::serialize(&data)?;
    clone_bytes_to_guest(caller, &bytes)
}
pub(crate) fn send_string_to_guest<U: Imports>(
    caller: &mut wasmtime::Caller<U>,
    string: String,
) -> wasmtime::Result<__shared::FatPtr> {
    clone_bytes_to_guest(caller, string.as_bytes())
}

pub type FreeFunc = wasmtime::TypedFunc<FatPtr, ()>;
pub type AllocFunc = wasmtime::TypedFunc<Size, Ptr>;

pub trait Imports {
    fn get_memory(&self) -> Option<wasmtime::Memory>;
    fn set_memory(&mut self, memory: wasmtime::Memory);
    fn get_free(&self) -> Option<FreeFunc>;
    fn set_free(&mut self, free: FreeFunc);
    fn get_alloc(&self) -> Option<AllocFunc>;
    fn set_alloc(&mut self, alloc: AllocFunc);
    fn multi_test_a(&self, a: i32);
    fn multi_test_b(&self, b: bool);
}
