use proc_macro2 as r;
#[cfg(feature = "proc_macro_support")]
use typed_builder::TypedBuilder;

use super::{
    Fragment, InputStr, Location, Token, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};
use crate::parser::ParseError;

#[cfg_attr(feature = "proc_macro_support", derive(TypedBuilder))]
#[derive(Debug, Clone)]
pub struct Interpolation {
    location: Location,

    expr: r::TokenStream,
    // open_loc: Location,
    // close_loc: Location,
}

// impl Interpolation {
//     /// Returns the location of the opening delimiter.
//     pub fn open_location(&self) -> &Location {
//         &self.open_loc
//     }

//     /// Returns the location of the closing delimiter.
//     pub fn close_location(&self) -> &Location {
//         &self.close_loc
//     }
// }

impl PartialEq for Interpolation {
    fn eq(&self, other: &Self) -> bool {
        self.expr.to_string() == other.expr.to_string()
    }
}

impl Token for Interpolation {
    fn location(&self) -> &Location {
        &self.location
    }

    // to_fragments for interpolation is special,
    // it returns the actual expression instead of a literal with `${}`.
    fn to_fragments(&self) -> Vec<Fragment> {
        vec![Fragment::Expr(self.expr.clone())]
    }
}

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
        // ${ + first char = 3
        let len = 3 + chars.take_while(valid_rest_char).count();

        if !input.chars().nth(len).map(|m| m == '}').unwrap_or(false) {
            let (_, location, _) = input.split_at(len).2.split_at(1);
            return Err(ParseError::unexpected_token(location).into());
        }

        let input_token = input.token();

        let (open_token, _open_loc, rest) = input.split_at(2);
        let (ident, ident_loc, rest) = rest.split_at(len - 2);
        let (close_token, _close_loc, rest) = rest.split_at(1);

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
                location,
                expr: arg.tokens,
                // open_loc,
                // close_loc,
            })
            .into(),
            rest,
        ))
    }
}
