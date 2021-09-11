use std::borrow::Cow;

use super::{RuleBlock, Selector, StyleAttribute, StyleContext, ToStyleStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockContent {
    StyleAttr(StyleAttribute),
    RuleBlock(RuleBlock),
}

impl ToStyleStr for BlockContent {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        match self {
            Self::StyleAttr(ref m) => m.write_style(w, ctx),
            Self::RuleBlock(ref m) => m.write_style(w, ctx),
        }
    }
}

impl From<StyleAttribute> for BlockContent {
    fn from(s: StyleAttribute) -> Self {
        BlockContent::StyleAttr(s)
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
    pub content: Cow<'static, [BlockContent]>,
}

impl Block {
    fn cond_str(&self, ctx: &mut StyleContext<'_>) -> Option<String> {
        if self.condition.is_empty() {
            return None;
        }

        let mut cond = "".to_string();

        for (index, sel) in self.condition.iter().enumerate() {
            sel.write_style(&mut cond, ctx);
            if index < self.condition.len() - 1 {
                cond.push_str(", ");
            }
        }

        Some(cond)
    }
}

impl ToStyleStr for Block {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        // TODO: nested block, which is not supported at the moment.
        let cond_s = self.cond_str(ctx);

        let mut block_ctx = ctx.with_block_condition(cond_s);

        for attr in self.content.iter() {
            attr.write_style(w, &mut block_ctx);
        }

        block_ctx.finish(w);
    }
}
