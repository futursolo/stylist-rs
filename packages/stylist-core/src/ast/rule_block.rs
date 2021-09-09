use std::borrow::Cow;

use super::{StringFragment, StyleAttribute};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleBlockContent {
    StyleAttr(StyleAttribute),
    RuleBlock(Box<RuleBlock>),
}

/// A declaration block for at-rules.
///
/// This is used to represent at-rules that contains declaration block (e.g.:`@font-face`) and
/// `@media` and `@supports` inside of a [`Block`](super::Block) which is a non-standard CSS feature.
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
    pub style_attributes: Cow<'static, [RuleBlockContent]>,
}
