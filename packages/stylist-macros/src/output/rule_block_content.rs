use super::{OutputAttribute, OutputBlock, OutputRule, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum OutputRuleBlockContent {
    Rule(Box<OutputRule>),
    Block(Box<OutputBlock>),
    StyleAttr(OutputAttribute),
}

impl Reify for OutputRuleBlockContent {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        match self {
            Self::Rule(m) => {
                let mut inner_ctx = ReifyContext::new();
                let tokens = m.into_token_stream(&mut inner_ctx);
                ctx.uses_nested(&inner_ctx);

                let bowed_tokens = if inner_ctx.is_const() {
                    quote! {
                        ::stylist::ast::Bow::Borrowed({
                            const RULE: ::stylist::ast::Rule = #tokens;
                            &RULE
                        })
                    }
                } else {
                    quote! {
                        ::stylist::ast::Bow::Boxed(
                            ::std::boxed::Box::new(#tokens)
                        )
                    }
                };

                quote! { ::stylist::ast::RuleBlockContent::Rule(#bowed_tokens) }
            }
            Self::Block(m) => {
                let mut inner_ctx = ReifyContext::new();
                let tokens = m.into_token_stream(&mut inner_ctx);
                ctx.uses_nested(&inner_ctx);

                let bowed_tokens = if inner_ctx.is_const() {
                    quote! {
                        ::stylist::ast::Bow::Borrowed({
                            const BLOCK: ::stylist::ast::Block = #tokens;
                            &BLOCK
                        })
                    }
                } else {
                    quote! {
                        ::stylist::ast::Bow::Boxed(
                            ::std::boxed::Box::new(#tokens)
                        )
                    }
                };

                quote! { ::stylist::ast::RuleBlockContent::Block(#bowed_tokens) }
            }
            Self::StyleAttr(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::RuleBlockContent::StyleAttr(#tokens) }
            }
        }
    }
}
