use std::borrow::Cow;
use std::fmt;

use super::{RuleContent, ToStyleStr};
use crate::Result;

/// An At-Rule can contain both other blocks and in some cases more At-Rules.
///
/// E.g.:
/// ```css
///  @keyframes move {
///     from {
///         width: 100px;
///     }
///     to {
///         width: 200px;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub condition: Cow<'static, str>,
    /// Note that not all At-Rules allow arbitrary other At-Rules to appear
    /// inside them, or arbitrary blocks. No safeguards at this point!
    pub content: Cow<'static, [RuleContent]>,
}

impl ToStyleStr for Rule {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> Result<()> {
        writeln!(w, "{} {{", self.condition)?;

        for i in self.content.iter() {
            i.write_style(w, class_name)?;
            writeln!(w)?;
        }

        write!(w, "}}")?;

        Ok(())
    }
}
