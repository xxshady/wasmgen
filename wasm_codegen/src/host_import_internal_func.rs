use proc_macro2::{Ident, TokenStream};

pub(crate) struct InternalFuncImpl {
    pub(crate) name: Ident,
    pub(crate) param_decls: Vec<TokenStream>,
    pub(crate) param_names: Vec<Ident>,
    pub(crate) params_deserialization: TokenStream,
    pub(crate) ret: TokenStream,
    pub(crate) ret_serialization: TokenStream,
}
