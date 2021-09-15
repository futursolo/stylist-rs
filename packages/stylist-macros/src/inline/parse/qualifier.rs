use super::{
    super::component_value::{ComponentValue, ComponentValueStream, PreservedToken},
    fragment_spacing, IntoOutputContext,
};
use crate::{output::OutputSelector, spacing_iterator::SpacedIterator};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

#[derive(Debug, Clone, Default)]
pub struct CssBlockQualifier {
    qualifiers: Vec<ComponentValue>,
    errors: Vec<ParseError>,
}

impl Parse for CssBlockQualifier {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let mut component_iter = ComponentValueStream::from(input);
        let mut qualifiers = vec![];
        let mut errors = vec![];
        loop {
            // Consume all tokens till the next '{'-block
            if input.peek(token::Brace) {
                break;
            }
            let next_token = component_iter
                .next()
                .ok_or_else(|| input.error("ScopeQualifier: unexpected end of input"))??;
            let token_errors = next_token.validate_selector_token()?;
            if token_errors.is_empty() {
                qualifiers.push(next_token);
            }
            errors.extend(token_errors);
        }
        Ok(Self { qualifiers, errors })
    }
}

impl ToTokens for CssBlockQualifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for q in self.qualifiers.iter() {
            q.to_tokens(tokens);
        }
    }
}

impl CssBlockQualifier {
    pub fn into_output(self, ctx: &mut IntoOutputContext) -> Vec<OutputSelector> {
        ctx.extend_errors(self.errors);

        fn is_not_comma(q: &ComponentValue) -> bool {
            !matches!(q, ComponentValue::Token(PreservedToken::Punct(ref p)) if p.as_char() == ',')
        }

        self.qualifiers
            .into_iter()
            .peekable()
            .batching(|it| {
                // Return if no items left
                it.peek()?;
                // Take until the next comma
                let selector_parts = it
                    .peeking_take_while(is_not_comma)
                    // reify the individual parts
                    .flat_map(|p| p.to_output_fragments())
                    // space them correctly
                    .spaced_with(fragment_spacing)
                    .collect();
                let selector = OutputSelector {
                    selectors: selector_parts,
                };
                it.next(); // Consume the comma
                Some(selector)
            })
            .collect()
    }
}
