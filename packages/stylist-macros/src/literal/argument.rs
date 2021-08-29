use proc_macro2::{Ident, TokenStream};

#[derive(Debug)]
pub(crate) struct Argument {
    pub name: String,
    pub name_token: Ident,
    pub tokens: TokenStream,
}
