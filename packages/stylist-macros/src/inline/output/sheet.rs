use super::Reify;
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputSheet {
    pub contents: Vec<TokenStream>,
}

impl Reify for OutputSheet {
    fn into_token_stream(self) -> TokenStream {
        let Self { contents } = self;

        quote! {
            {
                use ::std::convert::{From, Into};
                ::stylist::ast::Sheet::from(
                    ::std::vec![
                        #( ::stylist::ast::ScopeContent::from(#contents), )*
                    ]
                )
            }
        }
    }
}
