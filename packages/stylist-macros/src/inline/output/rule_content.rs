use super::{MaybeStatic, OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub enum OutputRuleContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}

impl Reify for OutputRuleContent {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        match self {
            Self::AtRule(rule) => rule.into_token_stream().flat_map(|block_tokens| {
                MaybeStatic::statick(quote! {
                    ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#block_tokens))
                })
            }),
            Self::Block(block) => block.into_token_stream().flat_map(|block_tokens| {
                MaybeStatic::statick(quote! {
                    ::stylist::ast::RuleContent::Block(#block_tokens)
                })
            }),
        }
    }
}
