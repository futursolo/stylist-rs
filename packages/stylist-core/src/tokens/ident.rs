use std::iter::FromIterator;

use arcstr::Substr;

use super::{
    InputStr, InputTokens, Location, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};
use crate::{__impl_partial_eq, __impl_token};

/// A token that represents a CSS ident.
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

impl Tokenize<InputTokens> for Ident {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        use super::rtokens::*;

        let mut tokens = Vec::new();
        let mut token_s = "".to_string();

        let is_valid = |m: &RTokenTree, last_is_ident: bool| match m {
            // You cannot have 2 consecutive idents without whitespaces
            RTokenTree::Ident(_) => !last_is_ident,
            RTokenTree::Punct(c) => c.as_char() == '-',
            _ => false,
        };

        let mut rest = input;
        let rest = loop {
            let last_is_ident = !matches!(tokens.last(), Some(RTokenTree::Ident(_)));

            match rest.pop_by(|m| is_valid(&m, last_is_ident).then(|| m)) {
                (Some(m), r) => {
                    token_s.push_str(&m.to_string());
                    tokens.push(m);
                    rest = r;
                }
                (None, rest) => {
                    break rest;
                }
            }
        };

        if tokens.is_empty() {
            Err(TokenizeError::NotTokenized(rest))
        } else {
            let location = Location::TokenStream(RTokenStream::from_iter(tokens));
            let ident = Self {
                inner: token_s.into(),
                location,
            };
            Ok((TokenTree::Ident(ident).into(), rest))
        }
    }
}
