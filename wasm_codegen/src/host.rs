use crate::{host_import_internal_func::InternalFuncImpl, parser, value_type::ValueKind};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::helpers::{
    build_code, parse_interface_file, value_type_to_repr_as_token_stream,
    value_type_to_rust_as_syn_type,
};

pub(crate) fn gen_exports(relative_path: &str) -> TokenStream {
    let mut value_types = Default::default();
    let (parser::Interface { exports, .. }, interface_file) =
        parse_interface_file(relative_path, &mut value_types);

    let mut private_props = vec![];
    let mut private_prop_names = vec![];
    let mut exported_names = vec![];
    let mut pub_methods = vec![];

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
        let name_call: Ident = syn::parse_str(&format!("call_{name}")).unwrap();
        let name_prop: Ident = syn::parse_str(&format!("prop_{name}")).unwrap();
        let exported_name: Ident = syn::parse_str(&format!("__custom_exports_{name}")).unwrap();

        let mut param_names = vec![];
        let mut params_signature = vec![];
        let mut params_prop_types = vec![];
        let mut params_serialization = vec![];

        for parser::Param { name, param_type } in params {
            let name: Ident = syn::parse_str(&name).unwrap();
            let internal_type = value_type_to_repr_as_token_stream(&param_type);
            let serialization = match param_type.kind {
                ValueKind::Native => quote! { #name as #internal_type },
                ValueKind::FatPtr => quote! { self.send_to_guest(&#name)? },
                ValueKind::Bool => quote! { #name as i32 },
                ValueKind::String => quote! { self.send_str_to_guest(&#name)? },
            };
            let param_type = value_type_to_rust_as_syn_type(&param_type, false);

            param_names.push(name.clone());
            params_signature.push(quote! {
                #name: #param_type
            });
            params_prop_types.push(internal_type);
            params_serialization.push(serialization);
        }

        let (ret_type, ret_prop_type, ret_deserialization) = if let Some(ret_type) = ret {
            let pub_type = value_type_to_rust_as_syn_type(&ret_type, true);
            let internal_type = value_type_to_repr_as_token_stream(&ret_type);
            let deserialization = match ret_type.kind {
                ValueKind::Native => quote! { Ok(call_return as #pub_type) },
                ValueKind::FatPtr => quote! { self.read_from_guest(call_return) },
                ValueKind::Bool => quote! { Ok(call_return == 1) },
                ValueKind::String => quote! { self.read_string_from_guest(call_return) },
            };

            (quote! { #pub_type }, internal_type, deserialization)
        } else {
            (quote! { () }, quote! { () }, quote! { Ok(()) })
        };

        private_props.push(quote! {
            #name_prop: wasmtime::TypedFunc<( #( #params_prop_types, )* ), #ret_prop_type>
        });

        private_prop_names.push(name_prop.clone());
        exported_names.push(exported_name);

        pub_methods.push(quote! {
            pub fn #name_call(&mut self, #( #params_signature, )* ) -> wasmtime::Result<#ret_type> {
                #[allow(clippy::unnecessary_cast)]
                {
                    #(
                        let #param_names = #params_serialization;
                    )*

                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self.#name_prop.call(
                        &mut self.store,
                        ( #( #param_names, )* )
                    )?;
                    #ret_deserialization
                }
            }
        });
    }

    build_code(
        quote! {
            pub mod exports {
                pub struct Exports<S> {
                    #( #private_props, )*

                    memory: wasmtime::Memory,
                    store: wasmtime::Store<S>,

                    alloc: super::AllocFunc,
                    free: super::FreeFunc,
                    pre_main: wasmtime::TypedFunc<(), ()>,
                    main: wasmtime::TypedFunc<(), ()>,
                }

                impl<S> Exports<S> {
                    pub fn new(
                        mutate_big_call_ptr: impl FnOnce(&mut S) -> &mut super::Ptr,
                        mut store: wasmtime::Store<S>,
                        instance: wasmtime::Instance
                    ) -> Self {
                        let mut exports = Self {
                            // user funcs
                            #(
                                #private_prop_names: instance.get_typed_func(&mut store, stringify!(#exported_names)).unwrap(),
                            )*

                            // internal
                            memory: instance.get_memory(&mut store, "memory").unwrap(),
                            alloc: instance.get_typed_func(&mut store, "__custom_alloc").unwrap(),
                            free: instance.get_typed_func(&mut store, "__custom_free").unwrap(),
                            pre_main: instance.get_typed_func(&mut store, "__pre_main").unwrap(),

                            // TODO: maybe do something other than panic here if it fails to find main fn?
                            main: instance.get_typed_func(&mut store, "main").unwrap(),

                            store,
                        };

                        {
                            let (ptr, size) = exports
                                .alloc_bytes(&[1_u8; super::__shared::BYTES_TO_STORE_U64_32_TIMES])
                                .unwrap();
                            // println!("allocated big_call size: {size}");
                            *mutate_big_call_ptr(exports.store.data_mut()) = ptr;
                            let init_big_call: wasmtime::TypedFunc<(super::__shared::Ptr,), ()> = instance
                                .get_typed_func(&mut exports.store, "__init_big_call")
                                .unwrap();
                            init_big_call.call(&mut exports.store, (ptr,)).unwrap();
                        }

                        exports
                    }

                    pub fn store_mut(&mut self) -> &mut wasmtime::Store<S> {
                        &mut self.store
                    }

                    pub fn alloc_bytes(&mut self, bytes: &[u8]) -> wasmtime::Result<(
                        super::__shared::Ptr,
                        super::__shared::Size
                    )> {
                        let size = bytes.len().try_into()?;
                        let ptr = self.alloc.call(&mut self.store, size)?;
                        self.memory.write(&mut self.store, ptr as usize, bytes)?;
                        Ok((ptr, size))
                    }

                    fn clone_bytes_to_guest(&mut self, bytes: &[u8]) -> wasmtime::Result<super::__shared::FatPtr> {
                        let (ptr, size) = self.alloc_bytes(bytes)?;
                        Ok(super::__shared::to_fat_ptr(ptr, size))
                    }

                    fn send_to_guest<T: ?Sized + serde::Serialize>(
                        &mut self,
                        data: &T,
                    ) -> wasmtime::Result<super::__shared::FatPtr> {
                        let encoded: Vec<u8> = bincode::serialize(&data)?;
                        self.clone_bytes_to_guest(&encoded)
                    }

                    fn send_str_to_guest(&mut self, str: &str) -> wasmtime::Result<super::__shared::FatPtr> {
                        self.clone_bytes_to_guest(str.as_bytes())
                    }

                    fn read_to_buffer(&mut self, fat_ptr: super::__shared::FatPtr) -> wasmtime::Result<Vec<u8>> {
                        let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                        let mut buffer = vec![0; size as usize];
                        self.memory
                            .read(&self.store, ptr as usize, &mut buffer)?;
                        self.free.call(&mut self.store, fat_ptr).unwrap();

                        Ok(buffer)
                    }

                    fn read_from_guest<T: serde::de::DeserializeOwned>(
                        &mut self,
                        fat_ptr: super::__shared::FatPtr,
                    ) -> wasmtime::Result<T> {
                        let buffer = self.read_to_buffer(fat_ptr)?;
                        Ok(bincode::deserialize(&buffer)?)
                    }

                    fn read_string_from_guest(&mut self, fat_ptr: super::__shared::FatPtr) -> wasmtime::Result<String> {
                        let buffer = self.read_to_buffer(fat_ptr)?;
                        Ok(String::from_utf8(buffer)?)
                    }

                    pub fn call_pre_main(&mut self) -> wasmtime::Result<()> {
                        self.pre_main.call(&mut self.store, ())
                    }

                    pub fn call_main(&mut self) -> wasmtime::Result<()> {
                        self.main.call(&mut self.store, ())
                    }

                    #( #pub_methods )*
                }
            }
        },
        vec![interface_file],
    )
}

pub(crate) fn impl_imports(main_interface_path: &str, extra_interfaces: &[&str]) -> TokenStream {
    let mut main_value_types = Default::default();
    let (parser::Interface { imports, .. }, main_interface_file) =
        parse_interface_file(main_interface_path, &mut main_value_types);

    let (main_trait_methods, main_linker_funcs) = gen_import_internals(imports, quote! {});

    let mut extra_interface_traits = vec![];
    let mut extra_interface_linker_funcs = vec![];
    let mut extra_interface_files = vec![];
    let mut extra_interface_getters = vec![];
    let mut extra_interface_placeholder_types = vec![];

    for interface_path in extra_interfaces {
        let mut value_types = main_value_types.clone();

        let (parser::Interface { imports, .. }, interface_file) =
            parse_interface_file(interface_path, &mut value_types);

        let interface_name = {
            let raw = std::path::Path::new(interface_path)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            syn::parse_str::<syn::Ident>(&raw).unwrap()
        };

        let getter_name: TokenStream = syn::parse_str(&format!("get_{interface_name}")).unwrap();
        let (trait_methods, mut linker_funcs) =
            gen_import_internals(imports, quote! { .#getter_name() });

        let interface_trait = quote! {
            pub trait #interface_name: Sized {
                #( #trait_methods )*
            }
        };
        extra_interface_traits.push(interface_trait);
        extra_interface_files.push(interface_file);
        extra_interface_linker_funcs.append(&mut linker_funcs);

        let placeholder_type_name: TokenStream =
            syn::parse_str(&format!("ExtraInterface_{interface_name}")).unwrap();
        let placeholder_type_declaration =
            quote! { type #placeholder_type_name: extra_interfaces::#interface_name; };
        extra_interface_placeholder_types.push(placeholder_type_declaration);

        let getter_declaration: TokenStream = syn::parse_str(&format!(
            "fn {getter_name}(&self) -> &Self::{placeholder_type_name};"
        ))
        .unwrap();
        extra_interface_getters.push(getter_declaration);
    }

    let trait_bounds = quote! { U: Imports };

    build_code(
        quote! {
            pub mod imports {
                pub trait Imports {
                    #( #extra_interface_placeholder_types )*

                    fn get_memory(&self) -> Option<wasmtime::Memory>;
                    fn set_memory(&mut self, memory: wasmtime::Memory);

                    fn get_free(&self) -> Option<super::FreeFunc>;
                    fn set_free(&mut self, free: super::FreeFunc);

                    fn get_alloc(&self) -> Option<super::AllocFunc>;
                    fn set_alloc(&mut self, alloc: super::AllocFunc);

                    fn get_big_call_ptr(&self) -> super::Ptr;

                    #( #extra_interface_getters )*
                    #( #main_trait_methods )*
                }

                pub mod extra_interfaces {
                    #( #extra_interface_traits )*
                }

                pub fn add_to_linker<#trait_bounds>(linker: &mut wasmtime::Linker<U>) {
                    use extra_interfaces::*;

                    fn get_memory<#trait_bounds>(caller: &mut wasmtime::Caller<U>) -> wasmtime::Memory {
                        let Some(wasmtime::Extern::Memory(memory)) = caller.get_export("memory") else {
                            panic!("Failed to get memory export")
                        };
                        memory
                    }

                    fn read_big_call_args<#trait_bounds>(
                        caller: &mut wasmtime::Caller<U>,
                    ) -> &'static std::thread::LocalKey<std::cell::RefCell<Vec<u8>>> {
                        thread_local! {
                            static ARGS: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(vec![0u8; super::__shared::BYTES_TO_STORE_U64_32_TIMES]);
                        }

                        let big_call_ptr = caller.data().get_big_call_ptr();

                        ARGS.with_borrow_mut(|args| {
                            get_memory(caller)
                                .read(caller, big_call_ptr as usize, args)
                                .unwrap();
                        });
                        &ARGS
                    }

                    fn get_memory_and<
                        #trait_bounds,
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

                    fn read_to_buffer<#trait_bounds>(
                        mut caller: &mut wasmtime::Caller<U>,
                        fat_ptr: super::__shared::FatPtr,
                        call_free: bool,
                    ) -> wasmtime::Result<Vec<u8>> {
                        let memory = caller.data().get_memory();
                        let free = caller.data().get_free();
                        let (memory, free) = if free.is_some() {
                            (memory.unwrap(), free.unwrap())
                        } else {
                            get_memory_and(caller, "__custom_free")
                        };

                        let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
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

                    fn read_from_guest<#trait_bounds, T: serde::de::DeserializeOwned>(
                        caller: &mut wasmtime::Caller<U>,
                        fat_ptr: super::__shared::FatPtr,
                    ) -> wasmtime::Result<T> {
                        let buffer = read_to_buffer(caller, fat_ptr, true)?;
                        Ok(bincode::deserialize(&buffer)?)
                    }

                    fn read_string_ref_from_guest<#trait_bounds>(
                        caller: &mut wasmtime::Caller<U>,
                        fat_ptr: super::__shared::FatPtr,
                    ) -> wasmtime::Result<String> {
                        let buffer = read_to_buffer(caller, fat_ptr, false)?;
                        Ok(String::from_utf8(buffer)?)
                    }

                    fn clone_bytes_to_guest<#trait_bounds>(
                        mut caller: &mut wasmtime::Caller<U>,
                        bytes: &[u8],
                    ) -> wasmtime::Result<super::__shared::FatPtr> {
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

                        Ok(super::__shared::to_fat_ptr(ptr, size))
                    }

                    fn send_to_guest<#trait_bounds, T: ?Sized + serde::Serialize>(
                        caller: &mut wasmtime::Caller<U>,
                        data: &T,
                    ) -> wasmtime::Result<super::__shared::FatPtr> {
                        let bytes = bincode::serialize(&data)?;
                        clone_bytes_to_guest(caller, &bytes)
                    }

                    fn send_string_to_guest<#trait_bounds>(
                        caller: &mut wasmtime::Caller<U>,
                        string: String,
                    ) -> wasmtime::Result<super::__shared::FatPtr> {
                        clone_bytes_to_guest(caller, string.as_bytes())
                    }

                    #( #main_linker_funcs )*

                    #( #extra_interface_linker_funcs )*
                }
            }
        },
        {
            let mut interface_paths = vec![main_interface_file];
            interface_paths.append(&mut extra_interface_files);
            interface_paths
        },
    )
}

fn gen_import_internals(
    imports: Vec<parser::AnyFunc>,
    extra_interface_name: TokenStream,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut trait_methods = vec![];
    let mut linker_funcs = vec![];

    for import in imports {
        match import {
            parser::AnyFunc::Normal(n) => handle_normal_func(
                n,
                &extra_interface_name,
                |InternalFuncImpl {
                     name,
                     param_decls,
                     param_names,
                     params_deserialization,
                     ret,
                     ret_serialization,
                     extra_interface_name,
                 }: InternalFuncImpl| {
                    linker_funcs.push(quote! {
                        linker
                            .func_wrap(
                                "__custom_imports",
                                stringify!(#name),
                                #[allow(unused_mut)]
                                |mut caller: wasmtime::Caller<U>, #( #param_decls, )*| #ret {
                                    #[allow(clippy::unnecessary_cast)]
                                    {
                                        #params_deserialization

                                        #[allow(unused_variables, clippy::let_unit_value)]
                                        let call_return = caller.data()#extra_interface_name.#name( #( #param_names, )* );
                                        #ret_serialization
                                    }
                                },
                            )
                            .unwrap();
                    });
                },
                |name| name,
                &mut trait_methods,
            ),
            parser::AnyFunc::MultiFunc(m) => handle_multi_func(
                m,
                &extra_interface_name,
                &mut trait_methods,
                &mut linker_funcs,
            ),
        }
    }

    fn handle_normal_func(
        parser::Func {
            name,
            params,
            ret,
            big_call,
        }: parser::Func,
        extra_interface_name: &TokenStream,
        handle_result: impl FnOnce(InternalFuncImpl),
        trait_method_name: impl FnOnce(String) -> String,
        trait_methods: &mut Vec<TokenStream>,
    ) {
        let name: Ident = syn::parse_str(&trait_method_name(name)).unwrap();

        let mut param_trait_decls = vec![];
        let mut param_internal_decls = vec![];
        let mut param_names = vec![];
        let mut param_deserializations = vec![];

        for (idx, p) in params.into_iter().enumerate() {
            // TODO: make proper panic messages
            let name: Ident = syn::parse_str(&p.name).expect("t");
            let pub_type = value_type_to_rust_as_syn_type(&p.param_type, true);
            let param_internal_type = value_type_to_repr_as_token_stream(&p.param_type);

            let deserialization = {
                let deserialization = if big_call {
                    let arg_bottom = idx * 8;
                    let arg_top = (idx + 1) * 8;
                    let current_arg_range = quote! { #arg_bottom..#arg_top };
                    quote! {
                        <#param_internal_type as super::__shared::NumAsU64Arr>::from_bytes(big_call_args[#current_arg_range].try_into().unwrap())
                    }
                } else {
                    quote! { #name }
                };

                match p.param_type.kind {
                    ValueKind::Native => quote! { #deserialization as #pub_type },
                    ValueKind::FatPtr => {
                        quote! { read_from_guest(&mut caller, #deserialization).unwrap() }
                    }
                    ValueKind::Bool => {
                        quote! { #deserialization == 1 }
                    }
                    ValueKind::String => {
                        quote! { read_string_ref_from_guest(&mut caller, #deserialization).unwrap() }
                    }
                }
            };

            param_trait_decls.push(quote! {
                #name: #pub_type
            });

            if !big_call {
                param_internal_decls.push(quote! {
                    #name: #param_internal_type
                });
            }
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
                    quote! { send_to_guest(&mut caller, &call_return).unwrap() }
                }
                ValueKind::Bool => quote! { call_return as i32 },
                ValueKind::String => {
                    quote! { send_string_to_guest(&mut caller, call_return).unwrap() }
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

        trait_methods.push(quote! {
            fn #name(&self, #( #param_trait_decls, )* ) #pub_ret;
        });

        let all_params_deserialization = if big_call {
            quote! {
                // TODO: add check if there are more than 0 params? or does compiler optimize it anyway?
                let ( #( #param_names, )* ) = read_big_call_args(&mut caller).with_borrow(|big_call_args| {
                    ( #(
                        #param_deserializations,
                    )* )
                });
            }
        } else {
            quote! {
                #(
                    let #param_names = #param_deserializations;
                )*
            }
        };

        handle_result(InternalFuncImpl {
            name,
            param_decls: param_internal_decls,
            param_names,
            params_deserialization: all_params_deserialization,
            ret: internal_ret,
            ret_serialization,
            extra_interface_name: extra_interface_name.clone(),
        });
    }

    fn handle_multi_func(
        parser::MultiFunc { name, funcs }: parser::MultiFunc,
        extra_interface_name: &TokenStream,
        trait_methods: &mut Vec<TokenStream>,
        linker_funcs: &mut Vec<TokenStream>,
    ) {
        let mut func_impls = vec![];

        for f in funcs {
            handle_normal_func(
                f,
                extra_interface_name,
                |func_impl| {
                    func_impls.push(func_impl);
                },
                |func_name| format!("{name}_{func_name}"),
                trait_methods,
            );
        }

        let func_match_arms = func_impls
            .into_iter()
            .enumerate()
            .map(
                |(
                    index,
                    InternalFuncImpl {
                        name,
                        param_names,
                        params_deserialization,
                        ret_serialization,
                        ret: _,
                        param_decls: _,
                        extra_interface_name,
                    },
                )| {
                    let index = index as u32;
                    let ret_serialization = if ret_serialization.is_empty() {
                        quote! { 0 }
                    } else {
                        ret_serialization
                    };
                    quote! {
                        #index => {
                            #params_deserialization

                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data()#extra_interface_name.#name( #( #param_names, )* );
                            #ret_serialization
                        }
                    }
                },
            )
            .collect::<Vec<_>>();

        let name: Ident = syn::parse_str(&name).unwrap();
        linker_funcs.push(quote! {
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(#name),
                    #[allow(unused_mut)]

                    // returns any value (fat ptr, bool, etc.) fitting into u64
                    |mut caller: wasmtime::Caller<U>, func_index: u32| -> u64 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            match func_index {
                                #( #func_match_arms )*
                                _ => {
                                    panic!("Unknown multi func index: {func_index} in func: {}", stringify!(#name));
                                }
                            }
                        }
                    },
                )
                .unwrap();
        });
    }

    (trait_methods, linker_funcs)
}
