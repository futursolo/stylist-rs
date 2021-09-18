use arcstr::Substr;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::parser::ParseError;
use crate::{__impl_partial_eq, __impl_token};

#[derive(Debug, Clone)]
pub struct Comment {
    inner: Substr,
    location: Location,
}

__impl_token!(Comment);
__impl_partial_eq!(Comment, inner);

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

// It's not possible to read comments from proc_macro2::TokenStream at this moment.
