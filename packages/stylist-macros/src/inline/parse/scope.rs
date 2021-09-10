use syn::{
    braced,
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

use super::{CssScopeContent, IntoOutputContext};
use crate::output::{
    OutputAttribute, OutputBlock, OutputBlockContent, OutputRuleBlockContent, OutputRuleContent,
};

#[derive(Debug)]
pub struct CssScope {
    brace: token::Brace,
    pub contents: Vec<CssScopeContent>,
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let brace = braced!(inner in input);
        let contents = CssScopeContent::consume_list_of_rules(&inner)?;
        Ok(Self { brace, contents })
    }
}

impl CssScope {
    pub fn into_rule_output(self, ctx: &mut IntoOutputContext) -> Vec<OutputRuleContent> {
        let mut attrs = Vec::new();
        let mut contents = Vec::new();

        let collect_attrs = |attrs: &mut Vec<OutputAttribute>,
                             contents: &mut Vec<OutputRuleContent>| {
            if !attrs.is_empty() {
                contents.push(OutputRuleContent::Block(OutputBlock {
                    condition: Vec::new(),
                    content: attrs
                        .drain(0..)
                        .map(OutputBlockContent::StyleAttr)
                        .collect(),
                }));
            }
        };

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => attrs.push(m.into_output(ctx)),
                CssScopeContent::AtRule(m) => {
                    collect_attrs(&mut attrs, &mut contents);
                    contents.push(OutputRuleContent::Rule(m.into_rule_output(ctx)));
                }
                CssScopeContent::Nested(m) => {
                    collect_attrs(&mut attrs, &mut contents);
                    contents.push(OutputRuleContent::Block(m.into_output(ctx)));
                }
            }
        }

        collect_attrs(&mut attrs, &mut contents);

        contents
    }

    pub fn into_rule_block_output(
        self,
        ctx: &mut IntoOutputContext,
    ) -> Vec<OutputRuleBlockContent> {
        let mut contents = Vec::new();

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => {
                    contents.push(OutputRuleBlockContent::StyleAttr(m.into_output(ctx)))
                }

                CssScopeContent::AtRule(m) => {
                    contents.push(OutputRuleBlockContent::RuleBlock(Box::new(
                        m.into_rule_block_output(ctx),
                    )));
                }

                CssScopeContent::Nested(m) => {
                    ctx.push_error(ParseError::new_spanned(
                        m.qualifier,
                        "Can not nest qualified blocks (yet)",
                    ));
                }
            }
        }

        contents
    }

    pub fn into_block_output(self, ctx: &mut IntoOutputContext) -> Vec<OutputBlockContent> {
        let mut contents = Vec::new();

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => {
                    contents.push(OutputBlockContent::StyleAttr(m.into_output(ctx)))
                }
                CssScopeContent::AtRule(m) => {
                    contents.push(OutputBlockContent::RuleBlock(m.into_rule_block_output(ctx)))
                }
                CssScopeContent::Nested(m) => {
                    ctx.push_error(ParseError::new_spanned(
                        m.qualifier,
                        "Can not nest qualified blocks (yet)",
                    ));
                }
            }
        }

        contents
    }
}
