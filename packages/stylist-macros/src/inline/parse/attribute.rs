use super::super::{
    component_value::{ComponentValue, ComponentValueStream, InjectedExpression, PreservedToken},
    css_ident::CssIdent,
    output::{OutputAttribute, OutputFragment, Reify},
};
use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    spanned::Spanned,
    token,
};

#[derive(Debug)]
pub enum CssAttributeName {
    Identifier(CssIdent),
    InjectedExpr(InjectedExpression),
}

#[derive(Debug)]
pub struct CssAttributeValue {
    values: Vec<ComponentValue>,
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub struct CssAttribute {
    name: CssAttributeName,
    colon: token::Colon,
    value: CssAttributeValue,
    terminator: token::Semi,
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
            colon,
            value,
            terminator,
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
            let token_errors = next_token
                .validate_attribute_token()
                .into_iter()
                .collect::<Vec<_>>();
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
            ComponentValue::Expr(expr) => Some(CssAttributeName::InjectedExpr(expr)),
            _ => None,
        }
    }
}

impl CssAttribute {
    pub(super) fn into_output(self) -> OutputAttribute {
        let key_tokens = self.name.into_output().into_token_stream();
        let values = self.value.values;
        let errors = self.value.errors;

        OutputAttribute {
            key: key_tokens,
            values,
            errors,
        }
    }
}

impl CssAttributeName {
    fn into_output(self) -> OutputFragment {
        match self {
            Self::Identifier(name) => name.to_lit_str().into(),
            Self::InjectedExpr(expr) => expr.to_output_fragment(),
        }
    }
}
