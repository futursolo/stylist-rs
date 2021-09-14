fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let sheet = stylist::sheet! {
        border: medium dashed green;
        // pseudo class, sibling
        &:checked + label {
            // color spec with #-bang spec
            color: #9799a7;
        }
        // nth child, general sibling
        &:nth-child(-n+4) ~ nav {
            // suffixed value
            max-height: 500px;
        }
        // pseudo-element selector
        ::first-letter {
            // attribute with different kinds of literals
            box-shadow: 3px 3px red, -1rem 0 0.4rem olive;
        }
        // descendent selector
        article span {
            box-shadow: inset 0 1px 2px rgba(0.32, 0, 0, 15%);
        }
        // contains selector, begins with, ends with, spaced hyphenated
        a[href*="login"],
        a[href^="https://"],
        // FIXME: should work, but incorrectly reparsed after emitting
        // parsing in macro works fine.
        //a[href$=".pdf" ],
        a[rel~="tag"],
        a[lang|="en"]
        {
            // string literals
            background-image: url("images/pdf.png");
        }
        // another pseudo selector
        #content::after {
            content: " (" attr(x) ")";
        }
    };
    log::debug!("{:?}", sheet);
    let style = stylist::Style::new(sheet).unwrap();
    let expected_result = format!(
        r#".{cls} {{
border: medium dashed green;
}}
.{cls}:checked+label {{
color: #9799a7;
}}
.{cls}:nth-child(-n+4)~nav {{
max-height: 500px;
}}
.{cls}::first-letter {{
box-shadow: 3px 3px red,-1rem 0 0.4rem olive;
}}
.{cls} article span {{
box-shadow: inset 0 1px 2px rgba(0.32,0,0,15%);
}}
.{cls} a[href*="login"], .{cls} a[href^="https://"], .{cls} a[rel~="tag"], .{cls} a[lang|="en"] {{
background-image: url("images/pdf.png");
}}
.{cls} #content::after {{
content: " (" attr(x)")";
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(expected_result, style.get_style_str());
}
