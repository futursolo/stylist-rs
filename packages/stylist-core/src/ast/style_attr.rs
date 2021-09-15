use std::borrow::Cow;

use super::{StringFragment, StyleContext, ToStyleStr};

/// A simple CSS property in the form of a key value pair. Mirrors what would
/// be called a "Declaration" in the CSS standard.
///
/// E.g.: `color: red`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleAttribute {
    pub key: Cow<'static, str>,
    pub value: Cow<'static, [StringFragment]>,
}

impl ToStyleStr for StyleAttribute {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        // Always try to print block
        ctx.start(w);
        ctx.write_padding(w);

        w.push_str(&self.key);
        w.push_str(": ");

        for i in self.value.iter() {
            i.write_style(w, ctx);
        }

        w.push_str(";\n");
    }
}
