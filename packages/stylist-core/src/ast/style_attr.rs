use std::borrow::Cow;
use std::fmt;

use super::{StringFragment, StyleContext, ToStyleStr};
use crate::Result;

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
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &mut StyleContext<'_>) -> Result<()> {
        // Always write starting clause.
        ctx.write_starting_clause(w)?;
        ctx.write_padding(w)?;

        write!(w, "{}: ", self.key)?;

        for i in self.value.iter() {
            i.write_style(w, ctx)?;
        }

        writeln!(w, ";")?;

        Ok(())
    }
}
