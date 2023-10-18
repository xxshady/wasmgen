use proc_macro::TokenStream;

#[proc_macro]
pub fn host(input: TokenStream) -> TokenStream {
    wasm_codegen_impl::host(input.into()).into()
}

#[proc_macro]
pub fn guest(input: TokenStream) -> TokenStream {
    wasm_codegen_impl::guest(input.into()).into()
}
