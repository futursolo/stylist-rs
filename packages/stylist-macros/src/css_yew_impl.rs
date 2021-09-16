use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    quote! { ::stylist::css!(#input).with_manager(__stylist_style_manager__.clone()) }
}
