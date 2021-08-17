use std::borrow::Cow;
use std::fmt;

use super::{Selector, StyleAttribute, ToStyleStr};

/// A block is a set of css properties that apply to elements that
/// match the condition. The CSS standard calls these "Qualified rules".
///
/// E.g.:
/// ```css
/// .inner {
///     color: red;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    /// Selector(s) for Current Block
    ///
    /// If value is set to [`&[]`], signals to substitute with the classname generated for the
    /// [`Sheet`](super::Sheet) in which this is conatined.
    pub condition: Cow<'static, [Selector]>,
    pub style_attributes: Cow<'static, [StyleAttribute]>,
}

impl ToStyleStr for Block {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        if !self.condition.is_empty() {
            for (index, sel) in self.condition.iter().enumerate() {
                sel.write_style(w, class_name)?;
                if index < self.condition.len() - 1 {
                    write!(w, ",")?;
                }
                write!(w, " ")?;
            }
        } else {
            write!(w, ".{} ", class_name)?;
        }

        writeln!(w, "{{")?;

        for attr in self.style_attributes.iter() {
            attr.write_style(w, class_name)?;
            writeln!(w)?;
        }

        write!(w, "}}")
    }
}
