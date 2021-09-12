use super::{ContextRecorder, OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::Error as ParseError;

pub enum OutputRuleContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
    String(String),
    Err(ParseError),
}

impl Reify for OutputRuleContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let mut inner = ContextRecorder::new();
                let block_tokens = rule.into_token_stream(&mut inner);

                let bowed_block = if inner.is_const() {
                    let const_ident = Ident::new("block", Span::mixed_site());
                    quote! {
                        ::stylist::macros::Bow::Borrowed({
                            const #const_ident: ::stylist::ast::Rule = #block_tokens;
                            &#const_ident
                        })
                    }
                } else {
                    ctx.uses_static(); // Box::new
                    ctx.uses_nested(&inner); // #block_tokens
                    quote! {
                        ::stylist::macros::Bow::Boxed(
                            ::std::boxed::Box::new(#block_tokens)
                        )
                    }
                };
                quote! {
                    ::stylist::ast::RuleContent::Rule(#bowed_block)
                }
            }
            Self::Block(block) => {
                let block_tokens = block.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Block(#block_tokens) }
            }
            Self::String(ref s) => {
                let s = Literal::string(s);
                quote! {
                    ::stylist::ast::RuleContent::String(
                        ::std::borrow::Cow::<str>::Borrowed(#s)
                    )
                }
            }
            Self::Err(err) => err.into_token_stream(ctx),
        }
    }
}
