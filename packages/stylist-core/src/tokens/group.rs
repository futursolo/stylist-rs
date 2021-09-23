#[cfg(feature = "proc_macro_support")]
use typed_builder::TypedBuilder;

use super::{
    ITokenizeResult, InputStr, Location, Token, TokenStream, TokenTree, Tokenize, TokenizeError,
    TokenizeResult,
};
use crate::__impl_partial_eq;
use crate::arc_ref::ArcRef;
use crate::parser::ParseError;

#[cfg(feature = "proc_macro_support")]
use super::Fragment;

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

    fn str_pair(&self) -> (&str, &str) {
        match self {
            Self::Paren => ("(", ")"),
            Self::Bracket => ("[", "]"),
            Self::Brace => ("{", "}"),
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

    /// Returns the opening delimiter as `&str`.
    pub fn open_str(&self) -> &str {
        self.str_pair().0
    }

    /// Returns the closing delimiter as `&str`.
    pub fn close_str(&self) -> &str {
        self.str_pair().1
    }
}

/// A token that represents a Group (Block) surrounded by a [`Delimiter`].
#[cfg_attr(feature = "proc_macro_support", derive(TypedBuilder))]
#[derive(Debug, Clone)]
pub struct Group {
    delim: Delimiter,

    open_loc: Location,
    close_loc: Location,

    inner: ArcRef<'static, TokenStream>,

    location: Location,
}

impl Group {
    /// Returns the [`TokenStream`] of tokens that are delimited in this [`Group`].
    pub fn stream(&self) -> ArcRef<'_, TokenStream> {
        self.inner.clone()
    }

    /// Returns the [`Delimiter`] of the current group.
    pub fn delimiter(&self) -> Delimiter {
        self.delim.clone()
    }

    /// Returns the location of the opening delimiter.
    pub fn open_location(&self) -> &Location {
        &self.open_loc
    }

    /// Returns the location of the closing delimiter.
    pub fn close_location(&self) -> &Location {
        &self.close_loc
    }
}

__impl_partial_eq!(Group, inner, delim);

impl Token for Group {
    #[cfg(not(feature = "proc_macro_support"))]
    fn to_fragments(&self) -> Vec<&str> {
        let fragments = vec![self.delimiter().open_str()];

        for token in self.stream().iter() {
            fragments.extend(token.as_fragments);
        }

        fragments.push(self.delimiter().close_str());

        fragments
    }

    fn to_fragments(&self) -> Vec<Fragment> {
        let mut fragments = vec![Fragment::Literal(self.delimiter().open_char().to_string())];

        for token in self.stream().iter() {
            fragments.extend(token.to_fragments());
        }

        fragments.push(Fragment::Literal(self.delimiter().close_char().to_string()));

        fragments
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

        #[cfg(feature = "proc_macro_support")]
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
            #[cfg(feature = "proc_macro_support")]
            token: input_token,
            range,
        };

        Ok((
            TokenTree::Group(Self {
                delim,

                open_loc,
                close_loc,

                inner: ArcRef::from(inner),

                location,
            })
            .into(),
            rest,
        ))
    }
}

#[cfg(feature = "proc_macro_support")]
mod feat_proc_macro {
    use super::*;
    use proc_macro2 as r;
    use std::convert::TryFrom;

    impl TryFrom<r::Delimiter> for Delimiter {
        type Error = ();

        fn try_from(m: r::Delimiter) -> std::result::Result<Self, Self::Error> {
            match m {
                r::Delimiter::Parenthesis => Ok(Delimiter::Paren),
                r::Delimiter::Brace => Ok(Delimiter::Brace),
                r::Delimiter::Bracket => Ok(Delimiter::Bracket),
                _ => Err(()),
            }
        }
    }
}
