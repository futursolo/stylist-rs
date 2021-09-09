use super::{
    fragment_coalesce, ContextRecorder, IntoCowVecTokens, OutputFragment, OutputRuleContent, Reify,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

pub struct OutputAtRule {
    pub prelude: Vec<OutputFragment>,
    pub contents: Vec<OutputRuleContent>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAtRule {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let condition = self
            .prelude
            .into_iter()
            .coalesce(fragment_coalesce)
            .into_cow_vec_tokens(ctx);
        let content = self.contents.into_cow_vec_tokens(ctx);
        let errors = self.errors.into_iter().map(|e| e.into_compile_error());

        quote! {
            ::stylist::ast::Rule {
                condition: {
                    #( #errors )*
                    #condition
                },
                content: #content,
            }
        }
    }
}
