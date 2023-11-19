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

pub fn host(main_interface_path: &str, extra_interface_paths: &[&str]) -> TokenStream {
    let exports = host::gen_exports(main_interface_path);
    let imports = host::impl_imports(main_interface_path, extra_interface_paths);
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

pub fn guest(main_interface_path: &str, extra_interface_paths: &[&str]) -> TokenStream {
    let helpers = guest::gen_helpers();
    let imports = guest::gen_imports(main_interface_path, extra_interface_paths);
    let exports = guest::impl_exports(main_interface_path);
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
