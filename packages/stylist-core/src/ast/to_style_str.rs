use super::StyleContext;
use crate::Result;
use std::fmt;

/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
pub trait ToStyleStr {
    fn to_style_str(&self, class_name: Option<&str>) -> Result<String> {
        let mut s = String::new();

        let ctx = StyleContext {
            parent_conditions: class_name.map(|m| vec![m]).unwrap_or_else(Vec::new),
        };

        self.write_style(&mut s, &ctx)?;

        Ok(s)
    }

    // If None is passed as class_name, it means to write a global style.
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &StyleContext<'_>) -> Result<()>;
}
