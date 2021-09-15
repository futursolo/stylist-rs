fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let style = stylist::style! {
        @supports (display: grid) {
            background-color: grey;
        }
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
