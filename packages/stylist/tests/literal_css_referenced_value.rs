#[test]
fn test_sheet_value_by_ref() {
    struct Theme {
        color: String,
    }

    let theme = Theme {
        color: "red".into(),
    };

    let theme = &theme;

    use stylist::*;
    let parsed = css!(
        r#"
            color: ${color};
        "#,
        color = theme.color,
    );

    let style = Style::new(parsed).expect("Failed to parse style.");

    let expected = format!(
        r#".{cls} {{
    color: red;
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(style.get_style_str(), expected);
}
