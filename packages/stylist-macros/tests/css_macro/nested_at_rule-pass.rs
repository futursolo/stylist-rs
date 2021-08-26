fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let style = stylist::style! {
        .outer {
            @media print {
                background-color: grey;
            }
            @supports (display: grid) {
                margin: ${"2cm"};
            }
        }
    }
    .unwrap();
    let expected_reusult = format!(
        r#"@media print {{
.{cls} .outer {{
background-color: grey;
}}
}}
@supports (display:grid) {{
.{cls} .outer {{
margin: 2cm;
}}
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(expected_reusult, style.get_style_str());
}
