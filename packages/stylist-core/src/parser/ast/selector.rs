use crate::parser::{Parse, ParseError, ParseResult, ParseStream};
use crate::tokens::{Delimiter, Token, TokenTree};

/// A Css Selector
///
/// https://www.w3.org/TR/selectors-4/
#[derive(Debug)]
pub struct Selector {
    inner: Vec<TokenTree>,
}

impl<'a> Parse<'a> for Selector {
    fn parse(input: ParseStream<'a>) -> ParseResult<Option<(Self, ParseStream<'a>)>> {
        // We do not implement a strict selector parsing logic, we try to collect everything
        // before a comma or a brace.
        let mut tokens = Vec::new();
        let mut input = input;

        let mut maybe_rule_token = None;

        loop {
            input = input.trim_start();

            let token = match input.pop_by(|m| match m {
                TokenTree::Punct(t) if t.as_char() == '@' => {
                    maybe_rule_token = Some(t.clone());
                    None
                }
                TokenTree::Punct(t) if t.as_char() == ',' => None,
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => None,
                _ => Some(m.clone()),
            }) {
                (Some(token), next_input) => {
                    input = next_input;
                    token
                }
                (None, next_input) => {
                    input = next_input;
                    break;
                }
            };

            tokens.push(token);
        }

        if tokens.is_empty() {
            return Ok(None);
        }

        if let Some(m) = maybe_rule_token {
            // You cannot have @ in between selectors.
            return Err(ParseError::unexpected_token(m.location().clone()));
        }

        Ok(Some((Self { inner: tokens }, input)))
    }
}
