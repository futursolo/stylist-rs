use arcstr::Substr;

use super::{
    InputStr, InputTokens, Location, Token, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};

#[derive(Debug, Clone)]
pub struct Punct {
    inner: Substr,
    location: Location,
}

impl PartialEq for Punct {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Token for Punct {
    fn as_str(&self) -> &str {
        &self.inner
    }
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Tokenize<InputStr> for Punct {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let valid_char = |c: &char| "#+,-.:;<@\\".contains(*c);

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
