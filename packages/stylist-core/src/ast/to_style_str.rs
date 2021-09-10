use super::StyleContext;

/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
pub trait ToStyleStr {
    fn to_style_str(&self, class_name: Option<&str>) -> String {
        let mut s = String::new();

        let mut ctx = StyleContext::new(class_name);

        self.write_style(&mut s, &mut ctx);

        s
    }

    // If None is passed as class_name, it means to write a global style.
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>);
}
