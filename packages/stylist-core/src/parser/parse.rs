use super::{ParseResult, ParseStream};

pub trait Parse<'a>: Sized {
    fn parse(input: ParseStream<'a>) -> ParseResult<(Self, ParseStream<'a>)>;
}
