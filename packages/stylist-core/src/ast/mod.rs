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

mod block;
mod into_sheet;
mod rule;
mod rule_content;
mod scope_content;
mod selector;
mod sheet;
mod style_attr;
mod to_style_str;

pub use block::Block;
pub use into_sheet::IntoSheet;
pub use rule::Rule;
pub use rule_content::RuleContent;
pub use scope_content::ScopeContent;
pub use selector::Selector;
pub use sheet::Sheet;
pub use style_attr::StyleAttribute;
pub use to_style_str::ToStyleStr;

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_scope_building_without_condition() {
        let test_block = Sheet(vec![
            ScopeContent::Block(Block {
                condition: Cow::Borrowed(&[]),
                style_attributes: vec![StyleAttribute {
                    key: String::from("width"),
                    value: String::from("100vw"),
                }],
            }),
            ScopeContent::Block(Block {
                condition: vec![".inner".into()].into(),
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
            test_block.to_style_str("test"),
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
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: String::from("width"),
                        value: String::from("100vw"),
                    }],
                }),
                RuleContent::Block(Block {
                    condition: vec![".inner".into()].into(),
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
            test_block.to_style_str("test"),
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
