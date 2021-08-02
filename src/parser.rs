// Copyright © 2020 Lukas Wagner

use super::style::ast::{Block, Rule, RuleContent, Scope, ScopeContent, StyleAttribute};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::one_of,
    combinator::{map, map_res, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

pub(super) struct Parser {}

impl Parser {
    pub(super) fn parse(css: String) -> Result<Vec<Scope>, String> {
        match Parser::scopes(css.as_str()) {
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                // here we use the `convert_error` function, to transform a `VerboseVerboseError<&str>`
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
            Ok((_, res)) => Ok(res),
        }
    }

    /// Parse whitespace
    fn sp(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        let chars = " \t\r\n";
        take_while(move |c| chars.contains(c))(i)
    }

    /// Parse a comment
    fn cmt(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        context(
            "StyleComment",
            terminated(
                preceded(
                    opt(Parser::sp),
                    delimited(
                        preceded(opt(Parser::sp), tag("/*")),
                        // not(tag("*/")), // TODO check for the string
                        is_not("*"),
                        terminated(tag("*/"), opt(Parser::sp)),
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    /// Parse a style attribute such as "width: 10px"
    fn attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        context(
            "StyleAttribute",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(
                        separated_pair(
                            preceded(
                                opt(Parser::cmt),
                                preceded(opt(Parser::sp), is_not(" \t\r\n:{")),
                            ),
                            preceded(opt(Parser::cmt), preceded(opt(Parser::sp), tag(":"))),
                            preceded(opt(Parser::cmt), preceded(opt(Parser::sp), is_not(";{}"))),
                        ),
                        move |p: (&str, &str)| -> StyleAttribute {
                            StyleAttribute {
                                key: String::from(p.0.trim()),
                                value: String::from(p.1.trim()),
                            }
                        },
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn attributes(i: &str) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        context(
            "StyleAttributes",
            terminated(
                preceded(
                    opt(Parser::sp),
                    terminated(
                        separated_list0(preceded(opt(Parser::sp), one_of(";")), Parser::attribute),
                        preceded(opt(Parser::sp), opt(tag(";"))),
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleBlock",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(
                        separated_pair(
                            is_not("@{"),
                            tag("{"),
                            terminated(terminated(Parser::attributes, opt(Parser::sp)), tag("}")),
                        ),
                        |p: (&str, Vec<StyleAttribute>)| -> ScopeContent {
                            ScopeContent::Block(Block {
                                condition: Some(String::from(p.0)),
                                style_attributes: p.1,
                            })
                        },
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn rule(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "Rule",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map_res(
                        separated_pair(
                            preceded(tag("@"), is_not("{")),
                            tag("{"),
                            terminated(
                                terminated(
                                    many0(alt((Parser::rule_string, Parser::rule_curly_braces))),
                                    opt(Parser::sp),
                                ),
                                tag("}"),
                            ),
                        ),
                        |p: (&str, Vec<RuleContent>)| -> Result<ScopeContent, String> {
                            if p.0.starts_with("media") {
                                return Err(String::from("Not a media query"));
                            }
                            Ok(ScopeContent::Rule(Rule {
                                condition: format!("{}{}", "@", p.0),
                                content: p.1,
                            }))
                        },
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    /// Parse everything that is not curly braces
    fn rule_string(i: &str) -> IResult<&str, RuleContent, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleRuleString",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(is_not("{}"), |p| RuleContent::String(String::from(p))),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    /// Parse values within curly braces. This is basically just a helper for rules since
    /// they may contain braced content. This function is for parsing it all and not
    /// returning an incomplete rule at the first appearance of a closed curly brace
    fn rule_curly_braces(i: &str) -> IResult<&str, RuleContent, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleRuleCurlyBraces",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(
                        delimited(
                            tag("{"),
                            many0(alt((Parser::rule_string, Parser::rule_curly_braces))),
                            tag("}"),
                        ),
                        RuleContent::CurlyBraces,
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    /// Parse a style attribute such as "width: 10px"
    fn dangling_attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleAttribute",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(
                        separated_pair(
                            preceded(
                                opt(Parser::cmt),
                                preceded(opt(Parser::sp), is_not(" \t\r\n:{")),
                            ),
                            preceded(opt(Parser::cmt), preceded(opt(Parser::sp), tag(":"))),
                            preceded(
                                opt(Parser::cmt),
                                preceded(opt(Parser::sp), terminated(is_not(";{}"), tag(";"))),
                            ),
                        ),
                        move |p: (&str, &str)| -> StyleAttribute {
                            StyleAttribute {
                                key: String::from(p.0.trim()),
                                value: String::from(p.1.trim()),
                            }
                        },
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn dangling_attributes(i: &str) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleAttributes",
            terminated(
                preceded(opt(Parser::sp), many1(Parser::dangling_attribute)),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn dangling_block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleDanglingBlock",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(Parser::dangling_attributes, |attr: Vec<StyleAttribute>| {
                        ScopeContent::Block(Block {
                            condition: None,
                            style_attributes: attr,
                        })
                    }),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn scope_contents(i: &str) -> IResult<&str, Vec<ScopeContent>, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "ScopeContents",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(
                        many0(alt((Parser::dangling_block, Parser::rule, Parser::block))),
                        |p| {
                            println!("scope_contents: {:?}", p);
                            p
                        },
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn scope(i: &str) -> IResult<&str, Scope, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleScope",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(Parser::scope_contents, |sc| Scope {
                        condition: None,
                        stylesets: sc,
                    }),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn media_rule(i: &str) -> IResult<&str, Scope, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "MediaRule",
            terminated(
                preceded(
                    opt(Parser::sp),
                    map(
                        separated_pair(
                            preceded(tag("@media "), is_not("{")),
                            tag("{"),
                            terminated(
                                terminated(Parser::scope_contents, opt(Parser::sp)),
                                tag("}"),
                            ),
                        ),
                        |p: (&str, Vec<ScopeContent>)| -> Scope {
                            Scope {
                                condition: Some(format!("{}{}", "@media ", p.0)),
                                stylesets: p.1,
                            }
                        },
                    ),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }

    fn scopes(i: &str) -> IResult<&str, Vec<Scope>, VerboseError<&str>> {
        if i.is_empty() {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )));
        }
        context(
            "StyleScopes",
            terminated(
                preceded(
                    opt(Parser::sp),
                    many0(alt((Parser::media_rule, Parser::scope))),
                ),
                opt(Parser::sp),
            ),
        )(i)
    }
}
