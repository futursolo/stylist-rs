use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct Argument {
    pub name: String,
    pub tokens: TokenStream,
}
