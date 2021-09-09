use super::{fragment_coalesce, ContextRecorder, IntoCowVecTokens, OutputFragment, Reify};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone)]
pub struct OutputSelector {
    pub selectors: Vec<OutputFragment>,
}

impl Reify for OutputSelector {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let parts = self
            .selectors
            .into_iter()
            // optimize successive (string) literals
            .coalesce(fragment_coalesce)
            .into_cow_vec_tokens(ctx);
        quote! {
            ::stylist::ast::Selector {
                fragments: #parts,
            }
        }
    }
}
