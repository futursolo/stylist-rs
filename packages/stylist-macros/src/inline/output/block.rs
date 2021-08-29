use super::{MaybeStatic, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputQualifiedRule {
    pub qualifier: MaybeStatic<TokenStream>,
    pub attributes: MaybeStatic<Vec<TokenStream>>,
}

impl Reify for OutputQualifiedRule {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        let Self {
            qualifier,
            attributes,
            ..
        } = self;
        let (qualifier, qualifier_context) = qualifier.into_value();
        let (attributes, attributes_context) = attributes
            .into_cow_vec_tokens(quote! {::stylist::ast::StyleAttribute})
            .into_value();

        MaybeStatic::in_context(
            qualifier_context & attributes_context,
            quote! {
                ::stylist::ast::Block {
                    condition: #qualifier,
                    style_attributes: #attributes,
                }
            },
        )
    }
}
