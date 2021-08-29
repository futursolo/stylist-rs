use super::Reify;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub struct OutputSheet {
    pub contents: Vec<TokenStream>,
}

impl Reify for OutputSheet {
    fn into_token_stream(self) -> TokenStream {
        let ident_scopes = Ident::new("scopes", Span::mixed_site());
        let Self { contents } = self;

        quote! {
            {
                let #ident_scopes: ::std::vec::Vec::<::stylist::ast::ScopeContent> = ::std::vec![
                    #( #contents, )*
                ];
                ::stylist::ast::Sheet::from(#ident_scopes)
            }
        }
    }
}
