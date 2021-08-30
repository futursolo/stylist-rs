use super::{
    super::component_value::{ComponentValue, PreservedToken},
    fragment_coalesce, fragment_spacing, ContextRecorder, IntoCowVecTokens, Reify,
};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

struct OutputSelector {
    selectors: Vec<ComponentValue>,
}

impl Reify for OutputSelector {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let parts = self
            .selectors
            .into_iter()
            // reify the individual parts
            .flat_map(|p| p.to_output_fragments())
            // space them correctly
            .spaced_with(fragment_spacing)
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
    pub selectors: Vec<ComponentValue>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputQualifier {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        fn is_not_comma(q: &ComponentValue) -> bool {
            !matches!(q, ComponentValue::Token(PreservedToken::Punct(ref p)) if p.as_char() == ',')
        }

        let Self {
            selectors, errors, ..
        } = self;

        let selectors = selectors
            .into_iter()
            .peekable()
            .batching(|it| {
                // Return if no items left
                it.peek()?;
                // Take until the next comma
                let selector_parts = it.peeking_take_while(is_not_comma);
                let selector = OutputSelector {
                    selectors: selector_parts.collect(),
                };
                it.next(); // Consume the comma
                Some(selector)
            })
            .into_cow_vec_tokens(ctx);
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        quote! {
            {
                #( #errors )*
                #selectors
            }
        }
    }
}
