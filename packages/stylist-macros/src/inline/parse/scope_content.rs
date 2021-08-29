use super::{
    super::component_value::{ComponentValue, ComponentValueStream, PreservedToken},
    CssAtRule, CssAttribute, CssQualifiedRule,
};
use itertools::Itertools;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug)]
pub enum CssScopeContent {
    Attribute(CssAttribute),
    AtRule(CssAtRule),
    Nested(CssQualifiedRule),
}

impl Parse for CssScopeContent {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        // Fork the stream. Peeking a component value might still consume tokens from the stream!
        let forked_input = input.fork();
        let mut component_peek = ComponentValueStream::from(&forked_input).multipeek();
        let next_input = component_peek
            .peek()
            .cloned()
            .ok_or_else(|| forked_input.error("Scope: unexpected end of input"))??;
        // Steps roughly follow Css-Syntax-Level 3, §5.4.4: Consume a list of declarations
        // Allows for directly nested attributes though
        // At-rule first
        if let ComponentValue::Token(PreservedToken::Punct(ref p)) = next_input {
            if p.as_char() == '@' {
                let atrule = input.parse()?;
                return Ok(Self::AtRule(atrule));
            }
        }
        // If it starts with an <ident-token>, it might be an attribute.
        if next_input.maybe_to_attribute_name().is_some() {
            // peek another token to see if it's colon
            let maybe_colon = component_peek.peek();
            if let Some(Ok(ComponentValue::Token(PreservedToken::Punct(p)))) = maybe_colon {
                if p.as_char() == ':' {
                    let attr = input.parse()?;
                    return Ok(Self::Attribute(attr));
                }
            }
        }
        // It isn't. All that's left now is that it's a qualified rule.
        let rule = input.parse()?;
        Ok(Self::Nested(rule))
    }
}

impl CssScopeContent {
    // §5.4.1: Consume a list of rules
    pub fn consume_list_of_rules(input: &ParseBuffer) -> ParseResult<Vec<Self>> {
        let mut contents = Vec::new();
        while !input.is_empty() {
            // Not handled: <CDO-token> <CDC-token>
            contents.push(input.parse()?);
        }
        Ok(contents)
    }
}
