use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn shared_mod() -> TokenStream {
    quote! {
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
    }
}
