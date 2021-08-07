/// A scope represents a media query or all content not in a media query.
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
/// Structs implementing this trait should be able to turn into
/// a part of a CSS style sheet.
use std::fmt;

pub(crate) trait ToCss {
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

/// Everything that can reside inside a scope.
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
/// match the condition.
///
/// E.g.:
/// ```css
/// .inner {
///     color: red;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
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

/// A simple CSS proprerty in the form of a key value pair.
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

/// A rule is everything that does not contain any properties.
///
/// An example would be `@keyframes`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub condition: String,
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
    // A block
    Block(Block),
    // A nested rule
    Rule(Rule),
    // A raw string literal, i.e. something that wasn't parsed
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
pub(crate) fn sample_scopes() -> Sheet {
    Sheet(vec![ScopeContent::Block(Block {
        condition: None,
        style_attributes: vec![StyleAttribute {
            key: "color".to_string(),
            value: "red".to_string(),
        }],
    })])
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
