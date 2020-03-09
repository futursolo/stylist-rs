// Copyright © 2020 Lukas Wagner

/// A scope is pretty much a media query or everything not in a media query.
#[derive(Debug, Clone)]
pub(crate) struct Scope {
    pub(crate) condition: Option<String>,
    pub(crate) stylesets: Vec<ScopeContent>,
}

impl Scope {
    pub(crate) fn to_css(&self, class_name: String) -> String {
        let stylesets = self.stylesets.clone();
        let stylesets_css = stylesets
            .into_iter()
            .map(|styleset| match styleset {
                ScopeContent::Block(block) => block.to_css(class_name.clone()),
                ScopeContent::Rule(rule) => rule.to_css(),
            })
            .fold(String::new(), |acc, css_part| {
                format!("{}\n{}", acc, css_part)
            });
        match &self.condition {
            Some(condition) => {
                println!("scope some");
                format!("{} {{\n{}\n}}", condition, stylesets_css)
            }
            None => {
                println!("scope none");
                stylesets_css
            }
        }
    }
}

/// Everything that can be inside a scope.
#[derive(Debug, Clone)]
pub(crate) enum ScopeContent {
    Block(Block),
    Rule(Rule),
}

/// A block is a set of css properties that apply to elements that
/// match the condition.
#[derive(Debug, Clone)]
pub(crate) struct Block {
    pub(crate) condition: Option<String>,
    pub(crate) style_attributes: Vec<StyleAttribute>,
}

impl Block {
    fn to_css(&self, class_name: String) -> String {
        let condition = match &self.condition {
            Some(condition) => format!(" {}", condition),
            None => String::new(),
        };
        let style_property_css = self
            .style_attributes
            .clone()
            .into_iter()
            .map(|style_property| style_property.to_css())
            .fold(String::new(), |acc, css_part| {
                format!("{}\n{}", acc, css_part)
            });
        format!(".{}{}{{{}\n}}", class_name, condition, style_property_css)
    }
}

/// A simple CSS proprerty in the form of a key value pair.
/// E.g.: `color: red`
#[derive(Debug, Clone)]
pub(crate) struct StyleAttribute {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl StyleAttribute {
    fn to_css(&self) -> String {
        format!("{}:{};", self.key, self.value)
    }
}

/// A rule is everything that does not contain any properties.
/// An example would be `@keyframes`
#[derive(Debug, Clone)]
pub(crate) struct Rule {
    condition: String,
    content: String,
}

impl Rule {
    fn to_css(&self) -> String {
        format!("{} {{\n{}\n}}", self.condition, self.content)
    }
}
