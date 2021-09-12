use super::{ContextRecorder, OutputAttribute, OutputBlock, OutputRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleBlockContent {
    Rule(Box<OutputRule>),
    Block(Box<OutputBlock>),
    StyleAttr(OutputAttribute),
}

impl Reify for OutputRuleBlockContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::Rule(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::RuleBlockContent::Rule(::std::boxed::Box::new(#tokens)) }
            }
            Self::Block(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::RuleBlockContent::Block(::std::boxed::Box::new(#tokens)) }
            }
            Self::StyleAttr(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::RuleBlockContent::StyleAttr(#tokens) }
            }
        }
    }
}
