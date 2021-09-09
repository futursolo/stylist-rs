use super::{ContextRecorder, IntoCowVecTokens, OutputAttribute, OutputFragment, Reify};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleBlockContent {
    RuleBlock(Box<OutputRuleBlock>),
    StyleAttr(OutputAttribute),
}

impl Reify for OutputRuleBlockContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::RuleBlock(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::RuleBlockContent::RuleBlock(::std::boxed::Box::new(#tokens)) }
            }
            Self::StyleAttr(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::RuleBlockContent::StyleAttr(#tokens) }
            }
        }
    }
}

#[derive(Debug)]
pub struct OutputRuleBlock {
    pub condition: Vec<OutputFragment>,
    pub content: Vec<OutputRuleBlockContent>,
}

impl Reify for OutputRuleBlock {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let condition = self.condition.into_cow_vec_tokens(ctx);
        let content = self.content.into_cow_vec_tokens(ctx);

        quote! {
            ::stylist::ast::RuleBlock {
                condition: #condition,
                content: #content,
            }
        }
    }
}
