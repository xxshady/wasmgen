use proc_macro2::TokenStream;
use std::fs;

pub use quote::quote;
pub use wasm_codegen_impl;

const AUTO_GENERATED: &str = "
    // AUTO-GENERATED
    // All manual changes will be overwritten

";

#[doc(hidden)]
pub fn __generate_bindings(get_code: impl FnOnce() -> TokenStream, side: &str) {
    let code = get_code();
    let side: syn::Ident = syn::parse_str(side).unwrap();

    let code = quote! {
        #code
        pub use #side::*;
    };

    let formatted_host_bindings =
        prettyplease::unparse(&syn::parse_file(&code.to_string()).unwrap_or_else(|e| {
            panic!("prettyplease failed with error: {e}, code: {code}");
        }));

    let path = format!("src/{side}.rs");
    fs::write(path, format!("{AUTO_GENERATED}{formatted_host_bindings}")).unwrap();
}

#[macro_export]
macro_rules! generate_bindings {
    ($side:ident, $interface_file:literal) => {
        $crate::__generate_bindings(
            || $crate::wasm_codegen_impl::$side($crate::quote! { $interface_file }),
            stringify!($side),
        );
    };
}
