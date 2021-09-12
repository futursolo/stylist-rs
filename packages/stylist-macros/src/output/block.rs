use super::{IntoCowVecTokens, OutputRuleBlockContent, OutputSelector, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct OutputBlock {
    pub condition: Vec<OutputSelector>,
    pub content: Vec<OutputRuleBlockContent>,
}

impl Reify for OutputBlock {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        let condition = self.condition.into_cow_vec_tokens(ctx);
        let content = self.content.into_cow_vec_tokens(ctx);

        quote! {
            ::stylist::ast::Block {
                condition: #condition,
                content: #content,
            }
        }
    }
}
