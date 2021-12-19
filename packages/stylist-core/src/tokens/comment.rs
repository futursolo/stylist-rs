use arcstr::Substr;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::parser::ParseError;
use crate::{__impl_partial_eq, __impl_token};

/// A token that represents a comment.
///
/// `/* ... */`
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{ITokenizeResult, Location, Token};

    #[test]
    fn test_comment_empty() {
        let input = InputStr::from("/**/".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Comment(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "/**/");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 4);
        }
    }

    #[test]
    fn test_comment_some() {
        let input = InputStr::from("/*something*/".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Comment(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "/*something*/");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 13);
        }
    }

    #[test]
    fn test_comment_asterisk() {
        let input = InputStr::from("/*****/".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Comment(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "/*****/");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 7);
        }
    }

    #[test]
    fn test_comment_not_ok() {
        let input = InputStr::from("a/**/".to_string());

        let e = Comment::tokenize_until_error(input).unwrap_err();

        let i = match e {
            TokenizeError::NotTokenized(i) => i,
            _ => panic!(),
        };

        assert_eq!(&*i, "a/**/");
    }

    #[test]
    fn test_comment_invalid() {
        let input = InputStr::from("/**".to_string());

        let e = TokenTree::tokenize_until_error(input).unwrap_err();

        let e = match e {
            TokenizeError::Terminal(e) => e,
            _ => panic!(),
        };

        let loc = match e.location() {
            Location::Literal { range, .. } => range,
            _ => panic!(),
        };

        assert_eq!(loc.start, 0);
        assert_eq!(loc.end, 2);
    }
}
