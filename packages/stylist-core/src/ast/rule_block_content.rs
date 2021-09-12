use super::{Block, Rule, StyleAttribute, StyleContext, ToStyleStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleBlockContent {
    StyleAttr(StyleAttribute),
    Rule(Box<Rule>),
    Block(Box<Block>),
}

impl From<StyleAttribute> for RuleBlockContent {
    fn from(s: StyleAttribute) -> Self {
        Self::StyleAttr(s)
    }
}

impl ToStyleStr for RuleBlockContent {
    fn write_style(&self, w: &mut String, ctx: &mut StyleContext<'_>) {
        match self {
            Self::StyleAttr(ref m) => m.write_style(w, ctx),
            Self::Rule(ref m) => m.write_style(w, ctx),
            Self::Block(ref m) => m.write_style(w, ctx),
        }
    }
}
