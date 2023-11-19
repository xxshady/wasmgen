use proc_macro2::TokenStream;
use quote::quote;

mod guest;
mod guest_import_internal_func;
mod helpers;
mod host;
mod host_import_internal_func;
mod parser;
mod shared;
mod value_type;

pub fn host(relative_path: &str) -> TokenStream {
    let exports = host::gen_exports(relative_path);
    let imports = host::impl_imports(relative_path);
    let shared = shared::shared_mod();

    quote! {
        mod host {
            #shared

            pub type FreeFunc = wasmtime::TypedFunc<FatPtr, ()>;
            pub type AllocFunc = wasmtime::TypedFunc<Size, Ptr>;

            #exports
            #imports
        }
    }
}

pub fn guest(relative_path: &str) -> TokenStream {
    let helpers = guest::gen_helpers();
    let imports = guest::gen_imports(relative_path);
    let exports = guest::impl_exports(relative_path);
    let shared = shared::shared_mod();

    quote! {
        mod guest {
            #shared
            #helpers
            #imports
            #exports
        }
    }
}
