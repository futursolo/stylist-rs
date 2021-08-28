use super::{OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub enum OutputRuleContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
}

impl Reify for OutputRuleContent {
    fn reify(self) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.reify();
                quote! { ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#block_tokens)) }
            }
            Self::Block(block) => {
                let block_tokens = block.reify();
                quote! { ::stylist::ast::RuleContent::Block(#block_tokens) }
            }
        }
    }
}
