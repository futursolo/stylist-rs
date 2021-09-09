use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

use super::{RuleBlock, Selector, StyleAttribute, StyleContext, ToStyleStr};
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

impl Block {
    fn cond_str(&self, ctx: &StyleContext<'_>) -> Result<Option<String>> {
        if self.condition.is_empty() {
            return Ok(None);
        }

        let mut cond = "".to_string();

        for (index, sel) in self.condition.iter().enumerate() {
            sel.write_style(&mut cond, ctx)?;
            if index < self.condition.len() - 1 {
                write!(&mut cond, ", ")?;
            }
        }

        Ok(Some(cond))
    }

    fn write_content<W: fmt::Write>(&self, w: &mut W, ctx: &StyleContext<'_>) -> Result<()> {
        writeln!(w, "{{")?;

        for attr in self.style_attributes.iter() {
            attr.write_style(w, ctx)?;
            writeln!(w)?;
        }

        write!(w, "}}")?;

        Ok(())
    }
}

impl ToStyleStr for Block {
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &StyleContext<'_>) -> Result<()> {
        if let Some(m) = self.cond_str(ctx)? {
            write!(w, "{} ", m)?;

            let block_ctx = ctx.clone().with_condition(&m);

            return self.write_content(w, &block_ctx);
            // TODO: nested block.
        }

        // Dangling Block.
        if let Some(m) = ctx.root_class_name() {
            write!(w, ".{} ", m)?;
        } else {
            // Generates global style for dangling block.
            write!(w, "html ")?;
        }

        self.write_content(w, ctx)?;

        Ok(())
    }
}
