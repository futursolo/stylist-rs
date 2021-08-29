use super::{
    super::component_value::{ComponentValue, PreservedToken},
    fragment_coalesce, fragment_spacing, Reify,
};
use crate::spacing_iterator::SpacedIterator;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

#[derive(Clone)]
pub struct OutputQualifier {
    pub selectors: Vec<ComponentValue>,
    pub errors: Vec<ParseError>,
}

impl Reify for OutputQualifier {
    fn into_token_stream(self) -> TokenStream {
        fn is_not_comma(q: &&ComponentValue) -> bool {
            !matches!(q, ComponentValue::Token(PreservedToken::Punct(ref p)) if p.as_char() == ',')
        }
        // Reify the expression of a Selector from the expressions of its fragments
        fn reify_selector<'c>(
            selector_parts: impl Iterator<Item = &'c ComponentValue>,
        ) -> TokenStream {
            let parts = selector_parts
                .flat_map(|p| p.reify_parts())
                .spaced_with(fragment_spacing)
                .coalesce(fragment_coalesce)
                .map(|e| e.into_token_stream());
            quote! {
                {
                    ::stylist::ast::Selector::from(
                        ::std::vec![
                            #( #parts, )*
                        ]
                    )
                }
            }
        }

        let Self {
            selectors, errors, ..
        } = self;

        let selectors = selectors.iter().peekable().batching(|it| {
            // Return if no items left
            it.peek()?;
            // Take until the next comma
            let selector_parts = it.peeking_take_while(is_not_comma);
            let selector = reify_selector(selector_parts);
            it.next(); // Consume the comma
            Some(selector)
        });
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        quote! {
            ::std::vec![
                #( #errors, )*
                #( #selectors, )*
            ].into()
        }
    }
}
