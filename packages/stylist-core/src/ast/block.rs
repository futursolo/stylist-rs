use std::borrow::Cow;
use std::fmt;

use super::{RuleBlock, Selector, StyleAttribute, ToStyleStr};
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockContent {
    StyleAttr(StyleAttribute),
    RuleBlock(RuleBlock),
}

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
    /// If the value is set as [`&[]`], it signals to substitute with the classname generated for the
    /// [`Sheet`](super::Sheet) in which this is contained.
    pub condition: Cow<'static, [Selector]>,
    pub style_attributes: Cow<'static, [StyleAttribute]>,
}

impl ToStyleStr for Block {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> Result<()> {
        if !self.condition.is_empty() {
            for (index, sel) in self.condition.iter().enumerate() {
                sel.write_style(w, class_name)?;
                if index < self.condition.len() - 1 {
                    write!(w, ",")?;
                }
                write!(w, " ")?;
            }
        } else if let Some(m) = class_name {
            write!(w, ".{} ", m)?;
        } else {
            // Generates global style for dangling block.
            write!(w, "html ")?;
        }

        writeln!(w, "{{")?;

        for attr in self.style_attributes.iter() {
            attr.write_style(w, class_name)?;
            writeln!(w)?;
        }

        write!(w, "}}")?;

        Ok(())
    }
}
