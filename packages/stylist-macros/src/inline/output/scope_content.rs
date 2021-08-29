use super::{MaybeStatic, OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub enum OutputScopeContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}

impl Reify for OutputScopeContent {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        match self {
            Self::AtRule(rule) => rule.into_token_stream().flat_map(|block_tokens| {
                MaybeStatic::statick(quote! {
                    ::stylist::ast::ScopeContent::Rule(#block_tokens)
                })
            }),
            Self::Block(block) => block.into_token_stream().flat_map(|block_tokens| {
                MaybeStatic::statick(quote! {
                    ::stylist::ast::ScopeContent::Block(#block_tokens)
                })
            }),
        }
    }
}
