use arcstr::Substr;

use super::{
    InputStr, InputTokens, Location, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};
use crate::{__impl_partial_eq, __impl_token};

/// A token that represents a punctuation mark.
///
/// It is a single punctuation character like +, - or #.
#[derive(Debug, Clone)]
pub struct Punct {
    inner: Substr,
    location: Location,
}

impl Punct {
    /// Returns the value of this punctuation as [`char`].
    pub fn as_char(&self) -> char {
        self.inner.chars().next().unwrap()
    }
}

__impl_partial_eq!(Punct, inner);
__impl_token!(Punct);

impl Tokenize<InputStr> for Punct {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let valid_char = |c: &char| "&#+,-.:;<@\\".contains(*c);

        if input.chars().next().filter(valid_char).is_some() {
            let (inner, location, rest) = input.split_at(1);

            Ok((TokenTree::Punct(Punct { inner, location }).into(), rest))
        } else {
            Err(TokenizeError::NotTokenized(input))
        }
    }
}

impl Tokenize<InputTokens> for Punct {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        use super::rtokens::*;

        let (punct, rest) = input.pop_by(|m| match m {
            RTokenTree::Punct(ref p) => Some(TokenStream::from(TokenTree::Punct(Punct {
                inner: p.as_char().to_string().into(),
                location: Location::TokenStream(m.clone().into()),
            }))),
            _ => None,
        });

        match punct {
            Some(m) => Ok((m, rest)),
            None => Err(TokenizeError::NotTokenized(rest)),
        }
    }
}
