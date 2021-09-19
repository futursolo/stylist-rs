use std::borrow::Cow;
use std::fmt;

use nom::branch::{alt, Alt};
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, anychar, char, none_of};
use nom::combinator::{fail, map, not, opt, recognize};
use nom::error::{convert_error, ErrorKind, ParseError as NomParseError, VerboseError};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::IResult;

use crate::ast::{
    Block, Rule, RuleBlockContent, ScopeContent, Selector, Sheet, StringFragment, StyleAttribute,
};
use crate::bow::Bow;
use crate::{Error, Result};

mod error;
mod parse;
mod parse_stream;

pub use error::{ParseError, ParseResult};
pub use parse::Parse;
pub use parse_stream::ParseStream;

#[cfg(test)]
use log::trace;

#[derive(Debug, PartialEq)]
enum RuleBlockKind {
    Keyframes,
    Other,
}

/// Wrap a parser, tracing input and output.
// if not cfg(test), this would trip up clippy.
#[allow(clippy::let_and_return)]
fn traced_context<I, O, F>(
    ctx: &'static str,
    mut p: impl nom::Parser<I, O, F>,
) -> impl FnMut(I) -> IResult<I, O, F>
where
    I: fmt::Display + fmt::Debug + Clone,
    O: fmt::Debug,
    F: fmt::Debug + nom::error::ContextError<I>,
{
    use nom::error::context;
    #[cfg(test)]
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[cfg(test)]
    thread_local! {
        static NESTING_LEVEL: AtomicUsize = AtomicUsize::default();
    }

    move |i| {
        #[cfg(test)]
        let nesting_lvl = NESTING_LEVEL.with(|lvl| lvl.fetch_add(1, Ordering::Relaxed));
        #[cfg(test)]
        trace!("> {} {}: {}", nesting_lvl, ctx, i);

        let result = context(ctx, |i| p.parse(i))(i);

        #[cfg(test)]
        trace!("< {} {}: {:#?}", nesting_lvl, ctx, result);
        #[cfg(test)]
        NESTING_LEVEL.with(|lvl| lvl.fetch_sub(1, Ordering::Relaxed));

        result
    }
}

/// Returns Error when string is Empty
fn expect_non_empty<'a, O, F>(
    mut p: impl nom::Parser<&'a str, O, F>,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, F>
where
    F: NomParseError<&'a str>,
{
    move |i| {
        if i.is_empty() {
            Err(nom::Err::Error(NomParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )))
        } else {
            p.parse(i)
        }
    }
}

/// A lightweight CSS Parser.
#[derive(Debug)]
pub(crate) struct Parser {}

#[allow(clippy::let_and_return)]
impl Parser {
    /// Drop whitespaces
    fn trimmed<'a, O, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&str>>
    where
        F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
        O: std::fmt::Debug,
    {
        traced_context(
            "Trimmed",
            delimited(
                // Drop Preceeding whitespaces.
                Self::sp,
                // Parse until finishes
                f,
                // Drop Trailing whitespaces.
                Self::sp,
            ),
        )
    }

    /// Parse a comment
    ///
    /// token('/*') + anything but '*' followed by '/' + token("*/")
    fn cmt(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        traced_context(
            "Comment",
            delimited(
                tag("/*"),
                recognize(many0(alt((
                    is_not("*"),
                    terminated(tag("*"), not(char('/'))),
                )))),
                tag("*/"),
            ),
        )(i)
    }

    /// Parse whitespace
    fn sp(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        let chars = " \t\r\n";
        let whitespace = take_while1(move |c| chars.contains(c));
        traced_context("Whitespace", recognize(many0(alt((whitespace, Self::cmt)))))(i)
    }

    /// Parse an ident
    ///
    /// [\-_a-zA-Z(non-ascii)]{1}[\-_a-zA-Z0-9(non-ascii)]*
    fn ident(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        traced_context(
            "Ident",
            recognize(preceded(
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
            )),
        )(i)
    }

    fn style_attr_key(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        traced_context("StyleAttrKey", Self::trimmed(Self::ident))(i)
    }

    // TODO: Parse value properly.
    fn style_attr_value(i: &str) -> IResult<&str, StringFragment, VerboseError<&str>> {
        traced_context(
            "StyleAttrValue",
            Self::trimmed(map(
                recognize(many1(alt((
                    is_not("${;}/\""),
                    recognize(Self::interpolation),
                    Self::string,
                    recognize(preceded(tag("/"), none_of("${;}/\"*"))),
                )))),
                |m: &str| StringFragment {
                    inner: m.to_string().trim().to_string().into(),
                },
            )),
        )(i)
    }

    /// Parse a style attribute such as "width: 10px"
    fn attribute(i: &str) -> IResult<&str, StyleAttribute, VerboseError<&str>> {
        traced_context(
            "StyleAttribute",
            Self::trimmed(expect_non_empty(map(
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
            ))),
        )(i)
    }

    fn attributes(
        i: &str,
        dangling: bool,
    ) -> IResult<&str, Vec<StyleAttribute>, VerboseError<&str>> {
        let final_semicolon = |i| {
            if dangling {
                map(tag(";"), Some)(i)
            } else {
                opt(tag(";"))(i)
            }
        };

        traced_context(
            "StyleAttributes",
            Self::trimmed(expect_non_empty(terminated(
                separated_list1(tag(";"), Self::attribute),
                preceded(opt(Self::sp), final_semicolon),
            ))),
        )(i)
    }

    /// Parse a quoted string.
    ///
    // TODO: Parse ' quoted strings.
    fn string(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        let escaped_char = traced_context("EscapedChar", recognize(preceded(tag("\\"), anychar)));

        let parse_str = recognize(preceded(
            tag("\""),
            terminated(many0(alt((is_not(r#"\""#), escaped_char))), tag("\"")),
        ));

        traced_context("String", Self::trimmed(expect_non_empty(parse_str)))(i)
    }

    /// Parse a string interpolation.
    ///
    // TODO: Handle escaping.
    fn interpolation(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        traced_context(
            "Interpolation",
            Self::trimmed(expect_non_empty(delimited(
                tag("${"),
                Self::trimmed(recognize(preceded(
                    alpha1,
                    many0(alt((alphanumeric1, tag("_")))),
                ))),
                tag("}"),
            ))),
        )(i)
    }

    /// Parse a selector.
    ///
    // TODO: Parse selector properly.
    fn selector(i: &str) -> IResult<&str, Selector, VerboseError<&str>> {
        traced_context(
            "Selector",
            Self::trimmed(expect_non_empty(map(
                recognize(many1(alt((
                    recognize(preceded(none_of("$,}@{\""), opt(is_not("$,\"{")))),
                    Self::string,
                    recognize(Self::interpolation),
                )))),
                |p: &str| vec![p.trim().to_owned().into()].into(),
            ))),
        )(i)
    }

    /// Parse a selector or selector list.
    fn condition(i: &str) -> IResult<&str, Vec<Selector>, VerboseError<&str>> {
        traced_context(
            "Condition",
            Self::trimmed(expect_non_empty(many1(terminated(
                Self::selector,
                opt(tag(",")),
            )))),
        )(i)
    }

    fn block_contents(i: &str) -> IResult<&str, Vec<RuleBlockContent>, VerboseError<&str>> {
        traced_context(
            "BlockContents",
            Self::trimmed(expect_non_empty(map(
                many0(alt((
                    // Either Style Attributes
                    map(
                        |i| Parser::attributes(i, false),
                        |m| m.into_iter().map(RuleBlockContent::StyleAttr).collect(),
                    ),
                    // Or an at rule
                    map(
                        |i| Parser::rule_block(i, RuleBlockKind::Other),
                        |m| vec![RuleBlockContent::Rule(Bow::Boxed(Box::new(m)))],
                    ),
                ))),
                |m: Vec<Vec<RuleBlockContent>>| m.into_iter().flatten().collect(),
            ))),
        )(i)
    }

    /// Parse a [`Block`].
    fn block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        traced_context(
            "Block",
            Self::trimmed(expect_non_empty(map(
                pair(
                    Self::condition,
                    delimited(tag("{"), Self::trimmed(Self::block_contents), tag("}")),
                ),
                |p: (Vec<Selector>, Vec<RuleBlockContent>)| {
                    ScopeContent::Block(Block {
                        condition: p.0.into(),
                        content: p.1.into(),
                    })
                },
            ))),
        )(i)
    }

    fn rule_block_contents(i: &str) -> IResult<&str, Vec<RuleBlockContent>, VerboseError<&str>> {
        map(
            traced_context(
                "RuleBlockContents",
                expect_non_empty(many0(alt((
                    // Either Style Attributes
                    map(
                        |i| Parser::attributes(i, false),
                        |m: Vec<StyleAttribute>| {
                            m.into_iter().map(RuleBlockContent::StyleAttr).collect()
                        },
                    ),
                    // Or an at rule
                    map(
                        |i| Parser::rule_block(i, RuleBlockKind::Other),
                        |m: Rule| vec![RuleBlockContent::Rule(Bow::Boxed(Box::new(m)))],
                    ),
                )))),
            ),
            |m: Vec<Vec<RuleBlockContent>>| m.into_iter().flatten().collect(),
        )(i)
    }

    /// Parses a Rule Block
    fn rule_block(i: &str, kind: RuleBlockKind) -> IResult<&str, Rule, VerboseError<&str>> {
        let cond = |i| match kind {
            RuleBlockKind::Other => Self::at_rule_condition(i, (tag("@media"), tag("@supports"))),
            RuleBlockKind::Keyframes => map(recognize(Self::condition), |m| {
                vec![m.trim().to_string().into()]
            })(i),
        };

        traced_context(
            "RuleBlock",
            Self::trimmed(expect_non_empty(map(
                separated_pair(
                    // Collect at Rules.
                    cond,
                    tag("{"),
                    // Collect contents with-in rules.
                    terminated(Self::rule_block_contents, tag("}")),
                ),
                // Map Results into a scope
                |p: (Vec<StringFragment>, Vec<RuleBlockContent>)| Rule {
                    condition: p.0.into(),
                    content: p.1.into(),
                },
            ))),
        )(i)
    }

    /// Parse anything that is not in a { ... }
    fn dangling_block(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        traced_context(
            "DanglingBlock",
            Self::trimmed(expect_non_empty(map(
                |i| Self::attributes(i, true),
                |attr: Vec<StyleAttribute>| {
                    ScopeContent::Block(Block {
                        condition: Cow::Borrowed(&[]),
                        content: attr
                            .into_iter()
                            .map(|m| m.into())
                            .collect::<Vec<RuleBlockContent>>()
                            .into(),
                    })
                },
            ))),
        )(i)
    }

    /// Parse a CSS Scope
    fn scope(i: &str) -> IResult<&str, Vec<ScopeContent>, VerboseError<&str>> {
        traced_context(
            "Scope",
            Self::trimmed(expect_non_empty(Parser::scope_contents)),
        )(i)
    }

    fn at_rule_condition<'a, T>(
        i: &'a str,
        tags: T,
    ) -> IResult<&'a str, Vec<StringFragment>, VerboseError<&'a str>>
    where
        T: Alt<&'a str, &'a str, VerboseError<&'a str>>,
    {
        let tags = recognize(terminated(alt(tags), tag(" ")));

        traced_context(
            "AtRuleCondition",
            Self::trimmed(expect_non_empty(map(
                pair(
                    tags,
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
            ))),
        )(i)
    }

    fn keyframes(i: &str) -> IResult<&str, Rule, VerboseError<&str>> {
        traced_context(
            "Keyframes",
            Self::trimmed(map(
                separated_pair(
                    // Collect at Rules.
                    |i| Self::at_rule_condition(i, (tag("@keyframes"), fail)),
                    tag("{"),
                    // Collect contents with-in rules.
                    terminated(
                        many0(|i| Parser::rule_block(i, RuleBlockKind::Keyframes)),
                        tag("}"),
                    ),
                ),
                // Map Results into a scope
                |p: (Vec<StringFragment>, Vec<Rule>)| Rule {
                    condition: p.0.into(),
                    content: p
                        .1
                        .into_iter()
                        .map(|m| RuleBlockContent::Rule(Bow::Boxed(Box::new(m))))
                        .collect(),
                },
            )),
        )(i)
    }

    /// Parse `@supports` and `@media`
    fn at_rule(i: &str) -> IResult<&str, ScopeContent, VerboseError<&str>> {
        traced_context(
            "AtRule",
            Self::trimmed(expect_non_empty(map(
                separated_pair(
                    // Collect at Rules.
                    |i| Self::at_rule_condition(i, (tag("@supports"), tag("@media"))),
                    tag("{"),
                    // Collect contents with-in rules.
                    terminated(Parser::scope_contents, tag("}")),
                ),
                // Map Results into a scope
                |p: (Vec<StringFragment>, Vec<ScopeContent>)| {
                    ScopeContent::Rule(Rule {
                        condition: p.0.into(),
                        content: p
                            .1
                            .into_iter()
                            .map(|m| match m {
                                ScopeContent::Block(m) => {
                                    RuleBlockContent::Block(Bow::Boxed(Box::new(m)))
                                }
                                ScopeContent::Rule(m) => {
                                    RuleBlockContent::Rule(Bow::Boxed(Box::new(m)))
                                }
                            })
                            .collect(),
                    })
                },
            ))),
        )(i)
    }

    /// Parse the Content of a Scope
    fn scope_contents(i: &str) -> IResult<&str, Vec<ScopeContent>, VerboseError<&str>> {
        traced_context(
            "ScopeContents",
            Self::trimmed(expect_non_empty(many0(alt((
                // Either a dangling block
                Parser::dangling_block,
                // Or a Block
                Parser::block,
                // @supports and @media
                Parser::at_rule,
                // @keyframes
                map(Parser::keyframes, ScopeContent::Rule),
            ))))),
        )(i)
    }

    /// Parse sheet
    /// A Scope can be either an at rule or a css scope.
    fn sheet(i: &str) -> IResult<&str, Sheet, VerboseError<&str>> {
        traced_context(
            "StyleSheet",
            // Drop trailing whitespaces.
            Self::trimmed(map(many0(Self::scope), |p: Vec<Vec<ScopeContent>>| {
                Sheet::from(p.into_iter().flatten().collect::<Vec<ScopeContent>>())
            })),
        )(i)
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
    fn test_whitespace() {
        init();

        let test_str = " \n\n\t\t\t/* comment */rest";
        assert_eq!(
            Parser::sp(test_str).expect("Failed to parse whitespace"),
            ("rest", " \n\n\t\t\t/* comment */")
        )
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
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["red".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![".nested".into()].into()].into(),
                content: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["blue".into()].into(),
                    }
                    .into(),
                    StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    }
                    .into(),
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
                content: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["red".into()].into(),
                    }
                    .into(),
                    StyleAttribute {
                        key: "content".into(),
                        value: vec![r#"";""#.into()].into(),
                    }
                    .into(),
                ]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![r#"[placeholder="someone@example.com"]"#.into()].into()]
                    .into(),
                content: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["blue".into()].into(),
                    }
                    .into(),
                    StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    }
                    .into(),
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
            content: vec![
                StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }
                .into(),
                StyleAttribute {
                    key: "width".into(),
                    value: vec!["100px".into()].into(),
                }
                .into(),
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
            content: vec![StyleAttribute {
                key: "background-color".into(),
                value: vec!["#d0d0d9".into()].into(),
            }
            .into()]
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
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "background-color".into(),
                            value: vec!["red".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and (max-width: 200px)".into()].into(),
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "color".into(),
                            value: vec!["yellow".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
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
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "background-color".into(),
                            value: vec!["red".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![".some-class2".into()].into()].into(),
                content: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["yellow".into()].into(),
                }
                .into()]
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
                content: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["yellow".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["&".into()].into(), vec!["& input".into()].into()].into(),
                content: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["pink".into()].into(),
                }
                .into()]
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
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![
                            StyleAttribute {
                                key: "backdrop-filter".into(),
                                value: vec!["blur(2px)".into()].into(),
                            }
                            .into(),
                            StyleAttribute {
                                key: "-webkit-backdrop-filter".into(),
                                value: vec!["blur(2px)".into()].into(),
                            }
                            .into(),
                            StyleAttribute {
                                key: "background-color".into(),
                                value: vec!["rgb(0, 0, 0, 0.7)".into()].into(),
                            }
                            .into(),
                        ]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec![
                    "@supports ".into(),
                    "not ((backdrop-filter: blur(2px)) or (-webkit-backdrop-filter: blur(2px)))"
                        .into(),
                ]
                .into(),
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "background-color".into(),
                            value: vec!["rgb(25, 25, 25)".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
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
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["red".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![
                    vec![".nested".into()].into(),
                    vec!["${var_a}".into()].into(),
                ]
                .into(),
                content: vec![
                    StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["blue".into()].into(),
                    }
                    .into(),
                    StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    }
                    .into(),
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
            content: vec![].into(),
        })]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_dense_style() {
        init();
        let test_str = r#"@media print{color:black;.nested{cursor:none;}}"#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Rule(Rule {
            condition: vec!["@media ".into(), "print".into()].into(),
            content: vec![
                RuleBlockContent::Block(
                    Block {
                        condition: vec![].into(),
                        content: vec![StyleAttribute {
                            key: "color".into(),
                            value: vec!["black".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                ),
                RuleBlockContent::Block(
                    Block {
                        condition: vec![vec![".nested".into()].into()].into(),
                        content: vec![StyleAttribute {
                            key: "cursor".into(),
                            value: vec!["none".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                ),
            ]
            .into(),
        })]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_issue_36() {
        init();
        let test_str = r#"
  position: fixed; /* Stay in place */
  z-index: 1; /* Sit on top */
  width: 100%; /* Full width */
  height: 100%; /* Full height */
        "#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Block(Block {
            condition: vec![].into(),
            content: vec![
                StyleAttribute {
                    key: "position".into(),
                    value: vec!["fixed".into()].into(),
                }
                .into(),
                StyleAttribute {
                    key: "z-index".into(),
                    value: vec!["1".into()].into(),
                }
                .into(),
                StyleAttribute {
                    key: "width".into(),
                    value: vec!["100%".into()].into(),
                }
                .into(),
                StyleAttribute {
                    key: "height".into(),
                    value: vec!["100%".into()].into(),
                }
                .into(),
            ]
            .into(),
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
                content: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["${color}".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into(), vec!["${sel_div}".into()].into()]
                    .into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![":not(${sel_root})".into()].into()].into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["black".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "display".into(),
                            value: vec!["flex".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_comment() {
        init();
        let test_str = r#"
                /* some comment */ /* another comment */
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
                /* end comment */
            "#;
        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                content: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["${color}".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into(), vec!["${sel_div}".into()].into()]
                    .into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![":not(${sel_root})".into()].into()].into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["black".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "display".into(),
                            value: vec!["flex".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
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
                content: vec![StyleAttribute {
                    key: "color".into(),
                    value: vec!["\"$${color}\"".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into(), vec!["${sel_div}".into()].into()]
                    .into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["blue".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![":not(${sel_root})".into()].into()].into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["black".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "display".into(),
                            value: vec!["flex".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_rule_block() {
        let test_str = r#"
                span {
                    @media screen and (max-width: 500px) {
                        background-color: blue;
                    }
                }

                div {
                    @supports (max-width: 500px) {
                        @media screen and (max-width: 500px) {
                            background-color: blue;
                        }
                    }
                }

                @media screen and ${breakpoint} {
                    display: flex;
                }
            "#;

        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: vec![vec!["span".into()].into()].into(),
                content: vec![RuleBlockContent::Rule(
                    Rule {
                        condition: vec!["@media ".into(), "screen and (max-width: 500px)".into()]
                            .into(),
                        content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                            key: "background-color".into(),
                            value: vec!["blue".into()].into(),
                        })]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec!["div".into()].into()].into(),
                content: vec![RuleBlockContent::Rule(
                    Rule {
                        condition: vec!["@supports ".into(), "(max-width: 500px)".into()].into(),
                        content: vec![RuleBlockContent::Rule(
                            Rule {
                                condition: vec![
                                    "@media ".into(),
                                    "screen and (max-width: 500px)".into(),
                                ]
                                .into(),
                                content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                                    key: "background-color".into(),
                                    value: vec!["blue".into()].into(),
                                })]
                                .into(),
                            }
                            .into(),
                        )]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@media ".into(), "screen and ${breakpoint}".into()].into(),
                content: vec![RuleBlockContent::Block(
                    Block {
                        condition: Cow::Borrowed(&[]),
                        content: vec![StyleAttribute {
                            key: "display".into(),
                            value: vec!["flex".into()].into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into(),
                )]
                .into(),
            }),
        ]);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_slash_parsing() {
        let test_str = r#"
            grid-row: 1 / 3;
            grid-row: 1/3;
            grid-row: 1/ 3;
            grid-row: 1 /3;
        "#;

        let parsed = Parser::parse(test_str).expect("Failed to Parse Style");

        let expected = Sheet::from(vec![ScopeContent::Block(Block {
            condition: vec![].into(),
            content: vec![
                RuleBlockContent::StyleAttr(StyleAttribute {
                    key: "grid-row".into(),
                    value: vec!["1 / 3".into()].into(),
                }),
                RuleBlockContent::StyleAttr(StyleAttribute {
                    key: "grid-row".into(),
                    value: vec!["1/3".into()].into(),
                }),
                RuleBlockContent::StyleAttr(StyleAttribute {
                    key: "grid-row".into(),
                    value: vec!["1/ 3".into()].into(),
                }),
                RuleBlockContent::StyleAttr(StyleAttribute {
                    key: "grid-row".into(),
                    value: vec!["1 /3".into()].into(),
                }),
            ]
            .into(),
        })]);

        assert_eq!(parsed, expected);
    }
}
