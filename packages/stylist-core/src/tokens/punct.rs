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
    fn tokenize(mut input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        use super::rtokens::*;

        if let Some(m) = input.get(0).cloned() {
            if let RTokenTree::Punct(ref p) = m {
                input.pop_front();

                let s = p.as_char().to_string();
                let location = Location::TokenStream(m.clone().into());

                return Ok((
                    TokenTree::Punct(Punct {
                        inner: s.into(),
                        location,
                    })
                    .into(),
                    input,
                ));
            }
        }

        Err(TokenizeError::NotTokenized(input))
    }
}
