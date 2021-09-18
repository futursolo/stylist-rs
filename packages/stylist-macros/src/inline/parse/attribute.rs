use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    spanned::Spanned,
    token,
};

use super::{fragment_spacing, IntoOutputContext};
use crate::inline::component_value::{
    ComponentValue, ComponentValueStream, InterpolatedExpression, PreservedToken,
};
use crate::inline::css_ident::CssIdent;
use crate::output::{OutputAttribute, OutputCowString};
use crate::spacing_iterator::SpacedIterator;

#[derive(Debug)]
pub enum CssAttributeName {
    Identifier(CssIdent),
    Expr(InterpolatedExpression),
}

#[derive(Debug)]
pub struct CssAttributeValue {
    values: Vec<ComponentValue>,
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub struct CssAttribute {
    name: CssAttributeName,
    _colon: token::Colon,
    value: CssAttributeValue,
    _terminator: token::Semi,
}

impl Parse for CssAttribute {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let mut component_iter = ComponentValueStream::from(input);
        // Advance the real iterator
        let name = component_iter
            .next()
            .ok_or_else(|| input.error("Attribute: unexpected end of input"))??;
        let name_span = name.span();
        let name = name.maybe_to_attribute_name().ok_or_else(|| {
            ParseError::new(
                name_span,
                "expected an identifier or interpolated expression",
            )
        })?;

        let colon = input.parse()?;
        let value = input.parse()?;
        let terminator = input.parse()?;
        Ok(CssAttribute {
            name,
            _colon: colon,
            value,
            _terminator: terminator,
        })
    }
}

impl Parse for CssAttributeValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let mut component_iter = ComponentValueStream::from(input);
        let mut values = vec![];
        let mut errors = vec![];

        loop {
            // Consume all tokens till the next ';'
            if input.peek(token::Semi) {
                break;
            }
            let next_token = component_iter
                .next()
                .ok_or_else(|| input.error("AttributeValue: unexpected end of input"))??;
            let token_errors = next_token.validate_attribute_token();
            if token_errors.is_empty() {
                values.push(next_token);
            }
            errors.extend(token_errors);
        }
        Ok(Self { values, errors })
    }
}

impl ComponentValue {
    pub(super) fn maybe_to_attribute_name(self) -> Option<CssAttributeName> {
        match self {
            ComponentValue::Token(PreservedToken::Ident(i)) => {
                Some(CssAttributeName::Identifier(i))
            }
            ComponentValue::Expr(expr) => Some(CssAttributeName::Expr(expr)),
            _ => None,
        }
    }
}

impl CssAttribute {
    pub(super) fn into_output(self, ctx: &mut IntoOutputContext) -> OutputAttribute {
        ctx.extend_errors(self.value.errors);

        let values = self
            .value
            .values
            .into_iter()
            .flat_map(|p| p.to_output_fragments())
            .spaced_with(fragment_spacing)
            .collect();
        OutputAttribute {
            key: self.name.into_output(),
            values,
        }
    }
}

impl CssAttributeName {
    fn into_output(self) -> OutputCowString {
        match self {
            Self::Identifier(name) => name.into(),
            Self::Expr(expr) => expr.to_output_fragment(),
        }
        .into_inner()
    }
}
