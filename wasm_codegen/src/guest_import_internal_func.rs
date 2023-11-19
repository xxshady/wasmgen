use proc_macro2::{Ident, TokenStream};

pub(crate) struct InternalFuncImpl {
    pub(crate) name: Ident,
    pub(crate) internal_name: Ident,
    pub(crate) param_decls: Vec<TokenStream>,
    pub(crate) param_names: TokenStream,
    pub(crate) internal_param_decls: Vec<TokenStream>,
    pub(crate) ret: TokenStream,
    pub(crate) internal_ret: TokenStream,
    pub(crate) ret_deserialization: TokenStream,
    pub(crate) big_call_args_init: TokenStream,
    pub(crate) params_serialization: TokenStream,
    pub(crate) big_call_args_tail: TokenStream,
}
