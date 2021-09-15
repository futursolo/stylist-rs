use std::borrow::Cow;

use super::{RuleBlockContent, StringFragment, StyleContext, ToStyleStr};

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
    pub condition: Cow<'static, [StringFragment]>,
    /// Note that not all At-Rules allow arbitrary other At-Rules to appear
    /// inside them, or arbitrary blocks. No safeguards at this point!
    pub content: Cow<'static, [RuleBlockContent]>,
}

impl ToStyleStr for Rule {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        let mut cond = "".to_string();
        for frag in self.condition.iter() {
            frag.write_style(&mut cond, ctx);
        }

        let mut rule_ctx = ctx.with_rule_condition(&cond);

        // keyframes should always be printed as they contain a global name.
        let always_print = cond.starts_with("@keyframes");
        if always_print {
            rule_ctx.start(w);
        }

        for i in self.content.iter() {
            i.write_style(w, &mut rule_ctx);
        }

        rule_ctx.finish(w);
    }
}
