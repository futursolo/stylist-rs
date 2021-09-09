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

impl ToStyleStr for BlockContent {
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &mut StyleContext<'_>) -> Result<()> {
        match self {
            Self::StyleAttr(ref m) => m.write_style(w, ctx)?,
            Self::RuleBlock(ref m) => m.write_style(w, ctx)?,
        }

        Ok(())
    }
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
    fn cond_str(&self, ctx: &mut StyleContext<'_>) -> Result<Option<String>> {
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
}

impl ToStyleStr for Block {
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &mut StyleContext<'_>) -> Result<()> {
        // Close last clause.
        ctx.write_finishing_clause(w)?;

        // TODO: nested block, which is not supported at the moment.
        let cond_s = self.cond_str(ctx)?;

        let mut final_ctx = cond_s
            .as_ref()
            .map(|m| ctx.with_condition(m))
            .unwrap_or_else(|| ctx.to_block_context());

        for attr in self.style_attributes.iter() {
            attr.write_style(w, &mut final_ctx)?;
        }

        final_ctx.write_finishing_clause(w)?;

        Ok(())
    }
}
