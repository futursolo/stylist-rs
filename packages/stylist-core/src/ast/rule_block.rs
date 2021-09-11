use std::borrow::Cow;

use super::{StringFragment, StyleAttribute, StyleContext, ToStyleStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleBlockContent {
    StyleAttr(StyleAttribute),
    RuleBlock(Box<RuleBlock>),
}

impl ToStyleStr for RuleBlockContent {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        match self {
            Self::StyleAttr(ref b) => b.write_style(w, ctx),
            Self::RuleBlock(ref r) => r.write_style(w, ctx),
        }
    }
}

/// A declaration block for at-rules.
///
/// This is used to represent at-rules that contains declaration block (e.g.:`@font-face`) and
/// `frame`s inside of a `@keyframes` at rule
/// as well as `@media` and `@supports` inside of a [`Block`](super::Block) which is a non-standard CSS feature.
///
/// E.g.:
/// ```css
/// .inner {
///     @media screen and (max-width: 500px) {
///         display: flex;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleBlock {
    pub condition: Cow<'static, [StringFragment]>,
    pub content: Cow<'static, [RuleBlockContent]>,
}

impl ToStyleStr for RuleBlock {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        let mut cond = "".to_string();
        for frag in self.condition.iter() {
            frag.write_style(&mut cond, ctx);
        }

        let mut rule_ctx = ctx.with_rule_condition(cond);
        // rule_ctx.start(w);
        for i in self.content.iter() {
            i.write_style(w, &mut rule_ctx);
        }

        rule_ctx.finish(w);
    }
}
