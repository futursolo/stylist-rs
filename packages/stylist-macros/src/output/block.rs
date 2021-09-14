use super::{ContextRecorder, IntoCowVecTokens, OutputAttribute, OutputQualifier, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputQualifiedRule {
    pub qualifier: OutputQualifier,
    pub attributes: Vec<OutputAttribute>,
}

impl Reify for OutputQualifiedRule {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let Self {
            qualifier,
            attributes,
        } = self;
        let qualifier = qualifier.into_token_stream(ctx);
        let attributes =
            attributes.into_cow_vec_tokens(quote! {::stylist::ast::StyleAttribute}, ctx);

        quote! {
            ::stylist::ast::Block {
                condition: #qualifier,
                style_attributes: #attributes,
            }
        }
    }
}
