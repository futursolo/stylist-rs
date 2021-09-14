use proc_macro2::{Ident, TokenStream};

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub name_token: Ident,
    pub tokens: TokenStream,
}
