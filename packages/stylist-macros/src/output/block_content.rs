use super::{ContextRecorder, OutputAttribute, OutputRuleBlock, Reify};
use proc_macro2::TokenStream;
use quote::quote;
// use syn::Error as ParseError;

pub enum OutputBlockContent {
    RuleBlock(OutputRuleBlock),
    StyleAttr(OutputAttribute),
    // Err(ParseError),
}

impl Reify for OutputBlockContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::RuleBlock(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::BlockContent::RuleBlock(#tokens) }
            }
            Self::StyleAttr(m) => {
                let tokens = m.into_token_stream(ctx);

                quote! { ::stylist::ast::BlockContent::StyleAttr(#tokens) }
            } // Self::Err(err) => err.into_token_stream(ctx),
        }
    }
}
