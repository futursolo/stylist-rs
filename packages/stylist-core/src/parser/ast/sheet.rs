use super::ScopeContent;
use crate::parser::{Parse, ParseError, ParseResult, ParseStream};
use crate::tokens::Token;

/// A Css Stylesheet
///
/// https://www.w3.org/TR/css-syntax-3/#css-stylesheets
#[derive(Debug)]
pub struct Sheet {
    inner: Vec<ScopeContent>,
}

impl<'a> Parse<'a> for Sheet {
    fn parse(input: ParseStream<'a>) -> ParseResult<Option<(Self, ParseStream<'a>)>> {
        let mut scopes = Vec::with_capacity(1); // at least 1 scope content is expected.

        let mut input = input;
        while let Some((scope, new_input)) = ScopeContent::parse(input.clone())? {
            scopes.push(scope);

            input = new_input;
        }

        let mut input = input.trim_start();

        if let Some(m) = input.first() {
            return Err(ParseError::unexpected_token(m.location().to_owned()));
        }

        Ok(Some((Sheet { inner: scopes }, input)))
    }
}
