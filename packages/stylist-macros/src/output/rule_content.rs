use super::{ContextRecorder, OutputBlock, OutputRule, OutputRuleBlock, Reify};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleContent {
    Rule(OutputRule),
    Block(OutputBlock),
    // String(String),
    RuleBlock(OutputRuleBlock),
}

impl Reify for OutputRuleContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::Rule(rule) => {
                let tokens = rule.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#tokens)) }
            }
            Self::Block(block) => {
                let tokens = block.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Block(#tokens) }
            }
            // Self::String(ref s) => {
            //     let s = Literal::string(s);
            //     quote! { ::stylist::ast::RuleContent::String(#s.into()) }
            // }
            Self::RuleBlock(m) => {
                let tokens = m.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::RuleBlock(#tokens) }
            }
        }
    }
}
