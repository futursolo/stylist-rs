use arcstr::Substr;

use super::{
    InputStr, Location, Token, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult,
};
use crate::parser::ParseError;

#[derive(Debug, Clone)]
pub struct Comment {
    inner: Substr,
    location: Location,
}

impl Token for Comment {
    fn as_str(&self) -> &str {
        &self.inner
    }
    fn location(&self) -> &Location {
        &self.location
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Tokenize<InputStr> for Comment {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        if !input.starts_with("/*") {
            return Err(TokenizeError::NotTokenized(input));
        }

        let len = input.find("*/").ok_or_else(|| {
            let (_inner, location, _rest) = input.clone().split_at(2);
            ParseError::new(
                "cannot find the end of this comment, expected '*/'",
                location,
            )
        })?;

        let (inner, location, rest) = input.split_at(len + 2);
        Ok((TokenTree::Comment(Self { inner, location }).into(), rest))
    }
}
