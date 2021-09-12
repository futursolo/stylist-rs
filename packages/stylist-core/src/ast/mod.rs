// this module is documented at stylist::ast

mod block;
mod context;
mod rule;
mod rule_block_content;
mod scope_content;
mod selector;
mod sheet;
mod str_frag;
mod style_attr;
mod to_style_str;

pub use context::StyleContext;

pub use block::Block;
pub use rule::Rule;
pub use rule_block_content::RuleBlockContent;
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
                content: vec![StyleAttribute {
                    key: "width".into(),
                    value: vec!["100vw".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Block(Block {
                condition: vec![vec![".inner".into()].into()].into(),
                content: vec![StyleAttribute {
                    key: "background-color".into(),
                    value: vec!["red".into()].into(),
                }
                .into()]
                .into(),
            }),
            ScopeContent::Rule(Rule {
                condition: vec!["@keyframes move".into()].into(),
                content: vec![
                    RuleBlockContent::Rule(Box::new(Rule {
                        condition: vec!["from".into()].into(),
                        content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                            key: "width".into(),
                            value: vec!["100px".into()].into(),
                        })]
                        .into(),
                    })),
                    RuleBlockContent::Rule(Box::new(Rule {
                        condition: vec!["to".into()].into(),
                        content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                            key: "width".into(),
                            value: vec!["200px".into()].into(),
                        })]
                        .into(),
                    })),
                ]
                .into(),
            }),
        ]);
        assert_eq!(
            test_block.to_style_str(Some("test")),
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
                RuleBlockContent::Block(Box::new(Block {
                    condition: Cow::Borrowed(&[]),
                    content: vec![StyleAttribute {
                        key: "width".into(),
                        value: vec!["100vw".into()].into(),
                    }
                    .into()]
                    .into(),
                })),
                RuleBlockContent::Block(Box::new(Block {
                    condition: vec![vec![".inner".into()].into()].into(),
                    content: vec![StyleAttribute {
                        key: "background-color".into(),
                        value: vec!["red".into()].into(),
                    }
                    .into()]
                    .into(),
                })),
                RuleBlockContent::Rule(
                    Rule {
                        condition: vec!["@keyframes move".into()].into(),
                        content: vec![
                            RuleBlockContent::Rule(Box::new(Rule {
                                condition: vec!["from".into()].into(),
                                content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                                    key: "width".into(),
                                    value: vec!["100px".into()].into(),
                                })]
                                .into(),
                            })),
                            RuleBlockContent::Rule(Box::new(Rule {
                                condition: vec!["to".into()].into(),
                                content: vec![RuleBlockContent::StyleAttr(StyleAttribute {
                                    key: "width".into(),
                                    value: vec!["200px".into()].into(),
                                })]
                                .into(),
                            })),
                        ]
                        .into(),
                    }
                    .into(),
                ),
            ]
            .into(),
        })]);
        assert_eq!(
            test_block.to_style_str(Some("test")),
            r#"@media only screen and (min-width: 1000px) {
    .test {
        width: 100vw;
    }
}
@media only screen and (min-width: 1000px) {
    .test .inner {
        background-color: red;
    }
}
@media only screen and (min-width: 1000px) {
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
