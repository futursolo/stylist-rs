use super::Reify;
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputQualifiedRule {
    pub qualifier: TokenStream,
    pub attributes: Vec<TokenStream>,
}

impl Reify for OutputQualifiedRule {
    fn into_token_stream(self) -> TokenStream {
        let Self {
            qualifier,
            attributes,
            ..
        } = self;

        quote! {
            ::stylist::ast::Block {
                condition: #qualifier,
                style_attributes: ::std::vec![
                    #( #attributes, )*
                ].into(),
            }
        }
    }
}
