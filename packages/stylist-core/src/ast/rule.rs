use std::borrow::Cow;

use super::{Block, RuleBlock, ScopeContent, StringFragment, StyleContext, ToStyleStr};

/// Everything that can be inside a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleContent {
    /// A block
    Block(Block),
    /// A nested rule
    Rule(Box<Rule>),
    // /// A raw string literal, i.e. something that wasn't parsed.
    // /// This is an escape-hatch and may get removed in the future
    // /// for a more meaningful alternative
    // String(Cow<'static, str>),
    /// A RuleBlock
    RuleBlock(RuleBlock),
}

impl From<ScopeContent> for RuleContent {
    fn from(scope: ScopeContent) -> Self {
        match scope {
            ScopeContent::Block(b) => RuleContent::Block(b),
            ScopeContent::Rule(r) => RuleContent::Rule(r.into()),
        }
    }
}

impl ToStyleStr for RuleContent {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_, '_>) {
        match self {
            RuleContent::Block(ref m) => m.write_style(w, ctx),
            RuleContent::Rule(ref m) => m.write_style(w, ctx),
            RuleContent::RuleBlock(ref m) => m.write_style(w, ctx),
            // RuleContent::String(ref s) => {
            //     ctx.write_starting_clause(w);
            //     w.push_str(s);
            //     w.push('\n');
            // }
        }
    }
}

// impl From<String> for RuleContent {
//     fn from(s: String) -> Self {
//         Self::String(s.into())
//     }
// }

// impl From<&'static str> for RuleContent {
//     fn from(s: &'static str) -> Self {
//         Self::String(s.into())
//     }
// }

// impl From<Cow<'static, str>> for RuleContent {
//     fn from(s: Cow<'static, str>) -> Self {
//         Self::String(s)
//     }
// }

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
    pub content: Cow<'static, [RuleContent]>,
}

impl ToStyleStr for Rule {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_, '_>) {
        let mut cond = "".to_string();
        for frag in self.condition.iter() {
            frag.write_style(&mut cond, ctx);
        }

        let mut rule_ctx = ctx.with_rule_condition(&cond);
        if cond.starts_with("@keyframes") {
            rule_ctx.start(w); // keyframes should always be printed.
        }

        for i in self.content.iter() {
            i.write_style(w, &mut rule_ctx);
        }

        rule_ctx.finish(w);
    }
}
