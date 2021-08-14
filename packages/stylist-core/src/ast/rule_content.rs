use std::fmt;

use super::{Block, Rule, ScopeContent, ToStyleStr};

/// Everything that can be inside a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleContent {
    /// A block
    Block(Block),
    /// A nested rule
    Rule(Rule),
    /// A raw string literal, i.e. something that wasn't parsed.
    /// This is an escape-hatch and may get removed in the future
    /// for a more meaningful alternative
    String(String),
}

impl From<ScopeContent> for RuleContent {
    fn from(scope: ScopeContent) -> Self {
        match scope {
            ScopeContent::Block(b) => RuleContent::Block(b),
            ScopeContent::Rule(r) => RuleContent::Rule(r),
        }
    }
}

impl ToStyleStr for RuleContent {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        match self {
            RuleContent::Block(ref b) => b.write_style(w, class_name),
            RuleContent::Rule(ref r) => r.write_style(w, class_name),
            RuleContent::String(ref s) => write!(w, "{}", s),
        }
    }
}

impl From<String> for RuleContent {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}
