use std::convert::TryFrom;
use std::iter::FromIterator;

use once_cell::sync::OnceCell;
use proc_macro2 as r;

pub use stylist_core::tokens::*;

mod input;

pub use input::InputTokens;

impl Tokenize<InputTokens> for Ident {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        let mut tokens = Vec::new();
        let mut token_s = "".to_string();

        let is_valid = |m: &r::TokenTree, last_is_ident: bool| match m {
            // You cannot have 2 consecutive idents without whitespaces
            r::TokenTree::Ident(_) => !last_is_ident,
            r::TokenTree::Punct(c) => c.as_char() == '-',
            _ => false,
        };

        let mut rest = input;
        let rest = loop {
            let last_is_ident = !matches!(tokens.last(), Some(r::TokenTree::Ident(_)));

            match rest.pop_by(|m| is_valid(m, last_is_ident).then(|| m.to_owned())) {
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
            let location = Location::TokenStream(r::TokenStream::from_iter(tokens));
            let ident = Ident::builder()
                .inner(token_s.into())
                .location(location)
                .build();
            Ok((TokenTree::Ident(ident).into(), rest))
        }
    }
}

impl Tokenize<InputTokens> for Group {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        let (result, rest) = input.pop_by(|m| match m {
            r::TokenTree::Group(ref group) => {
                let delim = Delimiter::try_from(group.delimiter()).ok()?;
                Some((m.to_owned(), group.to_owned(), delim))
            }
            _ => None,
        });

        let (group_token, group, delim) =
            result.ok_or_else(|| TokenizeError::NotTokenized(rest.clone()))?;

        let open_loc = Location::Span(group.span_open());
        let close_loc = Location::Span(group.span_close());

        let inner = TokenTree::tokenize_until_error(InputTokens::from(group.stream()))
            .empty_or_terminal() // MUST consume all.
            .map(|(m, _)| m)?;

        let location = Location::TokenStream(group_token.into());

        let group = Group::builder()
            .delim(delim)
            .open_loc(open_loc)
            .close_loc(close_loc)
            .inner(inner)
            .self_str(OnceCell::new())
            .location(location)
            .build();

        Ok((TokenTree::Group(group).into(), rest))
    }
}

impl Tokenize<InputTokens> for Punct {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        let (punct, rest) = input.pop_by(|m| match m {
            r::TokenTree::Punct(ref p) => Some(TokenStream::from(TokenTree::Punct(
                Punct::builder()
                    .inner(p.as_char().to_string().into())
                    .location(Location::TokenStream(m.clone().into()))
                    .build(),
            ))),
            _ => None,
        });

        match punct {
            Some(m) => Ok((m, rest)),
            None => Err(TokenizeError::NotTokenized(rest)),
        }
    }
}

impl Tokenize<InputTokens> for Literal {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        let (punct, rest) = input.pop_by(|m| match m {
            r::TokenTree::Literal(ref p) => Some(TokenStream::from(TokenTree::Literal(
                Literal::builder()
                    .inner(p.to_string().into())
                    .location(Location::TokenStream(m.clone().into()))
                    .build(),
            ))),
            _ => None,
        });

        match punct {
            Some(m) => Ok((m, rest)),
            None => Err(TokenizeError::NotTokenized(rest)),
        }
    }
}

impl Tokenize<InputTokens> for TokenTree {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        Ident::tokenize(input)
            // Comment are Spacing are not supported for inline.
            // .terminal_or_else(Spacing::tokenize)
            // .terminal_or_else(Comment::tokenize)
            .terminal_or_else(Punct::tokenize)
            .terminal_or_else(Ident::tokenize)
            .terminal_or_else(Group::tokenize)
            .terminal_or_else(Literal::tokenize)
    }
}
