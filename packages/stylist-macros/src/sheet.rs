use proc_macro2::{TokenStream, TokenTree};
pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    if let Some(TokenTree::Literal(_)) = input.clone().into_iter().next() {
        crate::stringly::macro_fn(input)
    } else {
        crate::tokentree::macro_fn(input)
    }
}
