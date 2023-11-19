use proc_macro2::TokenStream;
use std::path::PathBuf;

use crate::{
    parser,
    value_type::{ValueType, ValueTypePool},
};

pub(crate) fn parse_interface_file(
    interface_file: &str,
    mut value_types: &mut ValueTypePool,
) -> (parser::Interface, String) {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let interface_file = manifest_dir.join(interface_file);
    let interface_file = interface_file.to_string_lossy().to_string();

    (
        parser::read_and_parse_interface(&interface_file, &mut value_types),
        interface_file,
    )
}

pub(crate) fn build_code(code: TokenStream, interface_files: Vec<String>) -> TokenStream {
    let mut code = code.to_string();

    for file in interface_files {
        // this is needed for rustc to rebuild source code if interface file changed
        code += &format!("const _: &str = include_str!(r#{file:?}#);\n\n");
    }

    code.parse().unwrap()
}

pub(crate) fn value_type_to_repr_as_token_stream(value_type: &ValueType) -> TokenStream {
    let repr_str: &str = value_type.repr.clone().into();
    repr_str.parse().unwrap()
}

pub(crate) fn value_type_to_rust_as_syn_type(
    value_type: &ValueType,
    deserialization: bool,
) -> syn::Type {
    let r#type = if deserialization {
        match value_type.de.as_ref() {
            Some(de) => de.clone(),
            None => value_type.name.clone(),
        }
    } else {
        value_type.name.clone()
    };

    syn::parse_str(&r#type).unwrap_or_else(|e| {
        panic!(
            "value_type_to_rust_type_as_token_stream failed with type: {}, error: {e}",
            r#type
        );
    })
}
