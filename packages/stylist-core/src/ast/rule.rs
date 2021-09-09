use std::borrow::Cow;
use std::fmt;

use super::{Block, ScopeContent, StringFragment, StyleContext, ToStyleStr};
use crate::Result;

/// Everything that can be inside a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleContent {
    /// A block
    Block(Block),
    /// A nested rule
    Rule(Box<Rule>),
    /// A raw string literal, i.e. something that wasn't parsed.
    /// This is an escape-hatch and may get removed in the future
    /// for a more meaningful alternative
    String(Cow<'static, str>),
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
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &StyleContext<'_>) -> Result<()> {
        match self {
            RuleContent::Block(ref b) => b.write_style(w, ctx)?,
            RuleContent::Rule(ref r) => r.write_style(w, ctx)?,
            RuleContent::String(ref s) => write!(w, "{}", s)?,
        }

        Ok(())
    }
}

impl From<String> for RuleContent {
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}

impl From<&'static str> for RuleContent {
    fn from(s: &'static str) -> Self {
        Self::String(s.into())
    }
}

impl From<Cow<'static, str>> for RuleContent {
    fn from(s: Cow<'static, str>) -> Self {
        Self::String(s)
    }
}

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
    fn write_style<W: fmt::Write>(&self, w: &mut W, ctx: &StyleContext<'_>) -> Result<()> {
        let mut cond = "".to_string();
        for frag in self.condition.iter() {
            frag.write_style(&mut cond, ctx)?;
        }

        let rule_ctx = ctx.clone().with_condition(&cond);

        writeln!(w, "{} {{", cond)?;

        for i in self.content.iter() {
            i.write_style(w, &rule_ctx)?;
            writeln!(w)?;
        }

        write!(w, "}}")?;

        Ok(())
    }
}
