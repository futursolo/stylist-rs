#[test]
fn test_literal_at_support() {
    let style = stylist::style! {
        r#"@supports (display:grid) {
            background-color: grey;
        }"#
    }
    .unwrap();
    let expected_result = format!(
        r#"@supports (display:grid) {{
    .{cls} {{
        background-color: grey;
    }}
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(expected_result, style.get_style_str());
}
