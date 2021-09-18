use arcstr::Substr;

use super::{
    InputStr, Location, Token, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult,
};

#[derive(Debug, Clone)]
pub struct Spacing {
    inner: Substr,
    location: Location,
}

impl Token for Spacing {
    fn as_str(&self) -> &str {
        &self.inner
    }
    fn location(&self) -> &Location {
        &self.location
    }
}

impl PartialEq for Spacing {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Tokenize<InputStr> for Spacing {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let chars = input.chars();

        let len = chars.take_while(|c| " \t\r\n".contains(*c)).count();

        if len > 0 {
            let (inner, location, rest) = input.split_at(len);

            Ok((TokenTree::Spacing(Spacing { inner, location }).into(), rest))
        } else {
            Err(TokenizeError::NotTokenized(input))
        }
    }
}

// Inferred Space for tokens.
impl Default for Spacing {
    fn default() -> Self {
        use super::rtokens::*;

        let mut call_site_space = RLiteral::string(" ");
        call_site_space.set_span(RSpan::call_site());

        Self {
            inner: " ".into(),
            location: Location::TokenStream(RTokenTree::Literal(call_site_space).into()),
        }
    }
}

// Spacing is not parsed for TokenStream, but rather inferred after TokenStream is generated.
// impl Tokenize<InputTokens> for Spacing {
//     fn tokenize(mut input: InputTokens) -> Result<(TokenStream, InputTokens), InputTokens> {
//         unimplemented!();
//     }
// }
