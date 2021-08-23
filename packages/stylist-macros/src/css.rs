use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    let sheet_tokens = crate::sheet::macro_fn(input);

    quote! { ::stylist::IntoStyle::Sheet(#sheet_tokens) }
}
