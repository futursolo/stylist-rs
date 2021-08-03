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
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Scopes(Vec<Scope>);

impl ToCss for Scopes {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        for scope in self.0.iter() {
            scope.write_css(w, class_name)?;
            writeln!(w)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Scope {
    pub(crate) condition: Option<String>,
    pub(crate) stylesets: Vec<ScopeContent>,
}

impl ToCss for Scope {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        if let Some(ref m) = self.condition {
            writeln!(w, "{} {{", m)?;
        }

        for set in self.stylesets.iter() {
            match set {
                ScopeContent::Block(ref block) => block.write_css(w, class_name)?,
                ScopeContent::Rule(ref rule) => rule.write_css(w, class_name)?,
                // ScopeContent::Scope(scope) => scope.write_css(w, class_name)?,
            }

            writeln!(w)?;
        }

        if self.condition.is_some() {
            write!(w, "}}")?;
        }

        Ok(())
    }
}

/// Everything that can reside inside a scope.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ScopeContent {
    Block(Block),
    Rule(Rule),
    // e.g. media rules nested in support rules and vice versa
    // Scope(Scope),
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
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block {
    pub(crate) condition: Option<String>,
    pub(crate) style_attributes: Vec<StyleAttribute>,
}

impl ToCss for Block {
    fn to_css(&self, class_name: &str) -> String {
        let condition = match &self.condition {
            Some(condition) => format!(" {}", condition),
            None => String::new(),
        };
        let style_property_css = self
            .style_attributes
            .clone()
            .into_iter()
            .map(|style_property| style_property.to_css(class_name))
            .fold(String::new(), |mut acc, css_part| {
                acc.push('\n');
                acc.push_str(&css_part);
                acc
            });
        if condition.contains('&') {
            format!(
                "{} {{{}\n}}",
                condition.replace("&", format!(".{}", class_name).as_str()),
                style_property_css
            )
        } else {
            format!(".{}{} {{{}\n}}", class_name, condition, style_property_css)
        }
    }

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
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StyleAttribute {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl ToCss for StyleAttribute {
    fn write_css<W: fmt::Write>(&self, w: &mut W, _class_name: &str) -> fmt::Result {
        write!(w, "{}: {};", self.key, self.value)
    }
}

/// A rule is everything that does not contain any properties.
///
/// An example would be `@keyframes`
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Rule {
    pub(crate) condition: String,
    pub(crate) content: Vec<RuleContent>,
}

impl ToCss for Rule {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        writeln!(w, "{} {{", self.condition)?;

        for i in self.content.iter() {
            i.write_css(w, class_name)?
        }

        write!(w, "}}")
    }
}

/// Everything that can be inside a rule.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RuleContent {
    String(String),
    CurlyBraces(Vec<RuleContent>),
}

impl ToCss for RuleContent {
    fn write_css<W: fmt::Write>(&self, w: &mut W, class_name: &str) -> fmt::Result {
        match self {
            RuleContent::String(ref s) => writeln!(w, "{}", s),
            RuleContent::CurlyBraces(ref content) => {
                writeln!(w, "{{")?;

                for i in content.iter() {
                    i.write_css(w, class_name)?;
                    writeln!(w)?;
                }

                write!(w, "}}")
            }
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
        let test_block = Scope {
            condition: None,
            stylesets: vec![
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
            ],
        };
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
        let test_block = Scope {
            condition: Some(String::from("@media only screen and (min-width: 1000px)")),
            stylesets: vec![
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
            ],
        };
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
}"#
        );
    }
}
