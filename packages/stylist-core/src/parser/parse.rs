use super::{ParseResult, ParseStream};

pub trait Parse: Sized {
    fn parse(input: ParseStream) -> ParseResult<(Self, ParseStream)>;
}
