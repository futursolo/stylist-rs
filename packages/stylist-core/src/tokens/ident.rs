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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{ITokenizeResult, Location, Token};

    #[test]
    fn test_ident_ok() {
        let input = InputStr::from("ident-Ident_12345_".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Ident(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "ident-Ident_12345_");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 18);
        }
    }

    #[test]
    fn test_ident_not_ok() {
        let input = InputStr::from("12345".to_string());

        let e = Ident::tokenize_until_error(input).unwrap_err();

        let i = match e {
            TokenizeError::NotTokenized(i) => i,
            _ => panic!(),
        };

        assert_eq!(&*i, "12345");
    }
}
