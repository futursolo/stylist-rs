// this module is documented at stylist::ast

mod block;
mod context;
mod rule;
mod rule_block;
mod scope_content;
mod selector;
mod sheet;
mod str_frag;
mod style_attr;
mod to_style_str;

pub use context::StyleContext;

pub use block::{Block, BlockContent};
pub use rule::{Rule, RuleContent};
pub use rule_block::{RuleBlock, RuleBlockContent};
pub use scope_content::ScopeContent;
pub use selector::Selector;
pub use sheet::Sheet;
pub use style_attr::StyleAttribute;
pub use to_style_str::ToStyleStr;

pub use str_frag::StringFragment;

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
                    value: vec!["100vw".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![".inner".into()].into()].into(),
                style_attributes: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["red".into()].into(),
                }]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@keyframes move".into()].into(),
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
            condition: vec!["@media only screen and (min-width: 1000px)".into()].into(),
            content: vec![
                RuleContent::Block(Block {
                    condition: Cow::Borrowed(&[]),
                    style_attributes: vec![StyleAttribute {
                        key: "width".into(),
                        value: vec!["100vw".into()].into(),
                    }]
                    .into(),
                }),
                RuleContent::Block(Block {
                    condition: vec![vec![".inner".into()].into()].into(),
                    style_attributes: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["red".into()].into(),
                    }]
                    .into(),
                }),
                RuleContent::Rule(
                    Rule {
                        condition: vec!["@keyframes move".into()].into(),
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
