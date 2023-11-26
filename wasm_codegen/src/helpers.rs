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

pub(crate) fn ascii_camel_or_pascal_to_snake_case(str: &str) -> String {
    let mut current_part = String::new();
    let mut parts = vec![];
    let mut add_part = |current_part: &mut String| {
        parts.push(std::mem::take(current_part));
    };

    for c in str.chars() {
        if (c.is_uppercase() || c == '_') && current_part.len() > 1 {
            if c != '_' {
                current_part.push('_');
            }
            add_part(&mut current_part);
        }
        current_part.push(c);
    }
    add_part(&mut current_part);

    parts
        .into_iter()
        .map(|part| part.to_lowercase())
        .collect::<Vec<_>>()
        .join("")
}

pub(crate) fn ascii_snake_to_pascal_case(str: &str) -> String {
    let mut current_part = String::new();
    let mut parts = vec![];
    let mut add_part = |current_part: &mut String| {
        parts.push(std::mem::take(current_part));
    };

    for c in str.chars() {
        if (c.is_uppercase() || c == '_') && current_part.len() > 1 {
            add_part(&mut current_part);
        }

        if c == '_' {
            continue;
        }
        current_part.push(c);
    }
    add_part(&mut current_part);

    parts
        .into_iter()
        .map(|part| {
            let mut chars = part.chars();
            let first_char = chars.next().unwrap().to_ascii_uppercase();
            format!("{first_char}{}", chars.collect::<String>())
        })
        .collect::<Vec<_>>()
        .join("")
}
