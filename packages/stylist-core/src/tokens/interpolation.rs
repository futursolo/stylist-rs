use arcstr::Substr;
use proc_macro2 as r;

use super::{InputStr, Location, TokenStream, TokenTree, Tokenize, TokenizeError, TokenizeResult};
use crate::parser::ParseError;
use crate::{__impl_partial_eq, __impl_token};

#[derive(Debug, Clone)]
pub struct Interpolation {
    inner: Substr,
    location: Location,

    ident: Substr,
    ident_loc: Location,
    expr: r::TokenStream,

    open_loc: Location,
    close_loc: Location,
}

__impl_partial_eq!(Interpolation, inner);
__impl_token!(Interpolation);

impl Tokenize<InputStr> for Interpolation {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let valid_first_char = |c: char| c.is_ascii_alphabetic() || c == '_' || !c.is_ascii();
        let valid_rest_char = |c: &char| c.is_ascii_digit() || valid_first_char(*c);

        if !input.starts_with("${") {
            return Err(TokenizeError::NotTokenized(input));
        }

        let mut chars = input.chars().skip(2);

        if !chars.next().map(valid_first_char).unwrap_or(false) {
            let (_, location, _) = input.split_at(2).2.split_at(1);
            return Err(ParseError::unexpected_token(location).into());
        }

        let len = 3 + chars.take_while(valid_rest_char).count();

        if !input.chars().nth(len).map(|m| m == '}').unwrap_or(false) {
            let (_, location, _) = input.split_at(len).2.split_at(1);
            return Err(ParseError::unexpected_token(location).into());
        }

        let input_token = input.token();

        let (open_token, open_loc, rest) = input.split_at(2);
        let (ident, ident_loc, rest) = rest.split_at(len - 2);
        let (close_token, close_loc, rest) = rest.split_at(1);

        let arg = match rest.args().get(&ident) {
            Some(m) => m,
            None => {
                return Err(
                    ParseError::new(format!("missing argument: {}", ident), ident_loc).into(),
                )
            }
        };

        let range = open_token.range().start..close_token.range().end;
        let location = Location::Literal {
            token: input_token,
            range,
        };

        Ok((
            TokenTree::Expr(Self {
                inner: format!("${{{}}}", ident).into(),
                ident,
                ident_loc,
                location,
                expr: arg.tokens,
                open_loc,
                close_loc,
            })
            .into(),
            rest,
        ))
    }
}
