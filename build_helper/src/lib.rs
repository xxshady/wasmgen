use proc_macro2::TokenStream;
use std::fs;

pub use quote::quote;
pub use wasm_codegen;

const AUTO_GENERATED: &str = "
    // AUTO-GENERATED
    // All manual changes will be overwritten

";

#[doc(hidden)]
pub fn __generate_bindings(code: TokenStream, side: &str, out_file: &str) {
    let side: syn::Ident = syn::parse_str(side).unwrap();

    let code = quote! {
        #code
        pub use #side::*;
    };

    let formatted_bindings =
        prettyplease::unparse(&syn::parse_file(&code.to_string()).unwrap_or_else(|e| {
            panic!("prettyplease failed with error: {e}, code: {code}");
        }));

    let path = format!("src/{out_file}");
    fs::write(path, format!("{AUTO_GENERATED}{formatted_bindings}")).unwrap();
}

#[macro_export]
macro_rules! generate_bindings {
    ($side:ident, $out_file:literal, @interfaces main: $interface_file:literal extra: [ $( $extra_interface_file:literal, )* ]) => {
        $crate::__generate_bindings(
            $crate::wasm_codegen::$side($interface_file, &[
                $( $extra_interface_file, )*
            ]),
            stringify!($side),
            $out_file,
        );
    };
}
