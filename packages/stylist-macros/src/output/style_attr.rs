use super::{fragment_coalesce, ContextRecorder, IntoCowVecTokens, OutputFragment, Reify};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

pub struct OutputAttribute {
    pub key: OutputFragment,
    pub values: Vec<OutputFragment>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputAttribute {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let Self {
            key,
            values,
            errors,
        } = self;
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        let key = key.into_token_stream(ctx);
        let value_parts = values
            .into_iter()
            .coalesce(fragment_coalesce)
            .into_cow_vec_tokens(ctx);
        quote! {
            ::stylist::ast::StyleAttribute {
                key: #key,
                value: {
                    #( #errors )*
                    #value_parts
                },
            }
        }
    }
}
