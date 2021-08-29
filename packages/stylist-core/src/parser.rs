use std::borrow::Cow;

use crate::{
    ast::{
        Block, Rule, RuleContent, ScopeContent, Selector, Sheet, StringFragment, StyleAttribute,
    },
    Error, Result,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while, take_while1},
    character::complete::{alpha1, alphanumeric1, anychar, char, none_of},
    combinator::{map, map_res, not, opt, recognize},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

#[cfg(test)]
use log::trace;

/// A lightweight CSS Parser.
#[derive(Debug)]
pub(crate) struct Parser {}

#[allow(clippy::let_and_return)]
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
        context("Whitespace", take_while(move |c| chars.contains(c)))(i)
    }

    /// Drop whitespaces
    fn trimmed<'a, F, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&str>>
    where
        F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
    {
        context(
            "Trimmed",
            delimited(
                // Drop Preceeding whitespaces.
                opt(Self::sp),
                // Parse until finishes
                f,
                // Drop Trailing whitespaces.
                opt(Self::sp),
            ),
        )
    }

    /// Parse a comment
    ///
    /// token('/*') + anything but '*' followed by '/' + token("*/")
    fn cmt(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Comment: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Comment",
            Self::trimmed(delimited(
                tag("/*"),
                recognize(many0(alt((
                    is_not("*"),
                    terminated(tag("*"), not(char('/'))),
                )))),
                tag("*/"),
            )),
        )(i);

        #[cfg(test)]
        trace!("Comment: {:#?}", result);

        result
    }

    /// Drop comments
    fn trim_cmt<'a, F, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&str>>
    where
        F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
    {
        context(
            "TrimmedComments",
            delimited(
                // Drop Preceeding comments.
                opt(Self::cmt),
                // Parse until finishes
                f,
                // Drop Trailing comments.
                opt(Self::cmt),
            ),
        )
    }

    /// Parse an ident
    ///
    /// [\-_a-zA-Z(non-ascii)]{1}[\-_a-zA-Z0-9(non-ascii)]*
    fn ident(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Ident: {}", i);

        let result = recognize(preceded(
            alt((
                tag("-"),
                tag("_"),
                alpha1,
                take_while1(|m: char| !m.is_ascii()),
            )),
            many0(alt((
                tag("-"),
                tag("_"),
                alphanumeric1,
                take_while1(|m: char| !m.is_ascii()),
            ))),
        ))(i);

        #[cfg(test)]
        trace!("Ident: {:#?}", result);

        result
    }

    fn style_attr_key(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Style Attribute Key: {}", i);

        let result = context(
            "StyleAttrKey",
            Self::trimmed(Self::trim_cmt(Self::trimmed(Self::ident))),
        )(i);

        #[cfg(test)]
        trace!("Style Attribute Key: {:#?}", result);

        result
    }

    // TODO: Parse value properly.
    fn style_attr_value(i: &str) -> IResult<&str, StringFragment, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Style Attribute Value: {}", i);

        let result = context(
            "StyleAttrValue",
            Self::trimmed(Self::trim_cmt(Self::trimmed(map(
                recognize(many1(alt((
                    is_not("${;}/\""),
                    recognize(Self::interpolation),
                    Self::string,
                )))),
                |m: &str| StringFragment {
                    inner: m.to_string().trim().to_string().into(),
                },
            )))),
        )(i);

        #[cfg(test)]
        trace!("Style Attribute Value: {:#?}", result);

        result
    }

    /// Parse a style attribute such as "width: 10px;"
    fn dangling_attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Dangling Attribute: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleAttribute",
            Self::trimmed(map(
                separated_pair(
                    // Key
                    Self::style_attr_key,
                    // Separator
                    tag(":"),
                    // Value
                    terminated(Self::style_attr_value, tag(";")),
                ),
                move |p: (&str, StringFragment)| -> StyleAttribute {
                    StyleAttribute {
                        key: p.0.trim().to_string().into(),
                        value: vec![p.1].into(),
                    }
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Dangling Attribute: {:#?}", result);

        result
    }

    /// Parse a style attribute such as "width: 10px"
    fn attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Attribute: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleAttribute",
            Self::trimmed(map(
                separated_pair(
                    // Key
                    Self::style_attr_key,
                    // Separator
                    tag(":"),
                    Self::style_attr_value,
                ),
                move |p: (&str, StringFragment)| StyleAttribute {
                    key: p.0.trim().to_string().into(),
                    value: vec![p.1].into(),
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Attribute: {:#?}", result);

        result
    }

    /// Parse attributes outside of a { ... }.
    fn dangling_attributes(i: &str) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Dangling Attributes: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleAttributes",
            Self::trimmed(many1(Self::dangling_attribute)),
        )(i);

        #[cfg(test)]
        trace!("Dangling Attributes: {:#?}", result);

        result
    }

    fn attributes(i: &str) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Attributes: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleAttributes",
            Self::trimmed(terminated(
                separated_list0(tag(";"), Self::attribute),
                preceded(opt(Self::sp), opt(tag(";"))),
            )),
        )(i);

        #[cfg(test)]
        trace!("Attributes: {:#?}", result);

        result
    }

    /// Parse a quoted string.
    ///
    // TODO: Parse ' quoted strings.
    fn string(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("String: {}", i);

        Self::expect_non_empty(i)?;

        let escaped_char = context("EscapedChar", recognize(preceded(tag("\\"), anychar)));

        let parse_str = recognize(preceded(
            tag("\""),
            terminated(many0(alt((is_not(r#"\""#), escaped_char))), tag("\"")),
        ));

        let result = context("String", Self::trimmed(parse_str))(i);

        #[cfg(test)]
        trace!("String: {:#?}", result);

        result
    }

    /// Parse a string interpolation.
    ///
    // TODO: Handle escaping.
    fn interpolation(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Interpolation: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Interpolation",
            Self::trimmed(delimited(
                tag("${"),
                Self::trimmed(recognize(preceded(
                    alpha1,
                    many0(alt((alphanumeric1, tag("_")))),
                ))),
                tag("}"),
            )),
        )(i);

        #[cfg(test)]
        trace!("Interpolation: {:#?}", result);

        result
    }

    /// Parse a selector.
    ///
    // TODO: Parse selector properly.
    fn selector(i: &str) -> IResult<&str, Selector, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Selector: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Selector",
            Self::trimmed(map(
                recognize(many1(alt((
                    recognize(preceded(none_of("$,}@{\""), opt(is_not("$,\"{")))),
                    Self::string,
                    recognize(Self::interpolation),
                )))),
                |p: &str| vec![p.trim().to_owned().into()].into(),
            )),
        )(i);

        #[cfg(test)]
        trace!("Selector: {:#?}", result);

        result
    }

    /// Parse a selector or selector list.
    fn condition(i: &str) -> IResult<&str, Vec<Selector>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Condition: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Condition",
            Self::trimmed(many1(terminated(Self::selector, opt(tag(","))))),
        )(i);

        #[cfg(test)]
        trace!("Condition: {:#?}", result);

        result
    }

    /// Parse a [`Block`].
    fn block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Block: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "StyleBlock",
            Self::trimmed(map(
                separated_pair(
                    Self::condition,
                    tag("{"),
                    terminated(Self::trim_cmt(Self::attributes), tag("}")),
                ),
                |p: (Vec<Selector>, Vec<StyleAttribute>)| {
                    ScopeContent::Block(Block {
                        condition: p.0.into(),
                        style_attributes: p.1.into(),
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
            map(many0(string_or_curlies), |p: Vec<Vec<RuleContent>>| {
                p.into_iter().flatten().collect()
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
                    recognize(preceded(tag("@"), is_not("{"))),
                    tag("{"),
                    terminated(terminated(Self::rule_contents, opt(Parser::sp)), tag("}")),
                ),
                |p: (&str, Vec<RuleContent>)| {
                    if p.0.starts_with("@media") {
                        return Err(String::from("Not a media query"));
                    }

                    if p.0.starts_with("@supports") {
                        return Err(String::from("Not a support at rule"));
                    }

                    Ok(ScopeContent::Rule(Rule {
                        condition: vec![p.0.trim().to_string().into()].into(),
                        content: p.1.into(),
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
            Self::trimmed(map(is_not("{}"), |p: &str| {
                RuleContent::String(p.trim().to_string().into())
            })),
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
            Self::trimmed(map(
                delimited(tag("{"), Self::rule_contents, tag("}")),
                |mut m: Vec<RuleContent>| {
                    m.insert(0, RuleContent::String("{".to_string().into()));
                    m.push(RuleContent::String("}".to_string().into()));
                    m
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Curly Braces: {:#?}", result);

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
                Self::dangling_attributes,
                |attr: Vec<StyleAttribute>| {
                    ScopeContent::Block(Block {
                        condition: Cow::Borrowed(&[]),
                        style_attributes: attr.into(),
                    })
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Dangling Block: {:#?}", result);

        result
    }

    /// Parse a CSS Scope
    fn scope(i: &str) -> IResult<&str, Vec<ScopeContent>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Scope: {}", i);

        Self::expect_non_empty(i)?;

        let result = context("StyleScope", Self::trimmed(Parser::scope_contents))(i);

        #[cfg(test)]
        trace!("Scope: {:#?}", result);
        result
    }

    fn at_rule_condition(i: &str) -> IResult<&str, Vec<StringFragment>, VerboseError<&str>> {
        #[cfg(test)]
        trace!("At Rule: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "AtRule",
            Self::trimmed(map(
                pair(
                    alt((tag("@supports "), tag("@media "))),
                    map(
                        recognize(many1(alt((is_not("${"), recognize(Self::interpolation))))),
                        |m: &str| StringFragment {
                            inner: m.trim().to_string().into(),
                        },
                    ),
                ),
                |p: (&str, StringFragment)| {
                    vec![
                        StringFragment {
                            inner: p.0.to_string().into(),
                        },
                        p.1,
                    ]
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("At Rule: {:#?}", result);

        result
    }

    /// Parse `@supports` and `@media`
    fn at_rule(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("At Rule: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "AtRule",
            Self::trimmed(map(
                separated_pair(
                    // Collect at Rules.
                    Self::at_rule_condition,
                    tag("{"),
                    // Collect contents with-in rules.
                    terminated(Parser::scope_contents, tag("}")),
                ),
                // Map Results into a scope
                |mut p: (Vec<StringFragment>, Vec<ScopeContent>)| {
                    ScopeContent::Rule(Rule {
                        condition: p.0.into(),
                        content: p.1.drain(..).map(|i| i.into()).collect(),
                    })
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("At Rule: {:#?}", result);

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
                // Or a Block
                Parser::block,
                // Or an at rule
                Parser::at_rule,
                // Or a Rule
                Parser::rule,
            )))),
        )(i);

        #[cfg(test)]
        trace!("Scope Contents: {:#?}", result);

        result
    }

    /// Parse sheet
    /// A Scope can be either an at rule or a css scope.
    fn sheet(i: &str) -> IResult<&str, Sheet, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Sheet: {}", i);

        let result = context(
            "StyleSheet",
            // Drop trailing whitespaces.
            Self::trimmed(map(many0(Self::scope), |p: Vec<Vec<ScopeContent>>| {
                Sheet::from(p.into_iter().flatten().collect::<Vec<ScopeContent>>())
            })),
        )(i);

        #[cfg(test)]
        trace!("Sheet: {:#?}", result);

        result
    }

    /// The parse the style and returns a `Result<Sheet>`.
    pub fn parse(css: &str) -> Result<Sheet> {
        match Self::sheet(css) {
            // Converting to String, primarily due to lifetime requirements.
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(Error::Parse {
                reason: convert_error(css, e.clone()),
                source: Some(VerboseError {
                    errors: e
                        .errors
                        .into_iter()
                        .map(|(i, e)| (i.to_string(), e))
                        .collect(),
                }),
            }),
            Err(nom::Err::Incomplete(e)) => Err(Error::Parse {
                reason: format!("{:#?}", e),
                source: None,
            }),
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
    fn test_simple_example() {
        init();
        let test_str = r#"
            background-color: red;

            .nested {
                background-color: blue;
                width: 100px;
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["red".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![".nested".into()].into()].into(),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["blue".into()].into(),
                    },
                    StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    },
                ]
                .into(),
            }),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_simple_selector_with_at() {
        init();

        let test_str = r#"
            background-color: red;
            content: ";";

            [placeholder="someone@example.com"] {
                background-color: blue;
                width: 100px;
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["red".into()].into(),
                    },
                    StyleAttribute {
                        key: "content".into(),
                        value: vec![r#"";""#.into()].into(),
                    },
                ]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![r#"[placeholder="someone@example.com"]"#.into()].into()]
                    .into(),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["blue".into()].into(),
                    },
                    StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    },
                ]
                .into(),
            }),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_simple_escape() {
        init();

        let test_str = r#"
            [placeholder="\" {}"] {
                background-color: blue;
                width: 100px;
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Block(Block {
            condition: vec![vec![r#"[placeholder="\" {}"]"#.into()].into()].into(),
            style_attributes: vec![
                StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                },
                StyleAttribute {
                    key: "width".into(),
                    value: vec!["100px".into()].into(),
                },
            ]
            .into(),
        })]);
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

        let expected = Sheet::from(vec![ScopeContent::Block(Block {
            condition: vec![vec!["&:hover".into()].into()].into(),
            style_attributes: vec![StyleAttribute {
                key: "background-color".into(),
                value: vec!["#d0d0d9".into()].into(),
            }]
            .into(),
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

        let expected = Sheet::from(vec![
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and (max-width: 500px)".into()].into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["red".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and (max-width: 200px)".into()].into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "color".into(),
                        value: vec!["yellow".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
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

        let expected = Sheet::from(vec![
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and (max-width: 500px)".into()].into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["red".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![".some-class2".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["yellow".into()].into(),
                }]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_selector_list() -> Result<()> {
        init();

        let test_str = r#"
                div, span {
                    color: yellow;
                }

                &, & input {
                    color: pink;
                }

            "#;
        let parsed = Parser::parse(test_str)?;

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: vec![vec!["div".into()].into(), vec!["span".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["yellow".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["&".into()].into(), vec!["& input".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["pink".into()].into(),
                }]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_supports_rule() -> Result<()> {
        init();

        let test_str = r#"
                @supports (backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px)) {
                    backdrop-filter: blur(2px);
                    -webkit-backdrop-filter: blur(2px);
                    background-color: rgb(0, 0, 0, 0.7);
                }

                @supports not ((backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px))) {
                    background-color: rgb(25, 25, 25);
                }

            "#;
        let parsed = Parser::parse(test_str)?;

        let expected = Sheet::from(vec![
            ScopeContent::Rule(Rule {
                condition: vec![
                    "@supports ".into(),
                    "(backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px))".into(),
                ]
                .into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![
                        StyleAttribute {
                            key: "backdrop-filter".into(),
                            value: vec!["blur(2px)".into()].into(),
                        },
                        StyleAttribute {
                            key: "-webkit-backdrop-filter".into(),
                            value: vec!["blur(2px)".into()].into(),
                        },
                        StyleAttribute {
                            key: "background-color".into(),
                            value: vec!["rgb(0, 0, 0, 0.7)".into()].into(),
                        },
                    ]
                    .into(),
                })]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec![
                    "@supports ".into(),
                    "not ((backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px)))"
                        .into(),
                ]
                .into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["rgb(25, 25, 25)".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_selectors_list_2() {
        init();
        assert_eq!(
            Parser::selector("&").map(|m| m.1),
            Ok(vec!["&".into()].into())
        );
        assert_eq!(
            Parser::selector("& input").map(|m| m.1),
            Ok(vec!["& input".into()].into())
        );
    }

    #[test]
    fn test_interpolation() {
        init();
        let test_str = r#"
            background-color: red;

            .nested, ${var_a} {
                background-color: blue;
                width: 100px;
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["red".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![
                    vec![".nested".into()].into(),
                    vec!["${var_a}".into()].into(),
                ]
                .into(),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["blue".into()].into(),
                    },
                    StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    },
                ]
                .into(),
            }),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_empty_block() {
        init();
        let test_str = r#".nested {}"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Block(Block {
            condition: vec![vec![".nested".into()].into()].into(),
            style_attributes: vec![].into(),
        })]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_empty_media_rule() {
        init();
        let test_str = r#"@media screen and (max-width: 500px) {}"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Rule(Rule {
            condition: vec!["@media ".into(), "screen and (max-width: 500px)".into()].into(),
            content: vec![].into(),
        })]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_empty() {
        init();
        let test_str = r#""#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_pseudo_sel() {
        init();
        let test_str = r#"
                color: ${color};

                span, ${sel_div} {
                    background-color: blue;
                }

                :not(${sel_root}) {
                    background-color: black;
                }

                @media screen and ${breakpoint} {
                    display: flex;
                }
            "#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["${color}".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into(), vec!["${sel_div}".into()].into()]
                    .into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![":not(${sel_root})".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["black".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "display".into(),
                        value: vec!["flex".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_comment() {
        init();
        let test_str = r#"
                /* some comment */
                color: /**/${color};

                span, ${sel_div} {
                    /* comment before block */
                    background-color /* comment before colon */ : blue /* comment after attribute */;
                    /* comment after block */
                }

                :not(${sel_root}) {
                    background-color: black;
                }

                @media screen and ${breakpoint} {
                    /* a comment with * */
                    display: flex;
                }
            "#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["${color}".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into(), vec!["${sel_div}".into()].into()]
                    .into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![":not(${sel_root})".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["black".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "display".into(),
                        value: vec!["flex".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_pseudo_sel_escaped() {
        init();
        let test_str = r#"
                color: "$${color}";

                span, ${sel_div} {
                    background-color: blue;
                }

                :not(${sel_root}) {
                    background-color: black;
                }

                @media screen and ${breakpoint} {
                    display: flex;
                }
            "#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["\"$${color}\"".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into(), vec!["${sel_div}".into()].into()]
                    .into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![":not(${sel_root})".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["black".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "display".into(),
                        value: vec!["flex".into()].into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);
    }
}
