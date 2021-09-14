use std::borrow::Cow;
use std::fmt;

use super::{Block, Rule, ScopeContent, ToStyleStr};
use crate::bow::Bow;
use crate::Result;

/// Everything that can be inside a rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleContent {
    /// A block
    Block(Block),
    /// A nested rule
    Rule(Bow<'static, Rule>),
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
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> Result<()> {
        match self {
            RuleContent::Block(ref b) => b.write_style(w, class_name)?,
            RuleContent::Rule(ref r) => r.write_style(w, class_name)?,
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
