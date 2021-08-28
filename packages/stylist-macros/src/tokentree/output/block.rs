use super::Reify;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub struct OutputQualifiedRule {
    pub qualifier: TokenStream,
    pub attributes: Vec<TokenStream>,
}

impl Reify for OutputQualifiedRule {
    fn reify(self) -> TokenStream {
        let ident_attributes = Ident::new("attributes", Span::mixed_site());
        let Self {
            qualifier,
            attributes,
            ..
        } = self;

        quote! {
            ::stylist::ast::Block {
                condition: #qualifier,
                style_attributes: {
                    let mut #ident_attributes = ::std::vec::Vec::<::stylist::ast::StyleAttribute>::new();
                    #( #ident_attributes.push(#attributes); )*
                    #ident_attributes.into()
                },
            }
        }
    }
}
