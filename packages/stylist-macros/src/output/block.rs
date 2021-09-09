use super::{ContextRecorder, IntoCowVecTokens, OutputBlockContent, OutputQualifier, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputQualifiedRule {
    pub qualifier: OutputQualifier,
    pub content: Vec<OutputBlockContent>,
}

impl Reify for OutputQualifiedRule {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let qualifier = self.qualifier.into_token_stream(ctx);
        let content = self.content.into_cow_vec_tokens(ctx);

        quote! {
            ::stylist::ast::Block {
                condition: #qualifier,
                content: #content,
            }
        }
    }
}
