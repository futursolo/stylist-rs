use std::borrow::Cow;

use stylist::ast::*;

#[test]
fn test_sheet_interpolation() {
    let parsed = sheet!(
        r#"
            background-color: red;

            .nested, ${var_a} {
                background-color: blue;
                width: 100px
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
    );

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
            condition: vec![".nested".into(), ".some-selector".into()].into(),
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
        ScopeContent::Rule(Rule {
            condition: vec!["@keyframes myframe".into()].into(),
            content: vec![
                "from".into(),
                "{".into(),
                "width: 100px;".into(),
                "}".into(),
                "to".into(),
                "{".into(),
                "width: 200px;".into(),
                "}".into(),
            ]
            .into(),
        }),
        ScopeContent::Rule(Rule {
            condition: vec![
                "@media ".into(),
                "screen and ".into(),
                "(max-width: 500px)".into(),
            ]
            .into(),
            content: vec![RuleContent::Block(Block {
                condition: vec![].into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: "brown".into(),
                }]
                .into(),
            })]
            .into(),
        }),
    ]);
    assert_eq!(parsed, expected);
}
