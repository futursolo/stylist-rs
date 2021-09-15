use arcstr::Substr;

use super::{
    InputStr, Location, RTokenize, Token, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};

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

impl Comment {
    fn find_comment_len<I>(mut chars: I) -> usize
    where
        I: Iterator<Item = char>,
    {
        if chars.next() != Some('/') {
            return 0;
        }

        if chars.next() != Some('*') {
            return 0;
        }

        let mut len = 2;

        let mut ending = false;
        let mut ended = false;
        for c in chars {
            if ending {
                if c == '/' {
                    len += 1;
                    ended = true;
                    break;
                }
            } else if c == '*' {
                len += 1;
                ending = true;
                continue;
            }

            ending = false;
            len += 1;
        }

        if !ended {
            // Failed to find a */ to terminate the comment.
            // Should be terminal when inplemented.
            return 0;
        }

        len
    }
}

impl Tokenize<InputStr> for Comment {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let chars = input.chars();

        let len = Self::find_comment_len(chars);

        if len > 0 {
            let (inner, location, rest) = input.split_at(len);

            Ok((TokenTree::Comment(Self { inner, location }).into(), rest))
        } else {
            Err(TokenizeError::NotTokenized(input))
        }
    }
}

impl RTokenize<InputStr> for Comment {
    fn rtokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let chars = input.chars().rev();

        let len = Self::find_comment_len(chars);

        if len > 0 {
            let input_len = input.len();
            let (rest, location, inner) = input.rsplit_at(input_len - len);

            Ok((TokenTree::Comment(Self { inner, location }).into(), rest))
        } else {
            Err(TokenizeError::NotTokenized(input))
        }
    }
}
