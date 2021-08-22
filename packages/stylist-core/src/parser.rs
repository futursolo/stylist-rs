use std::borrow::Cow;

use crate::ast::{
    Block, Rule, RuleContent, ScopeContent, Selector, Sheet, StringKind, StyleAttribute,
};
use crate::{Error, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::{alpha1, alphanumeric1, anychar, none_of, one_of},
    combinator::{map, map_res, opt, recognize},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{fold_many0, many0, many1, separated_list0},
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
    fn cmt(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Comment: {}", i);

        let result = context(
            "StyleComment",
            Self::trimmed(delimited(
                tag("/*"),
                // not(tag("*/")), // TODO check for the string
                is_not("*"),
                tag("*/"),
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
                    key: p.0.trim().to_string().into(),
                    value: p.1.trim().to_string().into(),
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

    /// Parse a quoted string.
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

    /// Parse a selector interpolation.
    fn selector_interpolation(i: &str) -> IResult<&str, Selector, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Selector Interpolation: {}", i);

        let result = context(
            "SelectorInterpolation",
            Self::trimmed(map(
                delimited(
                    tag("${"),
                    Self::trimmed(recognize(preceded(
                        alpha1,
                        many0(alt((alphanumeric1, tag("_")))),
                    ))),
                    tag("}"),
                ),
                |p: &str| Selector {
                    inner: p.trim().to_owned().into(),
                    kind: StringKind::Interpolation,
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("Selector Interpolation: {:#?}", result);

        result
    }

    /// Parse a selector.
    fn selector(i: &str) -> IResult<&str, Selector, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Selector: {}", i);

        let result = context(
            "Selector",
            Self::trimmed(map(
                recognize(preceded(
                    none_of("$,}@{"),
                    many0(alt((is_not(",\"{"), Self::string))),
                )),
                |p: &str| p.trim().to_owned().into(),
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
            Self::trimmed(many1(terminated(
                alt((Self::selector, Self::selector_interpolation)),
                opt(tag(",")),
            ))),
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
                    terminated(terminated(Parser::attributes, opt(Parser::sp)), tag("}")),
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
                        condition: p.0.trim().to_string().into(),
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
                RuleContent::String(p.to_string().into())
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
            Self::trimmed(delimited(tag("{"), Self::rule_contents, tag("}"))),
        )(i);

        #[cfg(test)]
        trace!("Curly Braces: {:#?}", result);

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
                        key: p.0.trim().to_string().into(),
                        value: p.1.trim().to_string().into(),
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

    /// Parse `@supports` and `@media`
    fn at_rule(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        #[cfg(test)]
        trace!("At Rule: {}", i);

        // Cannot accept empty rule.
        Self::expect_non_empty(i)?;

        let result = context(
            "AtRule",
            Self::trimmed(map(
                separated_pair(
                    // Collect at Rules.
                    recognize(preceded(
                        alt((tag("@supports "), tag("@media "))),
                        is_not("{"),
                    )),
                    tag("{"),
                    // Collect contents with-in supports rules.
                    terminated(Parser::scope_contents, tag("}")),
                ),
                // Map Results into a scope
                |mut p: (&str, Vec<ScopeContent>)| {
                    ScopeContent::Rule(Rule {
                        condition: p.0.trim().to_string().into(),
                        content: p.1.drain(..).map(|i| i.into()).collect(),
                    })
                },
            )),
        )(i);

        #[cfg(test)]
        trace!("At Rule: {:#?}", result);

        result
    }

    /// Parse sheet
    /// A Scope can be either an at rule or a css scope.
    fn sheet(i: &str) -> IResult<&str, Sheet, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Sheet: {}", i);

        let at_rule = map(Self::at_rule, |s| vec![s]);
        let contents = alt((at_rule, Self::scope));
        let result = context(
            "StyleSheet",
            // Drop trailing whitespaces.
            Self::trimmed(map(many0(contents), |p: Vec<Vec<ScopeContent>>| {
                Sheet::from(p.into_iter().flatten().collect::<Vec<ScopeContent>>())
            })),
        )(i);

        #[cfg(test)]
        trace!("Sheet: {:#?}", result);

        result
    }

    /// The parse the style and returns a `Result<Sheet>`.
    pub(crate) fn parse(css: &str) -> Result<Sheet> {
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
                width: 100px
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: "red".into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![".nested".into()].into(),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: "blue".into(),
                    },
                    StyleAttribute {
                        key: "width".into(),
                        value: "100px".into(),
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

            [placeholder="someone@example.com"] {
                background-color: blue;
                width: 100px
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: "red".into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![r#"[placeholder="someone@example.com"]"#.into()].into(),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: "blue".into(),
                    },
                    StyleAttribute {
                        key: "width".into(),
                        value: "100px".into(),
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
                width: 100px
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Block(Block {
            condition: vec![r#"[placeholder="\" {}"]"#.into()].into(),
            style_attributes: vec![
                StyleAttribute {
                    key: "background-color".into(),
                    value: "blue".into(),
                },
                StyleAttribute {
                    key: "width".into(),
                    value: "100px".into(),
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
            condition: vec!["&:hover".into()].into(),
            style_attributes: vec![StyleAttribute {
                key: "background-color".into(),
                value: "#d0d0d9".into(),
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
                condition: "@media screen and (max-width: 500px)".into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: "@media screen and (max-width: 200px)".into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "color".into(),
                        value: "yellow".into(),
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
                condition: "@media screen and (max-width: 500px)".into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }]
                    .into(),
                })]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![".some-class2".into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: "yellow".into(),
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
                condition: vec!["div".into(), "span".into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: "yellow".into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec!["&".into(), "& input".into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "color".into(),
                    value: "pink".into(),
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
                condition:
                    "@supports (backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px))"
                        .into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![
                        StyleAttribute {
                            key: "backdrop-filter".into(),
                            value: "blur(2px)".into(),
                        },
                        StyleAttribute {
                            key: "-webkit-backdrop-filter".into(),
                            value: "blur(2px)".into(),
                        },
                        StyleAttribute {
                            key: "background-color".into(),
                            value: "rgb(0, 0, 0, 0.7)".into(),
                        }
                    ].into(),
                })].into(),
            }),

            ScopeContent::Rule(Rule {
                condition:
                    "@supports not ((backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px)))"
                        .into(),
                content: vec![RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "rgb(25, 25, 25)".into(),
                    }].into(),
                })].into(),
            }),
        ]);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_selectors_list_2() {
        init();
        assert_eq!(Parser::selector("&").map(|m| m.1), Ok("&".into()));
        assert_eq!(
            Parser::selector("& input").map(|m| m.1),
            Ok("& input".into())
        );
    }

    #[test]
    fn test_interpolation() {
        init();
        let test_str = r#"
            background-color: red;

            .nested, ${var_a} {
                background-color: blue;
                width: 100px
            }"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: "red".into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![
                    ".nested".into(),
                    Selector {
                        inner: "var_a".into(),
                        kind: StringKind::Interpolation,
                    },
                ]
                .into(),
                style_attributes: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: "blue".into(),
                    },
                    StyleAttribute {
                        key: "width".into(),
                        value: "100px".into(),
                    },
                ]
                .into(),
            }),
        ]);
        assert_eq!(parsed, expected);
    }
}
