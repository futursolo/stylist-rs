use std::borrow::Cow;

use stylist::ast::*;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_sheet_interpolation() {
    init();

    let parsed = sheet!(
        r#"
            background-color: red;

            .nested, ${var_a} {
                background-color: blue;
                width: ${size}px;
            }

            @keyframes myframe {
                from {
                    width: 100px;
                }
                to {
                    width: 200px;
                }
            }

            @media screen and ${breakpoint} {
                background-color: brown;
            }
        "#,
        var_a = ".some-selector",
        breakpoint = "(max-width: 500px)",
        size = 100,
    );

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
                vec![".some-selector".into()].into(),
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
                    value: vec!["100".into(), "px".into()].into(),
                }
                .into(),
            ]
            .into(),
        }),
        ScopeContent::Rule(Rule {
            condition: vec!["@keyframes myframe".into()].into(),
            content: vec![
                RuleBlockContent::Rule(Box::new(Rule {
                    condition: vec!["from".into()].into(),
                    content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                        key: "width".into(),
                        value: vec!["100px".into()].into(),
                    })]
                    .into(),
                })),
                RuleBlockContent::Rule(Box::new(Rule {
                    condition: vec!["to".into()].into(),
                    content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                        key: "width".into(),
                        value: vec!["200px".into()].into(),
                    })]
                    .into(),
                })),
            ]
            .into(),
        }),
        ScopeContent::Rule(Rule {
            condition: vec!["@media screen and ".into(), "(max-width: 500px)".into()].into(),
            content: vec![RuleBlockContent::Block(Box::new(Block {
                condition: vec![].into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["brown".into()].into(),
                }
                .into()]
                .into(),
            }))]
            .into(),
        }),
    ]);
    assert_eq!(parsed, expected);
}

#[test]
fn test_sheet_escaped() {
    let parsed = sheet!(
        r#"
            .nested, "$${var_a}" {
                content: "$${var_b}";
            }
        "#,
    );

    let expected = Sheet::from(vec![ScopeContent::Block(Block {
        condition: vec![
            vec![".nested".into()].into(),
            Selector {
                fragments: vec!["\"${var_a}\"".into()].into(),
            },
        ]
        .into(),
        content: vec![StyleAttribute {
            key: "content".into(),
            value: vec!["\"${var_b}\"".into()].into(),
        }
        .into()]
        .into(),
    })]);
    assert_eq!(parsed, expected);
}
