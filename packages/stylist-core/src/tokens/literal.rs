use arcstr::Substr;
#[cfg(feature = "proc_macro_support")]
use typed_builder::TypedBuilder;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::parser::ParseError;
use crate::{__impl_partial_eq, __impl_token};

/// A token that is either a string literal, a number or a percentage.
///
/// - `"Some String"`
/// - `123`
/// - `50%`
#[derive(Debug, PartialEq)]
enum LiteralState {
    NotFound,
    Escaped,
    Ended,
}

#[cfg_attr(feature = "proc_macro_support", derive(TypedBuilder))]
#[derive(Debug, Clone)]
pub struct Literal {
    // We don't care about the content of a literal, so we store everything as string.
    inner: Substr,
    location: Location,
}

__impl_partial_eq!(Literal, inner);
__impl_token!(Literal);

impl Literal {
    // https://www.w3.org/TR/css-syntax-3/#consume-numeric-token
    fn parse_number(input: InputStr) -> Result<Option<usize>, ParseError> {
        // + and - are handled as Punct.
        let is_digit = |m: &char| m.is_ascii_digit();

        let mut len = input.chars().take_while(is_digit).count();
        let to_return_result = |m| if m > 0 { Ok(Some(m)) } else { Ok(None) };

        let maybe_point = match input.chars().nth(len) {
            Some(m) => m,
            None => return to_return_result(len),
        };

        // floating point
        if maybe_point == '.' {
            len += 1;
            let rational_len = input.chars().skip(len).take_while(is_digit).count();

            if rational_len == 0 {
                let (_, _, rest) = input.split_at(len - 1);
                let (_, location, _rest) = rest.split_at(1);

                return Err(ParseError::unexpected_token(location));
            }

            len += rational_len
        }

        if Some('%') == input.chars().nth(len) {
            len += 1;
        }

        // dimension is handled as Ident.

        to_return_result(len)
    }

    fn parse_string(input: InputStr) -> Result<Option<usize>, ParseError> {
        let mut chars = input.chars();

        let delim = match chars
            .next()
            .and_then(|m| (m == '"' || m == '\'').then(|| m))
        {
            Some(m) => m,
            None => return Ok(None),
        };

        let mut len = 1;
        let mut state = LiteralState::NotFound;
        for c in chars {
            len += 1;

            if state != LiteralState::Escaped && c == delim {
                state = LiteralState::Ended;
                break;
            }

            if state == LiteralState::NotFound && c == '\\' {
                state = LiteralState::Escaped;
            } else {
                state = LiteralState::NotFound;
            }
        }

        if state != LiteralState::Ended {
            let (_, location, _rest) = input.split_at(1);

            return Err(ParseError::new(
                format!("cannot find the end of this string, expected {}", delim),
                location,
            ));
        }

        Ok(Some(len))
    }
}

impl Tokenize<InputStr> for Literal {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        match Self::parse_string(input.clone()).and_then(|m| {
            if m.is_some() {
                Ok(m)
            } else {
                Self::parse_number(input.clone())
            }
        })? {
            Some(m) => {
                let (inner, location, rest) = input.split_at(m);

                Ok((TokenTree::Literal(Self { inner, location }).into(), rest))
            }

            None => Err(TokenizeError::NotTokenized(input)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{ITokenizeResult, Location, Token};

    #[test]
    fn test_int() {
        let input = InputStr::from("12345".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "12345");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 5);
        }
    }

    #[test]
    fn test_float() {
        let input = InputStr::from("12345.6".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "12345.6");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 7);
        }
    }

    #[test]
    fn test_string_empty() {
        let input = InputStr::from("\"\"".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "\"\"");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 2);
        }
    }

    #[test]
    fn test_string_some() {
        let input = InputStr::from("\"somestr\"".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "\"somestr\"");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 9);
        }
    }
    #[test]
    fn test_string_mix() {
        let input = InputStr::from("\"'\"".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "\"'\"");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 3);
        }
    }

    #[test]
    fn test_string_single() {
        let input = InputStr::from("''".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "''");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 2);
        }
    }

    #[test]
    fn test_string_single_mix() {
        let input = InputStr::from("'\"'".to_string());

        let tokens = TokenTree::tokenize_until_error(input)
            .empty_or_terminal()
            .unwrap();

        for (index, token) in tokens.into_iter().enumerate() {
            assert_eq!(index, 0);

            let t = match token {
                TokenTree::Literal(m) => m,
                _ => panic!(),
            };

            assert_eq!(t.inner, "'\"'");

            let loc = match t.location() {
                Location::Literal { range, .. } => range,
                _ => panic!(),
            };

            assert_eq!(loc.start, 0);
            assert_eq!(loc.end, 3);
        }
    }

    #[test]
    fn test_string_invalid() {
        let input = InputStr::from("'".to_string());

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
        assert_eq!(loc.end, 1);
    }
}
