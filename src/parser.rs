use crate::ast::{Block, Rule, RuleContent, Scope, ScopeContent, StyleAttribute};
use crate::{Error, Result};
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

#[cfg(test)]
use log::trace;

pub(crate) struct Parser;

impl Parser {
    /// Returns Error when string is Empty
    fn expect_non_empty(i: &str) -> std::result::Result<(), nom::Err<VerboseError<&str>>> {
        if i.is_empty() {
            Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )))
        } else {
            Ok(())
        }
    }

    /// Parse whitespace
    fn sp(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        Self::expect_non_empty(i)?;

        let chars = " \t\r\n";
        take_while(move |c| chars.contains(c))(i)
    }

    /// Drop whitespaces
    fn trimmed<'a, F, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&str>>
    where
        F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
    {
        // Drop Trailing whitespaces.
        terminated(
            preceded(
                // Drop Preceeding whitespaces.
                opt(Self::sp),
                // Parse until finishes
                f,
            ),
            opt(Self::sp),
        )
    }

    /// Parse a comment
    fn cmt(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Comment: {}", i);

        let result = context(
            "StyleComment",
            Self::trimmed(delimited(
                preceded(opt(Self::sp), tag("/*")),
                // not(tag("*/")), // TODO check for the string
                is_not("*"),
                terminated(tag("*/"), opt(Self::sp)),
            )),
        )(i);

        #[cfg(test)]
        trace!("Comment: {:#?}", result);

        result
    }

    /// Parse a style attribute such as "width: 10px"
    fn attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Attribute: {}", i);

        let result = context(
            "StyleAttribute",
            Self::trimmed(map(
                separated_pair(
                    preceded(
                        opt(Parser::cmt),
                        preceded(opt(Parser::sp), is_not(" \t\r\n:{")),
                    ),
                    preceded(opt(Parser::cmt), preceded(opt(Parser::sp), tag(":"))),
                    preceded(opt(Parser::cmt), preceded(opt(Parser::sp), is_not(";{}"))),
                ),
                move |p: (&str, &str)| StyleAttribute {
                    key: String::from(p.0.trim()),
                    value: String::from(p.1.trim()),
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Attribute: {:#?}", result);

        result
    }

    fn attributes(i: &str) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Attributes: {}", i);

        let result = context(
            "StyleAttributes",
            Self::trimmed(terminated(
                separated_list0(preceded(opt(Parser::sp), one_of(";")), Parser::attribute),
                preceded(opt(Parser::sp), opt(tag(";"))),
            )),
        )(i);

        #[cfg(test)]
        trace!("Attributes: {:#?}", result);

        result
    }

    fn block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Block: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleBlock",
            Self::trimmed(map(
                separated_pair(
                    is_not("}@{"),
                    tag("{"),
                    terminated(terminated(Parser::attributes, opt(Parser::sp)), tag("}")),
                ),
                |p: (&str, Vec<StyleAttribute>)| {
                    ScopeContent::Block(Block {
                        condition: Some(p.0.trim().to_string()),
                        style_attributes: p.1,
                    })
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Block: {:#?}", result);

        result
    }

    fn rule(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Rule: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Rule",
            Self::trimmed(map_res(
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
                |p: (&str, Vec<RuleContent>)| {
                    if p.0.starts_with("media") {
                        return Err(String::from("Not a media query"));
                    }
                    Ok(ScopeContent::Rule(Rule {
                        condition: format!("@{}", p.0),
                        content: p.1,
                    }))
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Rule: {:#?}", result);

        result
    }

    /// Parse everything that is not curly braces
    fn rule_string(i: &str) -> IResult<&str, RuleContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Rule String: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleRuleString",
            Self::trimmed(map(is_not("{}"), |p| RuleContent::String(String::from(p)))),
        )(i);

        #[cfg(test)]
        trace!("Rule String: {:#?}", result);

        result
    }

    /// Parse values within curly braces. This is basically just a helper for rules since
    /// they may contain braced content. This function is for parsing it all and not
    /// returning an incomplete rule at the first appearance of a closed curly brace
    fn rule_curly_braces(i: &str) -> IResult<&str, RuleContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Curly Braces: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleRuleCurlyBraces",
            Self::trimmed(map(
                delimited(
                    tag("{"),
                    many0(alt((Parser::rule_string, Parser::rule_curly_braces))),
                    tag("}"),
                ),
                RuleContent::CurlyBraces,
            )),
        )(i);

        #[cfg(test)]
        trace!("Curly Braces: {:#?}", result);

        result
    }

    /// Parse a style attribute such as "width: 10px"
    fn dangling_attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Dangling Attribute: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleAttribute",
            Self::trimmed(map(
                separated_pair(
                    // Key
                    preceded(
                        opt(Parser::cmt),
                        preceded(opt(Parser::sp), is_not(" \t\r\n:{")),
                    ),
                    // Separator
                    preceded(opt(Parser::cmt), preceded(opt(Parser::sp), tag(":"))),
                    // Value
                    preceded(
                        opt(Parser::cmt),
                        preceded(opt(Parser::sp), terminated(is_not(";{}"), tag(";"))),
                    ),
                ),
                move |p: (&str, &str)| -> StyleAttribute {
                    StyleAttribute {
                        key: p.0.trim().to_string(),
                        value: p.1.trim().to_string(),
                    }
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Dangling Attribute: {:#?}", result);

        result
    }

    /// Parse attributes outside of a { ... }.
    fn dangling_attributes(i: &str) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Dangling Attributes: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleAttributes",
            Self::trimmed(many1(Parser::dangling_attribute)),
        )(i);

        #[cfg(test)]
        trace!("Dangling Attributes: {:#?}", result);

        result
    }

    /// Parse anything that is not in a { ... }
    fn dangling_block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Dangling Block: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleDanglingBlock",
            Self::trimmed(map(
                Parser::dangling_attributes,
                |attr: Vec<StyleAttribute>| {
                    ScopeContent::Block(Block {
                        condition: None,
                        style_attributes: attr,
                    })
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Dangling Block: {:#?}", result);

        result
    }

    /// Parse the Content of a Scope
    fn scope_contents(i: &str) -> IResult<&str, Vec<ScopeContent>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Scope Contents: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "ScopeContents",
            Self::trimmed(many0(alt((
                // Either a dangling block
                Parser::dangling_block,
                // Or a Rule
                Parser::rule,
                // Or a Block
                Parser::block,
            )))),
        )(i);

        #[cfg(test)]
        trace!("Scope Contents: {:#?}", result);

        result
    }

    /// Parse a CSS Scope
    fn scope(i: &str) -> IResult<&str, Scope, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Scope: {}", i);

        // Cannot accept empty media.
        Self::expect_non_empty(i)?;

        let result = context(
            "StyleScope",
            Self::trimmed(map(Parser::scope_contents, |sc| Scope {
                condition: None,
                stylesets: sc,
            })),
        )(i);

        #[cfg(test)]
        trace!("Scope: {:#?}", result);
        result
    }

    /// Parse `@media`
    fn media_rule(i: &str) -> IResult<&str, Scope, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Media Rule: {}", i);

        // Cannot accept empty media.
        Self::expect_non_empty(i)?;

        let result = context(
            "MediaRule",
            Self::trimmed(map(
                separated_pair(
                    // Collect Media Rules.
                    preceded(tag("@media "), is_not("{")),
                    tag("{"),
                    // Collect contents with-in media rules.
                    terminated(Parser::scope_contents, tag("}")),
                ),
                // Map Results into a scope
                |p: (&str, Vec<ScopeContent>)| -> Scope {
                    Scope {
                        condition: Some(format!("@media {}", p.0.trim())),
                        stylesets: p.1,
                    }
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Media Rule: {:#?}", result);

        result
    }

    /// Parse scopes
    /// A Scope can be either a media rule or a css scope.
    fn scopes(i: &str) -> IResult<&str, Vec<Scope>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Scopes: {}", i);

        let result = context(
            "StyleScopes",
            // Drop trailing whitespaces.
            Self::trimmed(many0(alt(
                // Either @media
                (
                    Parser::media_rule,
                    // Or Scope
                    Parser::scope,
                ),
            ))),
        )(i);

        #[cfg(test)]
        trace!("Scopes: {:#?}", result);

        result
    }

    /// The parse the style and returns a `Result<Vec<Scope>>`.
    pub(crate) fn parse(css: &str) -> Result<Vec<Scope>> {
        match Self::scopes(css) {
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                Err(Error::Parse(convert_error(css, e)))
            }
            Err(nom::Err::Incomplete(e)) => Err(Error::Parse(format!("{:#?}", e))),
            Ok((_, res)) => Ok(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_empty_rule() {
        init();

        let test_str = r#""#;
        assert!(Parser::parse(test_str)
            .expect("Failed to Parse Style")
            .is_empty());
    }

    #[test]
    fn test_multiple_media_queries() -> Result<()> {
        init();

        let test_str = r#"
                @media screen and (max-width: 500px) {
                    background-color: red;
                }

                @media screen and (max-width: 200px) {
                    color: yellow;
                }

            "#;
        let parsed = Parser::parse(test_str)?;

        let expected = vec![
            Scope {
                condition: Some("@media screen and (max-width: 500px)".into()),
                stylesets: vec![ScopeContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }],
                })],
            },
            Scope {
                condition: Some("@media screen and (max-width: 200px)".into()),
                stylesets: vec![ScopeContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: "color".into(),
                        value: "yellow".into(),
                    }],
                })],
            },
        ];

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_media_query_then_normal_class() -> Result<()> {
        init();

        let test_str = r#"
                @media screen and (max-width: 500px) {
                    background-color: red;
                }

                .some-class2 {
                    color: yellow;
                }

            "#;
        let parsed = Parser::parse(test_str)?;

        let expected = vec![
            Scope {
                condition: Some("@media screen and (max-width: 500px)".into()),
                stylesets: vec![ScopeContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }],
                })],
            },
            Scope {
                condition: None,
                stylesets: vec![ScopeContent::Block(Block {
                    condition: Some(".some-class2".into()),
                    style_attributes: vec![StyleAttribute {
                        key: "color".into(),
                        value: "yellow".into(),
                    }],
                })],
            },
        ];

        assert_eq!(parsed, expected);

        Ok(())
    }
}
