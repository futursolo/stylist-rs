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

#[derive(Debug, PartialEq)]
enum CommentState {
    Reading,
    Ending,
    Ended,
}

impl Tokenize<InputStr> for Comment {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        if !input.starts_with("/*") {
            return Err(TokenizeError::NotTokenized(input));
        }

        let mut state = CommentState::Reading;
        let mut len = 2;

        for c in input.chars().skip(2) {
            len += 1;

            if state == CommentState::Ending && c == '/' {
                state = CommentState::Ended;
                break;
            } else if state == CommentState::Reading && c == '*' {
                state = CommentState::Ending;
                continue;
            }

            state = CommentState::Reading;
        }

        if state != CommentState::Ended {
            let (_inner, location, _rest) = input.split_at(2);

            return Err(TokenizeError::Terminal(ParseError::new(
                "cannot find the end of this comment, expected '*/'",
                location,
            )));
        }

        let (inner, location, rest) = input.split_at(len);
        Ok((TokenTree::Comment(Self { inner, location }).into(), rest))
    }
}
