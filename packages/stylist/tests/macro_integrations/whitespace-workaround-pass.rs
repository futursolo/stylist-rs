fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let style = stylist::style! {
        &.class-a.class-b {
            color: red;
        }
        // FIXME: this test case is currently documenting a quirk with the inline style.
        // Once it can be fixed (proc_macro_span, https://github.com/rust-lang/rust/issues/54725)
        // update this test case output and the documentation
        //       v-- whitespace not detected
        &.class-a .class-b {
            color: black;
        }
        &.class-a *.class-b {
            color: white;
        }
        &.class-a #content {
            color: white;
        }
    }
    .unwrap();
    let expected_result = format!(
        r#".{cls}.class-a.class-b {{
color: red;
}}
.{cls}.class-a.class-b {{
color: black;
}}
.{cls}.class-a *.class-b {{
color: white;
}}
.{cls}.class-a #content {{
color: white;
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(expected_result, style.get_style_str());
}
