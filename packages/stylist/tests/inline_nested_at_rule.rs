#[test]
fn test_nested_at_rule() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dynamic_value = "blue";
    let style = stylist::style! {
        @supports (display: grid) {
            @media print {
                background-color: grey;
            }
            @media print {
                color: ${dynamic_value};
            }
        }
    }
    .unwrap();
    let expected_reusult = format!(
        r#"@supports (display:grid) {{
    @media print {{
        .{cls} {{
            background-color: grey;
        }}
    }}
}}
@supports (display:grid) {{
    @media print {{
        .{cls} {{
            color: blue;
        }}
    }}
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(expected_reusult, style.get_style_str());
}
