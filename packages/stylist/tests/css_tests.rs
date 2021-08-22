use stylist::*;

#[test]
fn test_sheet_interpolation() {
    let parsed = css!(
        r#"
            color: ${color};

            span, ${sel_div} {
                background-color: blue;
            }

            @media screen and ${breakpoint} {
                display: flex;
            }
        "#,
        color = "red",
        sel_div = "div.selected",
        breakpoint = "(max-width: 500px)",
    );

    let style = parsed.to_style();

    let expected = format!(
        r#".{cls} {{
color: red;
}}
.{cls} span, .{cls} div.selected {{
background-color: blue;
}}
@media screen and (max-width: 500px) {{
.{cls} {{
display: flex;
}}
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(style.get_style_str(), expected);
}
