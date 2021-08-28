use super::{OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub enum OutputScopeContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}

impl Reify for OutputScopeContent {
    fn reify(self) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.reify();
                quote! { ::stylist::ast::ScopeContent::Rule(#block_tokens) }
            }
            Self::Block(block) => {
                let block_tokens = block.reify();
                quote! { ::stylist::ast::ScopeContent::Block(#block_tokens) }
            }
        }
    }
}
