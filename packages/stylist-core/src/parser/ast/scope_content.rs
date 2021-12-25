use crate::parser::{Parse, ParseResult, ParseStream};

#[derive(Debug)]
pub struct ScopeContent {
    // inner: Vec<ScopeContent>,
}

impl<'a> Parse<'a> for ScopeContent {
    fn parse(input: ParseStream<'a>) -> ParseResult<Option<(Self, ParseStream<'a>)>> {
        todo!()
    }
}
