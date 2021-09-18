use arcstr::Substr;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::__impl_token;

#[derive(Debug, Clone)]
pub struct Space {
    inner: Substr,
    location: Location,
}

__impl_token!(Space);

impl PartialEq for Space {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Tokenize<InputStr> for Space {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let len = input.chars().take_while(|c| " \t\r\n".contains(*c)).count();

        if len > 0 {
            let (inner, location, rest) = input.split_at(len);

            Ok((TokenTree::Space(Self { inner, location }).into(), rest))
        } else {
            Err(TokenizeError::NotTokenized(input))
        }
    }
}

// Inferred Space for tokens.
impl Default for Space {
    fn default() -> Self {
        use super::rtokens::*;

        let space = RLiteral::string(" ");

        Self {
            inner: " ".into(),
            location: Location::TokenStream(RTokenTree::Literal(space).into()),
        }
    }
}

// Spacing is not parsed for TokenStream, but rather inferred after TokenStream is generated.
// impl Tokenize<InputTokens> for Spacing {
//     fn tokenize(mut input: InputTokens) -> Result<(TokenStream, InputTokens), InputTokens> {
//         unimplemented!();
//     }
// }
