//! This module contains the semantic representation of a CSS.
//!
//! ```text
//! struct Sheet
//! └── Vec<enum ScopeContent>
//!     ├── struct Block
//!     │   ├── selector: String
//!     │   └── Vec<struct StyleAttribute>
//!     │       ├── key: String
//!     │       └── value: String
//!     └── struct Rule
//!         ├── condition: String
//!         └── Vec<enum RuleContent>
//!             ├── Block (*)
//!             └── Rule (*)
//! ```
//!
use std::fmt;
use std::str::FromStr;

use crate::parser::Parser;

/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
pub trait ToCss {
    fn to_css(&self, class_name: &str) -> String {
        let mut s = String::new();

        self.write_css(&mut s, class_name).unwrap();

        s
    }
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result;
}

/// The top node of a style string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sheet(pub Vec<ScopeContent>);

impl FromStr for Sheet {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        Parser::parse(s)
    }
}

impl Sheet {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

impl ToCss for Sheet {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        for scope in self.0.iter() {
            scope.write_css(w, class_name)?;
            writeln!(w)?;
        }

        Ok(())
    }
}

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

impl ToCss for ScopeContent {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        match self {
            ScopeContent::Block(ref b) => b.write_css(w, class_name),
            ScopeContent::Rule(ref r) => r.write_css(w, class_name),
        }
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
    /// If set to [`None`], signals to substitute with the classname generated for the
    /// [`Sheet`] in which this is conatined. Otherwise substitute the classname for
    /// each occuring '&', i.e. `None` is equivalent to `Some("&")`.
    pub condition: Option<String>,
    pub style_attributes: Vec<StyleAttribute>,
}

impl ToCss for Block {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        if let Some(ref condition) = self.condition {
            if condition.contains('&') {
                let scoped_class = format!(".{}", class_name);
                writeln!(w, "{} {{", condition.replace("&", scoped_class.as_str()))?;
            } else {
                writeln!(w, ".{} {} {{", class_name, condition)?;
            }
        } else {
            writeln!(w, ".{} {{", class_name)?;
        }

        for attr in self.style_attributes.iter() {
            attr.write_css(w, class_name)?;
            writeln!(w)?;
        }

        write!(w, "}}")
    }
}

/// A simple CSS property in the form of a key value pair. Mirrors what would
/// be called a "Declaration" in the CSS standard.
///
/// E.g.: `color: red`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleAttribute {
    pub key: String,
    pub value: String,
}

impl ToCss for StyleAttribute {
    fn write_css<W: fmt::Write>(&self, w: &mut W, _class_name: &str) -> fmt::Result {
        write!(w, "{}: {};", self.key, self.value)
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
    pub condition: String,
    /// Note that not all At-Rules allow arbitrary other At-Rules to appear
    /// inside them, or arbitrary blocks. No safeguards at this point!
    pub content: Vec<RuleContent>,
}

impl ToCss for Rule {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        writeln!(w, "{} {{", self.condition)?;

        for i in self.content.iter() {
            i.write_css(w, class_name)?;
            writeln!(w)?;
        }

        write!(w, "}}")
    }
}

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

impl ToCss for RuleContent {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        match self {
            RuleContent::Block(ref b) => b.write_css(w, class_name),
            RuleContent::Rule(ref r) => r.write_css(w, class_name),
            RuleContent::String(ref s) => write!(w, "{}", s),
        }
    }
}

impl From<String> for RuleContent {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_building_without_condition() {
        let test_block = Sheet(vec![
            ScopeContent::Block(Block {
                condition: None,
                style_attributes: vec![StyleAttribute {
                    key: String::from("width"),
                    value: String::from("100vw"),
                }],
            }),
            ScopeContent::Block(Block {
                condition: Some(String::from(".inner")),
                style_attributes: vec![StyleAttribute {
                    key: String::from("background-color"),
                    value: String::from("red"),
                }],
            }),
            ScopeContent::Rule(Rule {
                condition: String::from("@keyframes move"),
                content: vec![String::from(
                    r#"from {
width: 100px;
}
to {
width: 200px;
}"#,
                )
                .into()],
            }),
        ]);
        assert_eq!(
            test_block.to_css("test"),
            r#".test {
width: 100vw;
}
.test .inner {
background-color: red;
}
@keyframes move {
from {
width: 100px;
}
to {
width: 200px;
}
}
"#
        );
    }

    #[test]
    fn test_scope_building_with_condition() {
        let test_block = Sheet(vec![ScopeContent::Rule(Rule {
            condition: String::from("@media only screen and (min-width: 1000px)"),
            content: vec![
                RuleContent::Block(Block {
                    condition: None,
                    style_attributes: vec![StyleAttribute {
                        key: String::from("width"),
                        value: String::from("100vw"),
                    }],
                }),
                RuleContent::Block(Block {
                    condition: Some(String::from(".inner")),
                    style_attributes: vec![StyleAttribute {
                        key: String::from("background-color"),
                        value: String::from("red"),
                    }],
                }),
                RuleContent::Rule(Rule {
                    condition: String::from("@keyframes move"),
                    content: vec![String::from(
                        r#"from {
width: 100px;
}
to {
width: 200px;
}"#,
                    )
                    .into()],
                }),
            ],
        })]);
        assert_eq!(
            test_block.to_css("test"),
            r#"@media only screen and (min-width: 1000px) {
.test {
width: 100vw;
}
.test .inner {
background-color: red;
}
@keyframes move {
from {
width: 100px;
}
to {
width: 200px;
}
}
}
"#
        );
    }
}
