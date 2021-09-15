use std::mem;

use syn::{
    braced,
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

use super::{CssAttribute, CssQualifiedRule, CssScopeContent, IntoOutputContext};
use crate::output::OutputRuleBlockContent;

#[derive(Debug)]
pub struct CssScope {
    _brace: token::Brace,
    pub contents: Vec<CssScopeContent>,
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let brace = braced!(inner in input);
        let contents = CssScopeContent::consume_list_of_rules(&inner)?;
        Ok(Self {
            _brace: brace,
            contents,
        })
    }
}

impl CssScope {
    pub fn into_rule_output(self, ctx: &mut IntoOutputContext) -> Vec<OutputRuleBlockContent> {
        let mut attrs = Vec::new();
        let mut contents = Vec::new();

        let flush_attrs = |attrs: &mut Vec<CssAttribute>,
                           contents: &mut Vec<OutputRuleBlockContent>,
                           ctx: &mut IntoOutputContext| {
            if !attrs.is_empty() {
                contents.push(OutputRuleBlockContent::Block(Box::new(
                    CssQualifiedRule::into_dangling_output(mem::take(attrs), ctx),
                )));
            }
        };

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => attrs.push(m),
                CssScopeContent::AtRule(m) => {
                    flush_attrs(&mut attrs, &mut contents, ctx);
                    contents.push(OutputRuleBlockContent::Rule(Box::new(
                        m.into_rule_output(ctx),
                    )));
                }
                CssScopeContent::Nested(m) => {
                    flush_attrs(&mut attrs, &mut contents, ctx);
                    contents.push(OutputRuleBlockContent::Block(Box::new(m.into_output(ctx))));
                }
            }
        }

        flush_attrs(&mut attrs, &mut contents, ctx);

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
                    contents.push(OutputRuleBlockContent::Rule(Box::new(
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
}
