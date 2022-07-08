use crate::parser::{Parse, ParseError, ParseResult, ParseStream};
use crate::tokens::{Delimiter, Ident, Token, TokenTree};

type Decl<'a> = ((Ident, Vec<TokenTree>), ParseStream<'a>);

fn parse_declaration(input: ParseStream<'_>, dangling: bool) -> ParseResult<Option<Decl<'_>>> {
    let original_input = input.clone();
    let input = input.trim_start();

    // Css Ident
    let (ident, input) = match input.pop_by(|m| match m {
        TokenTree::Ident(m) => Some(m.clone()),
        _ => None,
    }) {
        (Some(m), n) => (m, n),
        (None, _) => return Ok(None),
    };

    let input = input.trim_start();

    // colon(:)
    let (_, input) = match input.pop_by(|m| {
        if let TokenTree::Punct(m) = m {
            if m.as_char() == ':' {
                return Some(());
            }
        }
        None
    }) {
        (Some(m), n) => (m, n),
        (None, _) => return Ok(None),
    };

    // value.
    let mut value: Vec<TokenTree> = Vec::new();

    enum TokenKind {
        Fragment(Box<TokenTree>),
        Brace,
        Semicolon,
        Colon,
    }

    let mut input = input.trim_start();
    loop {
        if dangling {
            if input.is_empty() {
                // a ';' is required for dangling declarations.
                return Err(ParseError::new(
                    "expected ';', reached end of stylesheet.",
                    original_input
                        .iter()
                        .last()
                        .map(|m| m.location().clone())
                        .unwrap(),
                ));
            }
        } else if input.is_empty() {
            break; // end of the block.
        }

        let (token, rest) = match input.pop_by(|m| {
            match m {
                TokenTree::Group(m) => {
                    if m.delimiter() == Delimiter::Brace {
                        return None;
                    }
                }
                TokenTree::Punct(m) => match m.as_char() {
                    ';' => return Some(TokenKind::Semicolon),
                    ':' => return Some(TokenKind::Colon), // cannot have 2 colons in 1 declaration.
                    _ => {}
                },
                _ => {}
            }

            Some(TokenKind::Fragment(m.clone().into()))
        }) {
            (Some(m), n) => (m, n),
            (None, n) => (TokenKind::Brace, n),
        };

        input = rest;

        match token {
            TokenKind::Fragment(m) => value.push(*m),
            TokenKind::Brace => return Ok(None),
            TokenKind::Semicolon => break,
            TokenKind::Colon => return Ok(None),
        }
    }

    Ok(Some(((ident, value), input)))
}

/// A Css Style Declaration
///
/// https://www.w3.org/TR/css-syntax-3/#consume-declaration
#[derive(Debug)]
pub struct Declaration {
    name: Ident,
    value: Vec<TokenTree>,
}

impl<'a> Parse<'a> for Declaration {
    fn parse(input: ParseStream<'a>) -> ParseResult<Option<(Self, ParseStream<'a>)>> {
        Ok(parse_declaration(input, false)?
            .map(|((name, value), rest)| (Self { name, value }, rest)))
    }
}

/// A Css Style Declaration, but outside of a block.
///
/// https://www.w3.org/TR/css-syntax-3/#consume-declaration
#[derive(Debug)]
pub struct DanglingDeclaration {
    name: Ident,
    value: Vec<TokenTree>,
}

impl<'a> Parse<'a> for DanglingDeclaration {
    fn parse(input: ParseStream<'a>) -> ParseResult<Option<(Self, ParseStream<'a>)>> {
        Ok(parse_declaration(input, true)?
            .map(|((name, value), rest)| (Self { name, value }, rest)))
    }
}
