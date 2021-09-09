use super::{ContextRecorder, OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;
// use syn::Error as ParseError;

#[derive(Debug)]
pub enum OutputScopeContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
    // Err(ParseError),
}

impl Reify for OutputScopeContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.into_token_stream(ctx);
                quote! { ::stylist::ast::ScopeContent::Rule(#block_tokens) }
            }
            Self::Block(block) => {
                let block_tokens = block.into_token_stream(ctx);
                quote! { ::stylist::ast::ScopeContent::Block(#block_tokens) }
            } // Self::Err(err) => err.into_token_stream(ctx),
        }
    }
}
