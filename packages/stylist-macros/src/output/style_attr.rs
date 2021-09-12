use super::{fragment_coalesce, IntoCowVecTokens, OutputFragment, Reify, ReifyContext};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct OutputAttribute {
    pub key: OutputFragment,
    pub values: Vec<OutputFragment>,
}

impl Reify for OutputAttribute {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        let key = self.key.into_token_stream(ctx);
        let value_parts = self
            .values
            .into_iter()
            .coalesce(fragment_coalesce)
            .into_cow_vec_tokens(ctx);

        quote! {
            ::stylist::ast::StyleAttribute {
                key: #key,
                value: {
                    #value_parts
                },
            }
        }
    }
}
