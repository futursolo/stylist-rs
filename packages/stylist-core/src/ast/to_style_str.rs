use super::StyleContext;
use crate::Result;
use std::fmt;

/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
pub trait ToStyleStr {
    fn to_style_str(&self, class_name: Option<&str>) -> Result<String> {
        let mut s = String::new();

        let mut ctx = StyleContext::new(class_name);

        self.write_style(&mut s, &mut ctx)?;

        Ok(s)
    }

    // If None is passed as class_name, it means to write a global style.
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &mut StyleContext<'_>) -> Result<()>;
}
