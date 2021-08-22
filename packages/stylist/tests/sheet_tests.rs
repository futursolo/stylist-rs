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
            "#,
        var_a = ".some-selector",
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
    ]);
    assert_eq!(parsed, expected);
}
