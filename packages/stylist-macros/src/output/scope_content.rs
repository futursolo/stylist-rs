use super::{OutputBlock, OutputRule, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputScopeContent {
    Rule(OutputRule),
    Block(OutputBlock),
}

impl Reify for OutputScopeContent {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        match self {
            Self::Rule(rule) => {
                let tokens = rule.into_token_stream(ctx);
                quote! { ::stylist::ast::ScopeContent::Rule(#tokens) }
            }
            Self::Block(block) => {
                let tokens = block.into_token_stream(ctx);
                quote! { ::stylist::ast::ScopeContent::Block(#tokens) }
            }
        }
    }
}
