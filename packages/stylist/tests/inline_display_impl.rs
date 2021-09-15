use std::fmt::{Display, Formatter, Result};
enum Foo {
    Bar,
}
impl Display for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("none")
    }
}
impl Foo {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        "confused user impl".into()
    }
}
fn main() {
    let style = stylist::style! {
        display: ${Foo::Bar};
    }
    .unwrap();
    let expected_result = format!(
        r#".{cls} {{
    display: none;
}}
"#,
        cls = style.get_class_name()
    );
    assert_eq!(expected_result, style.get_style_str());
}
