use std::convert::TryFrom;

use once_cell::sync::OnceCell;

use super::rtokens::RDelimiter;
use super::{
    ITokenizeResult, InputStr, InputTokens, Location, Token, TokenStream, TokenTree, Tokenize,
    TokenizeError, TokenizeResult,
};
use crate::__impl_partial_eq;
use crate::parser::ParseError;

/// The delimiter of a [`Group`].
#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    /// `(...)`
    Paren,
    /// `[...]`
    Bracket,
    /// `{...}`
    Brace,
}

impl Delimiter {
    fn char_pair(&self) -> (char, char) {
        match self {
            Self::Paren => ('(', ')'),
            Self::Bracket => ('[', ']'),
            Self::Brace => ('{', '}'),
        }
    }

    fn parse_open(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Paren),
            '[' => Some(Self::Bracket),
            '{' => Some(Self::Brace),
            _ => None,
        }
    }

    fn match_close(&self, c: char) -> bool {
        self.close_char() == c
    }

    /// Returns the opening delimiter as char.
    pub fn open_char(&self) -> char {
        self.char_pair().0
    }

    /// Returns the closing delimiter as char.
    pub fn close_char(&self) -> char {
        self.char_pair().1
    }
}

impl TryFrom<RDelimiter> for Delimiter {
    type Error = ();

    fn try_from(m: RDelimiter) -> std::result::Result<Self, Self::Error> {
        match m {
            RDelimiter::Parenthesis => Ok(Delimiter::Paren),
            RDelimiter::Brace => Ok(Delimiter::Brace),
            RDelimiter::Bracket => Ok(Delimiter::Bracket),
            _ => Err(()),
        }
    }
}

/// A token that represents a Group (Block) surrounded by a [`Delimiter`].
#[derive(Debug, Clone)]
pub struct Group {
    delim: Delimiter,

    open_loc: Location,
    close_loc: Location,

    inner: TokenStream,

    self_str: OnceCell<String>,

    location: Location,
}

impl Group {
    /// Returns the [`TokenStream`] of tokens that are delimited in this [`Group`].
    pub fn stream(&self) -> TokenStream {
        self.inner.clone()
    }

    /// Returns the [`Delimiter`] of the current group.
    pub fn delimiter(&self) -> Delimiter {
        self.delim.clone()
    }

    /// Returns the location of the opening delimiter.
    pub fn open_location(&self) -> &Location {
        &self.location
    }

    /// Returns the location of the closing delimiter.
    pub fn close_location(&self) -> &Location {
        &self.location
    }
}

__impl_partial_eq!(Group, inner, delim);

impl Token for Group {
    fn as_str(&self) -> &str {
        self.self_str.get_or_init(|| {
            let mut s = self.delim.open_char().to_string();

            for token in self.stream().into_iter() {
                s.push_str(token.as_str());
            }

            s.push(self.delim.close_char());

            s
        })
    }
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Tokenize<InputStr> for Group {
    fn tokenize(input: InputStr) -> TokenizeResult<InputStr, TokenStream> {
        let delim = input
            .chars()
            .next()
            .and_then(Delimiter::parse_open)
            .ok_or_else(|| TokenizeError::NotTokenized(input.clone()))?;

        let input_token = input.token();
        let (open_char, open_loc, rest) = input.split_at(1);

        let (inner, rest) = TokenTree::tokenize_until_error(rest).terminal_or_ok()?;

        let (end_char, close_loc, rest) = match rest.chars().next().map(|m| delim.match_close(m)) {
            Some(true) => Ok(rest.split_at(1)),
            Some(false) => {
                let (actual, location, _) = rest.split_at(1);

                Err(ParseError::new(
                    format!("expected '{}', found '{}'", delim.close_char(), actual),
                    location,
                ))
            }
            None => Err(ParseError::new(
                format!(
                    "didn't find the corresponding closing tag for {}",
                    delim.open_char()
                ),
                open_loc.clone(),
            )),
        }?;

        let range = open_char.range().start..end_char.range().end;
        let location = Location::Literal {
            token: input_token,
            range,
        };

        Ok((
            TokenTree::Group(Self {
                delim,

                open_loc,
                close_loc,

                inner,

                self_str: OnceCell::new(),

                location,
            })
            .into(),
            rest,
        ))
    }
}

impl Tokenize<InputTokens> for Group {
    fn tokenize(input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        use super::rtokens::*;

        let (result, rest) = input.pop_by(|m| match m.clone() {
            RTokenTree::Group(group) => {
                let delim = Delimiter::try_from(group.delimiter()).ok()?;
                Some((m, group, delim))
            }
            _ => None,
        });

        let (group_token, group, delim) =
            result.ok_or_else(|| TokenizeError::NotTokenized(rest.clone()))?;

        let open_loc = Location::Span(group.span_open());
        let close_loc = Location::Span(group.span_close());

        let inner = TokenTree::tokenize_until_error(InputTokens::from(group.stream()))
            .terminal_or_else(|e| // MUST consume all.
                match e.peek() {
                    Some(m) => {
                        let location = Location::TokenStream(m.clone().into());
                        Err(ParseError::unexpected_token(
                            location,
                        ).into())
                    }
                    None => Ok((TokenStream::new(), e)),
                })
            .map(|(m, _)| m)?;

        let location = Location::TokenStream(group_token.into());

        Ok((
            TokenTree::Group(Self {
                delim,

                open_loc,
                close_loc,

                inner,

                self_str: OnceCell::new(),

                location,
            })
            .into(),
            rest,
        ))
    }
}
