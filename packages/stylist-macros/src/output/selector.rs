use super::{fragment_coalesce, ContextRecorder, IntoCowVecTokens, OutputFragment, Reify};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct OutputQualifier {
    pub selectors: Vec<OutputSelector>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputQualifier {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let Self {
            selectors, errors, ..
        } = self;

        let selectors = selectors.into_iter().into_cow_vec_tokens(ctx);
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        quote! {
            {
                #( #errors )*
                #selectors
            }
        }
    }
}
