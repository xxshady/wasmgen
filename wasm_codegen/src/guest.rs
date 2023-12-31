use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    guest_import_internal_func::InternalFuncImpl,
    helpers::{
        build_code, parse_interface_file, value_type_to_repr_as_token_stream,
        value_type_to_rust_as_syn_type,
    },
    parser,
    value_type::ValueKind,
};

pub(crate) fn gen_imports(
    main_interface_path: &str,
    extra_interface_paths: &[&str],
) -> TokenStream {
    let mut main_value_types = Default::default();
    let (parser::Interface { mut imports, .. }, main_interface_file) =
        parse_interface_file(main_interface_path, &mut main_value_types);

    let mut extra_interface_files = vec![];

    for interface_path in extra_interface_paths {
        let mut value_types = main_value_types.clone();
        let (
            parser::Interface {
                imports: mut extra_imports,
                ..
            },
            interface_file,
        ) = parse_interface_file(interface_path, &mut value_types);

        extra_interface_files.push(interface_file);
        imports.append(&mut extra_imports);
    }

    let mut funcs = vec![];

    for import in imports {
        match import {
            parser::AnyFunc::Normal(n) => handle_normal_func(
                n,
                |InternalFuncImpl {
                     name,
                     internal_name,
                     param_decls,
                     internal_param_decls,
                     ret,
                     internal_ret,
                     ret_deserialization,
                     big_call_args_init,
                     params_serialization,
                     big_call_args_tail,
                     param_names,
                 }: InternalFuncImpl| {
                    quote! {
                        pub fn #name( #( #param_decls, )* ) #ret {
                            #[link(wasm_import_module = "__custom_imports")]
                            extern "C" {
                                #[link_name = stringify!(#name)]
                                fn #internal_name( #( #internal_param_decls, )* ) #internal_ret;
                            }

                            #[allow(clippy::unnecessary_cast)]
                            {
                                #big_call_args_init
                                #params_serialization
                                #big_call_args_tail

                                #[allow(unused_variables, clippy::let_unit_value)]
                                let call_return = unsafe { #internal_name(#param_names) };
                                #ret_deserialization
                            }
                        }
                    }
                },
                &mut funcs,
            ),
            parser::AnyFunc::MultiFunc(m) => handle_multi_func(m, &mut funcs),
        }
    }

    fn handle_normal_func(
        parser::Func {
            name,
            params,
            ret,
            big_call,
        }: parser::Func,
        declare_pub_wrapper: impl FnOnce(InternalFuncImpl) -> TokenStream,
        funcs: &mut Vec<TokenStream>,
    ) {
        let internal_name = generate_internal_name(&name);
        let name: Ident = syn::parse_str(&name).unwrap();

        let mut param_names = vec![];
        let mut params_signature = vec![];
        let mut internal_param_decls = vec![];
        let mut params_serialization = vec![];

        for parser::Param { name, param_type } in params {
            let name: Ident = syn::parse_str(&name).unwrap();
            let internal_type = value_type_to_repr_as_token_stream(&param_type);
            let serialization = {
                let mut serialization = match param_type.kind {
                    ValueKind::Native => quote! { #name as #internal_type },
                    ValueKind::FatPtr => quote! { super::__internal::send_to_host(&#name) },
                    ValueKind::Bool => quote! { #name as i32 },
                    ValueKind::String => quote! { super::__internal::send_str_to_host(#name) },
                };

                if big_call {
                    serialization = quote! {
                        big_call_args.extend_from_slice(&super::__shared::NumAsU64Arr::into_bytes(#serialization))
                    };
                }

                serialization
            };
            let param_type = value_type_to_rust_as_syn_type(&param_type, false);

            param_names.push(name.clone());
            params_signature.push(quote! {
                #name: #param_type
            });

            if !big_call {
                internal_param_decls.push(quote! { #name: #internal_type });
            }
            params_serialization.push(serialization);
        }

        let (ret_type, internal_ret, ret_deserialization) = if let Some(ret_type) = ret {
            let pub_type = value_type_to_rust_as_syn_type(&ret_type, true);
            let internal_ret_type = value_type_to_repr_as_token_stream(&ret_type);
            let deserialization = match ret_type.kind {
                ValueKind::Native => quote! { call_return as #pub_type },
                ValueKind::FatPtr => {
                    quote! { super::__internal::read_from_host(call_return) }
                }
                ValueKind::Bool => quote! { call_return == 1 },
                ValueKind::String => {
                    quote! { super::__internal::read_string_from_host(call_return) }
                }
            };

            (
                quote! { -> #pub_type },
                quote! { -> #internal_ret_type },
                deserialization,
            )
        } else {
            (quote! {}, quote! {}, quote! {})
        };

        let (big_call_args_init, big_call_args_tail) = if big_call {
            (
                quote! {
                    let mut big_call_args = unsafe {
                        let mut args = Vec::from_raw_parts(
                            BIG_CALL_PTR.get() as *mut u8,
                            super::__shared::BYTES_TO_STORE_U64_32_TIMES, // length
                            super::__shared::BYTES_TO_STORE_U64_32_TIMES, // capacity
                        );
                        // TODO: maybe just set length to 0 in from_raw_parts?
                        args.set_len(0);
                        args
                    };
                },
                quote! {
                    std::mem::forget(big_call_args);
                },
            )
        } else {
            (quote! {}, quote! {})
        };

        let (all_params_serialization, passed_param_names) = if big_call {
            (
                quote! {
                    #(
                        #params_serialization;
                    )*
                },
                quote! {},
            )
        } else {
            (
                quote! {
                    #(
                        let #param_names = #params_serialization;
                    )*
                },
                quote! {
                    #( #param_names, )*
                },
            )
        };

        funcs.push(declare_pub_wrapper(InternalFuncImpl {
            name,
            internal_name,
            param_decls: params_signature,
            param_names: passed_param_names,
            internal_param_decls,
            ret: ret_type,
            internal_ret,
            ret_deserialization,
            big_call_args_init,
            params_serialization: all_params_serialization,
            big_call_args_tail,
        }));
    }

    fn handle_multi_func(
        parser::MultiFunc { name, funcs }: parser::MultiFunc,
        funcs_code: &mut Vec<TokenStream>,
    ) {
        let internal_name = generate_internal_name(&name);
        funcs_code.push(quote! {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = #name]
                fn #internal_name(func_index: u32) -> u64;
            }
        });

        for (index, func) in funcs.into_iter().enumerate() {
            let index = index as u32;
            handle_normal_func(
                func,
                |InternalFuncImpl {
                     name: func_name,
                     internal_name: _,
                     param_decls,
                     internal_param_decls: _,
                     ret,
                     internal_ret: _,
                     ret_deserialization,
                     big_call_args_init,
                     params_serialization,
                     big_call_args_tail,
                     param_names: _,
                 }: InternalFuncImpl| {
                    let full_name: Ident = syn::parse_str(&format!("{name}_{func_name}")).unwrap();
                    quote! {
                        pub fn #full_name( #( #param_decls, )* ) #ret {
                            #[allow(clippy::unnecessary_cast)]
                            {
                                #big_call_args_init
                                #params_serialization
                                #big_call_args_tail

                                #[allow(unused_variables, clippy::let_unit_value)]
                                let call_return = unsafe { #internal_name(#index) };
                                #ret_deserialization
                            }
                        }
                    }
                },
                funcs_code,
            )
        }
    }

    fn generate_internal_name(name: &str) -> Ident {
        syn::parse_str(&format!("__custom_imports_{name}")).unwrap()
    }

    build_code(
        quote! {
            pub mod imports {
                thread_local! {
                    pub(super) static BIG_CALL_PTR: std::cell::Cell<super::__shared::Ptr> = std::cell::Cell::new(0);
                }

                #( #funcs )*
            }
        },
        {
            let mut interface_paths = vec![main_interface_file];
            interface_paths.append(&mut extra_interface_files);
            interface_paths
        },
    )
}

pub(crate) fn impl_exports(relative_path: &str) -> TokenStream {
    let mut main_value_types = Default::default();
    let (parser::Interface { exports, .. }, interface_file) =
        parse_interface_file(relative_path, &mut main_value_types);

    let mut trait_funcs = vec![];
    let mut extern_funcs = vec![];

    for parser::Func {
        name,
        params,
        ret,
        big_call: _, // for now only implemented for guest -> host calls
    } in exports.into_iter().map(|e| match e {
        parser::AnyFunc::Normal(f) => f,
        parser::AnyFunc::MultiFunc(_) => todo!(),
    }) {
        let name: Ident = syn::parse_str(&name).unwrap();
        let exported_name: Ident = syn::parse_str(&format!("__custom_exports_{name}")).unwrap();

        let mut param_trait_decls = vec![];
        let mut param_internal_decls = vec![];
        let mut param_names = vec![];
        let mut param_deserializations = vec![];

        for p in params {
            let name: Ident = syn::parse_str(&p.name).expect("dt");
            let pub_type = value_type_to_rust_as_syn_type(&p.param_type, true);
            let param_internal_type = value_type_to_repr_as_token_stream(&p.param_type);
            let deserialization = match p.param_type.kind {
                ValueKind::Native => quote! { #name as #pub_type },
                ValueKind::FatPtr => {
                    quote! { super::__internal::read_from_host(#name) }
                }
                ValueKind::Bool => quote! { #name == 1 },
                ValueKind::String => {
                    quote! { super::__internal::read_string_from_host(#name) }
                }
            };

            param_trait_decls.push(quote! {
                #name: #pub_type
            });
            param_internal_decls.push(quote! {
                #name: #param_internal_type
            });
            param_deserializations.push(quote! {
                #deserialization
            });
            param_names.push(name);
        }

        let (pub_ret, internal_ret, ret_serialization) = if let Some(ret_type) = ret {
            let internal_type = value_type_to_repr_as_token_stream(&ret_type);
            let serialization = match ret_type.kind {
                ValueKind::Native => quote! { call_return as #internal_type },
                ValueKind::FatPtr => {
                    quote! { super::__internal::send_to_host(&call_return) }
                }
                ValueKind::Bool => quote! { call_return as i32 },
                ValueKind::String => {
                    quote! { super::__internal::send_string_to_host(call_return) }
                }
            };

            let ret_type = value_type_to_rust_as_syn_type(&ret_type, false);

            (
                quote! { -> #ret_type },
                quote! { -> #internal_type },
                serialization,
            )
        } else {
            (quote! {}, quote! {}, quote! {})
        };

        trait_funcs.push(quote! {
            fn #name(#( #param_trait_decls, )* ) #pub_ret;
        });

        extern_funcs.push(quote! {
            #[no_mangle]
            extern "C" fn #exported_name(#( #param_internal_decls, )*) #internal_ret {
                #[allow(clippy::unnecessary_cast)]
                {
                    #(
                        let #param_names = #param_deserializations;
                    )*

                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = <ExportsImpl as Exports>::#name( #( #param_names, )* );
                    #ret_serialization
                }
            }
        });
    }

    build_code(
        quote! {
            pub mod exports {
                pub trait Exports {
                    #( #trait_funcs )*
                }

                pub struct ExportsImpl;

                #( #extern_funcs )*
            }
        },
        vec![interface_file],
    )
}

pub(crate) fn gen_helpers() -> TokenStream {
    quote! {
        mod __internal {
            #[cfg(target_family = "wasm")]
            const _: () = assert!(std::mem::size_of::<usize>() == std::mem::size_of::<u32>());

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

            pub(super) fn send_to_host<T: ?Sized + serde::Serialize>(data: &T) -> super::__shared::FatPtr {
                let encoded = bincode::serialize(data).unwrap();
                let ptr = encoded.as_ptr();
                let size = encoded.len();
                std::mem::forget(encoded);
                super::__shared::to_fat_ptr(ptr as u32, size as u32)
            }

            // used in params
            pub(super) fn send_str_to_host(str: &str) -> super::__shared::FatPtr {
                let ptr = str.as_ptr();
                let size = str.len();
                super::__shared::to_fat_ptr(ptr as u32, size as u32)
            }

            // used in return
            pub(super) fn send_string_to_host(string: String) -> super::__shared::FatPtr {
                let ptr = string.as_ptr();
                let size = string.len();
                std::mem::forget(string);
                super::__shared::to_fat_ptr(ptr as u32, size as u32)
            }

            pub(super) fn read_from_host<T: serde::de::DeserializeOwned>(fat_ptr: super::__shared::FatPtr) -> T {
                let buffer = buffer_from_fat_ptr(fat_ptr);
                bincode::deserialize(&buffer).unwrap()
            }

            pub(super) fn read_string_from_host(fat_ptr: super::__shared::FatPtr) -> String {
                let buffer = buffer_from_fat_ptr(fat_ptr);
                String::from_utf8(buffer).unwrap()
            }
        }
    }
}
