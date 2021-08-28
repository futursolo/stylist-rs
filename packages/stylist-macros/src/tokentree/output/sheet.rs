use super::Reify;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub struct OutputSheet {
    pub contents: Vec<TokenStream>,
}

impl Reify for OutputSheet {
    fn reify(self) -> TokenStream {
        let ident_scopes = Ident::new("scopes", Span::mixed_site());
        let Self { contents } = self;

        quote! {
            ::stylist::ast::Sheet::from(
                {
                    let mut #ident_scopes = ::std::vec::Vec::<::stylist::ast::ScopeContent>::new();
                    #( #ident_scopes.push( #contents ); )*
                    #ident_scopes
                }
            )
        }
    }
}
