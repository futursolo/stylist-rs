use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    quote! { ::stylist::yew::use_style(::stylist::css!(#input)) }
}
