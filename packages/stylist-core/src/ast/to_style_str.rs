use std::fmt;

/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
pub trait ToStyleStr {
    fn to_style_str(&self, class_name: &str) -> String {
        let mut s = String::new();

        self.write_style(&mut s, class_name).unwrap();

        s
    }
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result;
}
