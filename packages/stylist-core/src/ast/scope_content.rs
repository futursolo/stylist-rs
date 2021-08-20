use std::fmt;

use super::{Block, Rule, ToStyleStr};

/// A scope represents a media query or all content not in a media query.
/// The CSS-Syntax-Level-3 standard calls all of these rules, which is used
/// here specifically for At-Rules. A Qualified rule is represented by a [`Block`],
/// an At-Rule is represented by a [`Rule`].
///
/// As an example:
/// ```css
/// /* BEGIN Scope */
/// .wrapper {
///     width: 100vw;
/// }
/// /* END Scope */
/// /* BEGIN Scope */
/// @media only screen and (min-width: 1000px) {
///     .wrapper {
///         width: 1000px;
///     }
/// }
/// /* END Scope */
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeContent {
    Block(Block),
    Rule(Rule),
    // e.g. media rules nested in support rules and vice versa
    // Scope(Scope),
}

impl ToStyleStr for ScopeContent {
    fn write_style<W: fmt::Write>(&self, w: &mut W, class_name: Option<&str>) -> fmt::Result {
        match self {
            ScopeContent::Block(ref b) => b.write_style(w, class_name),
            ScopeContent::Rule(ref r) => r.write_style(w, class_name),
        }
    }
}
