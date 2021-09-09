#[test]
fn test_sheet_interpolation() {
    use stylist::ast::ToStyleStr;
    use stylist::*;
    let parsed = css!(
        r#"
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
        "#,
        color = "red",
        sel_div = "div.selected",
        sel_root = "&.highlighted",
        breakpoint = "(max-width: 500px)",
    );

    let sheet = parsed.try_to_sheet().expect("Failed to parse style.");

    let expected = format!(
        r#".{cls} {{
    color: red;
}}
.{cls} span, .{cls} div.selected {{
    background-color: blue;
}}
:not(.{cls}.highlighted) {{
    background-color: black;
}}
@media screen and (max-width: 500px) {{
    .{cls} {{
        display: flex;
    }}
}}
"#,
        cls = "stylist-testtest"
    );
    assert_eq!(sheet.to_style_str(Some("stylist-testtest")), Ok(expected));
}
