use super::{fragment_coalesce, ContextRecorder, IntoCowVecTokens, OutputFragment, Reify};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
// use syn::parse::Error as ParseError;

#[derive(Debug)]
pub struct OutputAttribute {
    pub key: OutputFragment,
    pub values: Vec<OutputFragment>,
    // pub errors: Vec<ParseError>,
}

impl Reify for OutputAttribute {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        // let errors = self.errors.into_iter().map(|e| e.into_compile_error());

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
                    // #( #errors )*
                    #value_parts
                },
            }
        }
    }
}
