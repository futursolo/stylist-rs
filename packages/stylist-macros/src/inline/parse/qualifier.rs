use super::super::{
    component_value::{ComponentValue, ComponentValueStream},
    output::OutputQualifier,
};

use proc_macro2::TokenStream;
use quote::ToTokens;

use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

#[derive(Debug, Clone)]
pub struct CssBlockQualifier {
    qualifiers: Vec<ComponentValue>,
    qualifier_errors: Vec<ParseError>,
}

impl Parse for CssBlockQualifier {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let mut component_iter = ComponentValueStream::from(input);
        let mut qualifiers = vec![];
        let mut qualifier_errors = vec![];
        loop {
            // Consume all tokens till the next '{'-block
            if input.peek(token::Brace) {
                break;
            }
            let next_token = component_iter
                .next()
                .ok_or_else(|| input.error("ScopeQualifier: unexpected end of input"))??;
            qualifier_errors.extend(next_token.validate_selector_token()?);
            qualifiers.push(next_token);
        }
        Ok(Self {
            qualifiers,
            qualifier_errors,
        })
    }
}

impl ToTokens for CssBlockQualifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for q in self.qualifiers.iter() {
            q.to_tokens(tokens);
        }
    }
}

impl Default for CssBlockQualifier {
    fn default() -> Self {
        Self {
            qualifiers: vec![],
            qualifier_errors: vec![],
        }
    }
}

impl CssBlockQualifier {
    pub fn is_empty(&self) -> bool {
        self.qualifiers.is_empty()
    }

    pub fn into_output(self) -> OutputQualifier {
        OutputQualifier {
            selectors: self.qualifiers,
            errors: self.qualifier_errors,
        }
    }
}
