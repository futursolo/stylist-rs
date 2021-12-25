use super::Declaration;
use crate::parser::{Parse, ParseResult, ParseStream};

/// A Css Style Rule
///
/// https://www.w3.org/TR/css-syntax-3/#style-rules
#[derive(Debug)]
pub struct StyleRule {
    // inner: Vec<ScopeContent>,
}

impl<'a> Parse<'a> for StyleRule {
    fn parse(input: ParseStream<'a>) -> ParseResult<Option<(Self, ParseStream<'a>)>> {
        // let mut selector

        // let mut declarations = Vec::new();

        // loop {

        // }
        todo!()
    }
}
