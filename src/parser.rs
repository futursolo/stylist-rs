// Copyright © 2020 Lukas Wagner

use super::style::ast::{Block, Rule, RuleContent, Scope, ScopeContent, StyleAttribute};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::one_of,
    combinator::{map, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{many0, many1, separated_list},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

pub(super) struct Parser {}

impl Parser {
    pub(super) fn parse(css: String) -> Result<Vec<Scope>, String> {
        match Parser::scope_contents::<VerboseError<&str>>(css.as_str()) {
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                // here we use the `convert_error` function, to transform a `VerboseError<&str>`
                // into a printable trace.
                println!(
                    "CSS parsing error:\n{}",
                    convert_error(css.as_str(), e.clone())
                );
                Err(convert_error(css.as_str(), e))
            }
            Err(nom::Err::Incomplete(e)) => {
                println!("CSS parsing incomplete:\n{:?}", e);
                Err(format!("{:?}", e))
            }
            Ok((_, res)) => Ok(vec![Scope {
                condition: None,
                stylesets: res,
            }]),
        }
    }

    /// Parse whitespace
    fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
        let chars = " \t\r\n";
        take_while(move |c| chars.contains(c))(i)
    }

    /// Parse a comment
    fn cmt<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
        delimited(
            preceded(opt(Parser::sp), tag("/*")),
            is_not("*/"),
            terminated(tag("*/"), opt(Parser::sp)),
        )(i)
    }

    /// Parse a style attribute such as "width: 10px"
    fn attribute<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, StyleAttribute, E> {
        context(
            "StyleAttribute",
            map(
                separated_pair(
                    preceded(opt(Parser::cmt), preceded(Parser::sp, is_not(" \t\r\n:{"))),
                    preceded(opt(Parser::cmt), preceded(Parser::sp, tag(":"))),
                    preceded(opt(Parser::cmt), preceded(Parser::sp, is_not(";{}"))),
                ),
                move |p: (&str, &str)| -> StyleAttribute {
                    StyleAttribute {
                        key: String::from(p.0.trim()),
                        value: String::from(p.1.trim()),
                    }
                },
            ),
        )(i)
    }

    fn attributes<'a, E: ParseError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, Vec<StyleAttribute>, E> {
        context(
            "StyleAttributes",
            terminated(
                separated_list(preceded(Parser::sp, one_of(";")), Parser::attribute),
                preceded(Parser::sp, opt(tag(";"))),
            ),
        )(i)
    }

    fn block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ScopeContent, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleBlock",
            preceded(
                Parser::sp,
                map(
                    separated_pair(
                        is_not("{"),
                        tag("{"),
                        terminated(terminated(Parser::attributes, Parser::sp), tag("}")),
                    ),
                    |p: (&str, Vec<StyleAttribute>)| -> ScopeContent {
                        ScopeContent::Block(Block {
                            condition: Some(String::from(p.0)),
                            style_attributes: p.1,
                        })
                    },
                ),
            ),
        )(i)
    }

    fn rule<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ScopeContent, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "Rule",
            preceded(
                Parser::sp,
                map(
                    separated_pair(
                        preceded(tag("@"), is_not("{")),
                        tag("{"),
                        terminated(
                            terminated(
                                many0(alt((Parser::rule_string, Parser::rule_curly_braces))),
                                Parser::sp,
                            ),
                            tag("}"),
                        ),
                    ),
                    |p: (&str, Vec<RuleContent>)| -> ScopeContent {
                        ScopeContent::Rule(Rule {
                            condition: format!("{}{}", "@", p.0),
                            content: p.1,
                        })
                    },
                ),
            ),
        )(i)
    }

    /// Parse everything that is not curly braces
    fn rule_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, RuleContent, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        map(is_not("{}"), |p| RuleContent::String(String::from(p)))(i)
    }

    /// Parse values within curly braces. This is basically just a helper for rules since
    /// they may contain braced content. This function is for parsing it all and not
    /// returning an incomplete rule at the first appearance of a closed curly brace
    fn rule_curly_braces<'a, E: ParseError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, RuleContent, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        map(
            delimited(
                tag("{"),
                many0(alt((Parser::rule_string, Parser::rule_curly_braces))),
                tag("}"),
            ),
            RuleContent::CurlyBraces,
        )(i)
    }

    /// Parse a style attribute such as "width: 10px"
    fn dangling_attribute<'a, E: ParseError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, StyleAttribute, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleAttribute",
            map(
                separated_pair(
                    preceded(opt(Parser::cmt), preceded(Parser::sp, is_not(" \t\r\n:{"))),
                    preceded(opt(Parser::cmt), preceded(Parser::sp, tag(":"))),
                    preceded(
                        opt(Parser::cmt),
                        preceded(Parser::sp, terminated(is_not(";{}"), tag(";"))),
                    ),
                ),
                move |p: (&str, &str)| -> StyleAttribute {
                    StyleAttribute {
                        key: String::from(p.0.trim()),
                        value: String::from(p.1.trim()),
                    }
                },
            ),
        )(i)
    }

    fn dangling_attributes<'a, E: ParseError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, Vec<StyleAttribute>, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context("StyleAttributes", many1(Parser::dangling_attribute))(i)
    }

    fn dangling_block<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, ScopeContent, E> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        map(Parser::dangling_attributes, |attr: Vec<StyleAttribute>| {
            ScopeContent::Block(Block {
                condition: None,
                style_attributes: attr,
            })
        })(i)
    }

    fn scope_contents<'a, E: ParseError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, Vec<ScopeContent>, E> {
        context(
            "StyleScope",
            map(
                many0(alt((Parser::dangling_block, Parser::rule, Parser::block))),
                |p| {
                    println!("scope_contents: {:?}", p);
                    p
                },
            ),
        )(i)
    }
}
