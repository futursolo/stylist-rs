use crate::{Error, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::one_of,
    combinator::{map, map_res, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{fold_many0, many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use stylist_core::ast::{Block, Rule, RuleContent, ScopeContent, Sheet, StyleAttribute};

#[cfg(test)]
use log::trace;

/// Trait for types that can be parsed into css [`Sheets`].
///
/// [`Sheets`]: `stylist_core::ast::Sheet`
pub trait TryParseCss {
    /// Error type encountered during parsing
    type Error;
    /// Try to convert a self to a css [`Sheet`].
    fn try_parse(self) -> std::result::Result<Sheet, Self::Error>;
}

impl<'a> TryParseCss for &'a str {
    type Error = Error;

    fn try_parse(self) -> Result<Sheet> {
        crate::parser::Parser::parse(self)
    }
}

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

    fn rule_contents(i: &str) -> IResult<&str, Vec<RuleContent>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Rule contents: {}", i);

        Self::expect_non_empty(i)?;

        let string_as_contents = map(Parser::rule_string, |s| vec![s]);
        let string_or_curlies = alt((Parser::rule_curly_braces, string_as_contents));
        let result = context(
            "RuleContents",
            fold_many0(string_or_curlies, Vec::new(), |mut acc, item| {
                acc.extend(item);
                acc
            }),
        )(i)?;

        #[cfg(test)]
        trace!("Rule contents: {:#?}", result);

        Ok(result)
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
                    terminated(terminated(Self::rule_contents, opt(Parser::sp)), tag("}")),
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
    fn rule_curly_braces(i: &str) -> IResult<&str, Vec<RuleContent>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Curly Braces: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleRuleCurlyBraces",
            Self::trimmed(delimited(tag("{"), Self::rule_contents, tag("}"))),
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
    fn scope(i: &str) -> IResult<&str, Vec<ScopeContent>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Scope: {}", i);

        // Cannot accept empty media.
        Self::expect_non_empty(i)?;

        let result = context("StyleScope", Self::trimmed(Parser::scope_contents))(i);

        #[cfg(test)]
        trace!("Scope: {:#?}", result);
        result
    }

    /// Parse `@media`
    fn media_rule(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
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
                |mut p: (&str, Vec<ScopeContent>)| {
                    ScopeContent::Rule(Rule {
                        condition: format!("@media {}", p.0.trim()),
                        content: p.1.drain(..).map(|i| i.into()).collect(),
                    })
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Media Rule: {:#?}", result);

        result
    }

    /// Parse sheet
    /// A Scope can be either a media rule or a css scope.
    fn sheet(i: &str) -> IResult<&str, Sheet, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Sheet: {}", i);

        let media_rule = map(Self::media_rule, |s| vec![s]);
        let contents = alt((media_rule, Self::scope));
        let result = context(
            "StyleSheet",
            // Drop trailing whitespaces.
            Self::trimmed(map(
                fold_many0(contents, Vec::new(), |mut acc, item| {
                    acc.extend(item);
                    acc
                }),
                Sheet,
            )),
        )(i);

        #[cfg(test)]
        trace!("Sheet: {:#?}", result);

        result
    }

    /// The parse the style and returns a `Result<Sheet>`.
    pub(crate) fn parse(css: &str) -> Result<Sheet> {
        match Self::sheet(css) {
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
            .0
            .is_empty());
    }

    #[test]
    fn test_simple_example() {
        init();
        let test_str = r#"
            background-color: red;

            .nested {
                background-color: blue;
                width: 100px
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet(vec![
            ScopeContent::Block(Block {
                condition: None,
                style_attributes: vec![StyleAttribute {
                    key: "background-color".to_string(),
                    value: "red".to_string(),
                }],
            }),
            ScopeContent::Block(Block {
                condition: Some(".nested".into()),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".to_string(),
                        value: "blue".to_string(),
                    },
                    StyleAttribute {
                        key: "width".to_string(),
                        value: "100px".to_string(),
                    },
                ],
            }),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_rule_with_ampersand() {
        init();
        let test_str = r#"
            &:hover {
                background-color: #d0d0d9;
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet(vec![ScopeContent::Block(Block {
            condition: Some("&:hover".into()),
            style_attributes: vec![StyleAttribute {
                key: "background-color".to_string(),
                value: "#d0d0d9".to_string(),
            }],
        })]);
        assert_eq!(parsed, expected);
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

        let expected = Sheet(vec![
            ScopeContent::Rule(Rule {
                condition: "@media screen and (max-width: 500px)".into(),
                content: vec![RuleContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }],
                })],
            }),
            ScopeContent::Rule(Rule {
                condition: "@media screen and (max-width: 200px)".into(),
                content: vec![RuleContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: "color".into(),
                        value: "yellow".into(),
                    }],
                })],
            }),
        ]);

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

        let expected = Sheet(vec![
            ScopeContent::Rule(Rule {
                condition: "@media screen and (max-width: 500px)".into(),
                content: vec![RuleContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }],
                })],
            }),
            ScopeContent::Block(Block {
                condition: Some(".some-class2".into()),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: "yellow".into(),
                }],
            }),
        ]);

        assert_eq!(parsed, expected);

        Ok(())
    }
}
