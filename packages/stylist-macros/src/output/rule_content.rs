use super::{ContextRecorder, OutputBlock, OutputRule, Reify};
use proc_macro2::{Literal, TokenStream};
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleContent {
    Rule(OutputRule),
    Block(OutputBlock),
    String(String),
}

impl Reify for OutputRuleContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::Rule(rule) => {
                let block_tokens = rule.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#block_tokens)) }
            }
            Self::Block(block) => {
                let block_tokens = block.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Block(#block_tokens) }
            }
            Self::String(ref s) => {
                let s = Literal::string(s);
                quote! { ::stylist::ast::RuleContent::String(#s.into()) }
            }
        }
    }
}
