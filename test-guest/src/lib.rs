wasm_codegen::guest!("../wasm.interface");

impl guest::exports::Exports for guest::exports::ExportsImpl {
    fn give_string_to_guest(string: String) {
        println!("string from host: {string:?}");
    }

    fn return_string_to_host() -> String {
        "string for host ಥ_ಥ".to_string()
    }
}

#[no_mangle]
pub fn __pre_main() {}

#[no_mangle]
pub fn main() {
    // let string = "llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@llwelre_wdwd_wWdWWddwdwdddddddddddddddddddddwd@44344@wdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwddddddddddwdwdwdwdwwddwdwdwdwdwdwdwdddddddddd".to_string();

    // let string = "dada123_(❁´◡`❁)_ёклмн_end".to_string();

    // for _ in 1..=25_000 {
    guest::imports::give_string_to_host("string for host (❁´◡`❁)");
    dbg!(guest::imports::return_string_to_guest());

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

#[no_mangle]
pub fn __custom_custom_free(fat_ptr: u64) {
    let (ptr, size) = from_fat_ptr(fat_ptr);
    unsafe { std::alloc::dealloc(ptr as *mut u8, array_layout(size)) }
}

fn array_layout(len: u32) -> std::alloc::Layout {
    std::alloc::Layout::array::<u8>(len as usize).unwrap()
}

type FatPtr = u64;

fn from_fat_ptr(fat_ptr: u64) -> (u32, u32) {
    let ptr = (fat_ptr >> 32) as u32;
    let size = fat_ptr as u32;
    (ptr, size)
}

fn to_fat_ptr(ptr: u32, size: u32) -> FatPtr {
    ((ptr as u64) << 32) | (size as u64)
}

fn send_to_host<T: ?Sized + serde::Serialize>(data: &T) -> FatPtr {
    let encoded = bincode::serialize(data).unwrap();
    let ptr = encoded.as_ptr();
    let size = encoded.len();
    std::mem::forget(encoded);
    to_fat_ptr(ptr as u32, size as u32)
}
