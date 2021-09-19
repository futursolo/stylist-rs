use arcstr::Substr;
#[cfg(feature = "proc_macro_support")]
use typed_builder::TypedBuilder;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::{__impl_partial_eq, __impl_token};

/// A token that represents a CSS ident.
#[cfg_attr(feature = "proc_macro_support", derive(TypedBuilder))]
#[derive(Debug, Clone)]
pub struct Ident {
    inner: Substr,
    location: Location,
}

__impl_partial_eq!(Ident, inner);
__impl_token!(Ident);

impl Tokenize<InputStr> for Ident {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let valid_first_char =
            |c: char| c.is_ascii_alphabetic() || c == '-' || c == '_' || !c.is_ascii();
        let valid_rest_char = |c: &char| c.is_ascii_digit() || valid_first_char(*c);

        let mut chars = input.chars();

        if !chars.next().map(valid_first_char).unwrap_or(false) {
            return Err(TokenizeError::NotTokenized(input));
        }

        let len = 1 + chars.take_while(valid_rest_char).count();
        let (inner, location, rest) = input.split_at(len);

        Ok((TokenTree::Ident(Ident { inner, location }).into(), rest))
    }
}
