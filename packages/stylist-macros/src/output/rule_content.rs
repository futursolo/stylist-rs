use super::{ContextRecorder, OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::{Literal, TokenStream};
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
    String(String),
    // Err(ParseError),
}

impl Reify for OutputRuleContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
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
            } // Self::Err(err) => err.into_token_stream(ctx),
        }
    }
}
