use once_cell::sync::OnceCell;

use super::{
    InputStr, InputTokens, Location, Token, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};
use crate::parser::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    Paren,
    Bracket,
    Brace,
}

impl Delimiter {
    fn parse_open(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Paren),
            '[' => Some(Self::Bracket),
            '{' => Some(Self::Brace),
            _ => None,
        }
    }

    fn match_close(&self, c: char) -> bool {
        match self {
            Self::Paren => ')' == c,
            Self::Bracket => ']' == c,
            Self::Brace => '}' == c,
        }
    }

    pub fn open_char(&self) -> char {
        match self {
            Self::Paren => '(',
            Self::Bracket => '[',
            Self::Brace => '{',
        }
    }

    pub fn close_char(&self) -> char {
        match self {
            Self::Paren => ')',
            Self::Bracket => ']',
            Self::Brace => '}',
        }
    }
}

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
    pub fn stream(&self) -> TokenStream {
        self.inner.clone()
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

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
        let delim = match input.chars().next().and_then(|m| Delimiter::parse_open(m)) {
            Some(m) => m,
            None => return Err(TokenizeError::NotTokenized(input)),
        };

        let input_token = input.token();
        let (open_char, open_loc, rest) = input.split_at(1);

        let (inner, rest) = match TokenTree::tokenize_until_error(rest) {
            Ok(m) => m,
            Err(TokenizeError::NotTokenized(e)) => (TokenStream::new(), e),
            Err(TokenizeError::Terminal(e)) => return Err(TokenizeError::Terminal(e)),
        };

        match rest.chars().next().map(|m| delim.match_close(m)) {
            Some(true) => (),
            Some(false) => {
                let (actual, location, _) = rest.split_at(1);

                return Err(TokenizeError::Terminal(ParseError::new(
                    format!("expected '{}', found '{}'", delim.close_char(), actual),
                    location,
                )));
            }
            None => {
                return Err(TokenizeError::Terminal(ParseError::new(
                    format!(
                        "didn't find the corresponding closing tag for {}",
                        delim.open_char()
                    ),
                    open_loc,
                )));
            }
        }

        let (end_char, close_loc, rest) = rest.split_at(1);

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
    fn tokenize(mut input: InputTokens) -> TokenizeResult<InputTokens, TokenStream> {
        use super::rtokens::*;

        let group_token = match input.get(0).cloned() {
            Some(m) => m,
            _ => return Err(TokenizeError::NotTokenized(input)),
        };

        let group = match group_token.clone() {
            RTokenTree::Group(m) => m,
            _ => return Err(TokenizeError::NotTokenized(input)),
        };

        let delim = match group.delimiter() {
            RDelimiter::Parenthesis => Delimiter::Paren,
            RDelimiter::Brace => Delimiter::Brace,
            RDelimiter::Bracket => Delimiter::Bracket,
            _ => return Err(TokenizeError::NotTokenized(input)),
        };
        input.pop_front();

        let open_loc = Location::Span(group.span_open());
        let close_loc = Location::Span(group.span_close());

        let (inner, _rest) =
            match TokenTree::tokenize_until_error(InputTokens::from(group.stream())) {
                Ok(m) => m,
                Err(TokenizeError::NotTokenized(e)) => {
                    // MUST consume all.
                    if let Some(m) = e.get(0).cloned() {
                        let location = Location::TokenStream(m.into());
                        return Err(TokenizeError::Terminal(ParseError::unexpected_token(
                            location,
                        )));
                    } else {
                        (TokenStream::new(), e)
                    }
                }
                Err(TokenizeError::Terminal(e)) => return Err(TokenizeError::Terminal(e)),
            };

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
            input,
        ))
    }
}
