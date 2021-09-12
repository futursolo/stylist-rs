use std::iter::FromIterator;

use arcstr::Substr;

use super::{InputStr, InputTokens, Location, Token, TokenTree, Tokenize};

#[derive(Debug, Clone)]
pub struct Ident {
    inner: Substr,
    location: Location,
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Token for Ident {
    fn as_str(&self) -> &str {
        &self.inner
    }
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Tokenize<InputStr> for Ident {
    fn tokenize(input: InputStr) -> Result<(TokenTree, InputStr), InputStr> {
        let valid_first_char =
            |c: char| c.is_ascii_alphabetic() || c == '-' || c == '_' || !c.is_ascii();
        let valid_rest_char =
            |c: &char| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || !c.is_ascii();

        let mut chars = input.chars();

        if !chars.next().map(valid_first_char).unwrap_or(false) {
            return Err(input);
        }

        let len = 1 + chars.take_while(valid_rest_char).count();
        let (inner, location, rest) = input.split_at(len);

        Ok((TokenTree::Ident(Ident { inner, location }), rest))
    }
}

impl Tokenize<InputTokens> for Ident {
    fn tokenize(mut input: InputTokens) -> Result<(TokenTree, InputTokens), InputTokens> {
        use super::rtokens::*;

        let mut tokens = Vec::new();

        while let Some(m) = input.pop_front() {
            // Accepts only - and ident.
            let valid = match &m {
                // You cannot have 2 consecutive idents without whitespaces
                RTokenTree::Ident(_) => !matches!(tokens.last(), Some(RTokenTree::Ident(_))),
                RTokenTree::Punct(c) => c.as_char() == '-',
                _ => false,
            };

            if valid {
                tokens.push(m);
            } else {
                input.push_front(m);
                break;
            }
        }

        let s = tokens
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join("");

        if tokens.is_empty() {
            Err(input)
        } else {
            let location = Location::Span(RTokenStream::from_iter(tokens));
            let ident = Self {
                inner: s.into(),
                location,
            };
            Ok((TokenTree::Ident(ident), input))
        }
    }
}
