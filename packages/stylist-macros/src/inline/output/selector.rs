use super::{
    super::component_value::{ComponentValue, PreservedToken},
    fragment_coalesce, fragment_spacing, MaybeStatic, Reify,
};
use crate::{inline::output::maybe_static::ExpressionContext, spacing_iterator::SpacedIterator};
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
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        fn is_not_comma(q: &&ComponentValue) -> bool {
            !matches!(q, ComponentValue::Token(PreservedToken::Punct(ref p)) if p.as_char() == ',')
        }
        // Reify the expression of a Selector from the expressions of its fragments
        fn reify_selector<'c>(
            selector_parts: impl Iterator<Item = &'c ComponentValue>,
        ) -> MaybeStatic<TokenStream> {
            let (parts, parts_context) = selector_parts
                // reify the individual parts
                .flat_map(|p| p.to_output_fragments())
                // space them correctly
                .spaced_with(fragment_spacing)
                // optimize successive (string) literals
                .coalesce(fragment_coalesce)
                .map(|e| e.into_token_stream())
                .collect::<MaybeStatic<_>>()
                .into_cow_vec_tokens(quote! {::stylist::ast::StringFragment})
                .into_value();
            MaybeStatic::in_context(
                parts_context & ExpressionContext::Static,
                quote! {
                    ::stylist::ast::Selector::from(#parts)
                },
            )
        }

        let Self {
            selectors, errors, ..
        } = self;

        let (selectors, selectors_context) = selectors
            .iter()
            .peekable()
            .batching(|it| {
                // Return if no items left
                it.peek()?;
                // Take until the next comma
                let selector_parts = it.peeking_take_while(is_not_comma);
                let selector = reify_selector(selector_parts);
                it.next(); // Consume the comma
                Some(selector)
            })
            .collect::<MaybeStatic<_>>()
            .into_cow_vec_tokens(quote! {::stylist::ast::Selector})
            .into_value();
        let errors = errors.into_iter().map(|e| e.into_compile_error());

        MaybeStatic::in_context(
            // errors are const context
            selectors_context,
            quote! {
                {
                    #( #errors )*
                    #selectors
                }
            },
        )
    }
}
