use super::{OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub enum OutputScopeContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}

impl Reify for OutputScopeContent {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.into_token_stream();
                quote! { ::stylist::ast::ScopeContent::Rule(#block_tokens) }
            }
            Self::Block(block) => {
                let block_tokens = block.into_token_stream();
                quote! { ::stylist::ast::ScopeContent::Block(#block_tokens) }
            }
        }
    }
}
