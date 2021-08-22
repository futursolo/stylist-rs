//! This module contains the semantic representation of a CSS.
//!
//! ```text
//! struct Sheet
//! └── Vec<enum ScopeContent>
//!     ├── struct Block
//!     │   ├── selector: Vec<Selector>
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

mod block;
mod into_sheet;
mod rule;
mod rule_content;
mod scope_content;
mod selector;
mod sheet;
mod style_attr;
mod to_style_str;

mod str_kind;

pub use block::Block;
pub use into_sheet::IntoSheet;
pub use rule::Rule;
pub use rule_content::RuleContent;
pub use scope_content::ScopeContent;
pub use selector::Selector;
pub use sheet::Sheet;
pub use style_attr::StyleAttribute;
pub use to_style_str::ToStyleStr;

pub use str_kind::StringKind;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::borrow::Cow;

    #[test]
    fn test_scope_building_without_condition() -> Result<()> {
        let test_block = Sheet::from(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: "width".into(),
                    value: "100vw".into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![".inner".into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: "red".into(),
                }]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: "@keyframes move".into(),
                content: vec![String::from(
                    r#"from {
width: 100px;
}
to {
width: 200px;
}"#,
                )
                .into()]
                .into(),
            }),
        ]);
        assert_eq!(
            test_block.to_style_str(Some("test"))?,
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

        Ok(())
    }

    #[test]
    fn test_scope_building_with_condition() -> Result<()> {
        let test_block = Sheet::from(vec![ScopeContent::Rule(Rule {
            condition: "@media only screen and (min-width: 1000px)".into(),
            content: vec![
                RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "width".into(),
                        value: "100vw".into(),
                    }]
                    .into(),
                }),
                RuleContent::Block(Block {
                    condition: vec![".inner".into()].into(),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: "red".into(),
                    }]
                    .into(),
                }),
                RuleContent::Rule(
                    Rule {
                        condition: "@keyframes move".into(),
                        content: vec![r#"from {
width: 100px;
}
to {
width: 200px;
}"#
                        .into()]
                        .into(),
                    }
                    .into(),
                ),
            ]
            .into(),
        })]);
        assert_eq!(
            test_block.to_style_str(Some("test"))?,
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

        Ok(())
    }
}
