use std::mem;

use super::{CssAttribute, CssQualifiedRule, CssScopeContent, IntoOutputContext};
use crate::output::{OutputScopeContent, OutputSheet};
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug)]
pub struct CssRootNode {
    contents: Vec<CssScopeContent>,
}

impl Parse for CssRootNode {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let contents = CssScopeContent::consume_list_of_rules(input)?;
        Ok(Self { contents })
    }
}

impl CssRootNode {
    pub fn into_output(self, ctx: &mut IntoOutputContext) -> OutputSheet {
        let mut contents = Vec::new();

        let mut attrs: Vec<CssAttribute> = Vec::new();
        let flush_attrs = |attrs: &mut Vec<CssAttribute>,
                           contents: &mut Vec<OutputScopeContent>,
                           ctx: &mut IntoOutputContext| {
            if !attrs.is_empty() {
                contents.push(OutputScopeContent::Block(
                    CssQualifiedRule::into_dangling_output(mem::take(attrs), ctx),
                ));
            }
        };

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => attrs.push(m),
                CssScopeContent::AtRule(m) => {
                    flush_attrs(&mut attrs, &mut contents, ctx);
                    contents.push(OutputScopeContent::Rule(m.into_rule_output(ctx)));
                }

                CssScopeContent::Nested(m) => {
                    flush_attrs(&mut attrs, &mut contents, ctx);
                    contents.push(OutputScopeContent::Block(m.into_output(ctx)));
                }
            }
        }

        flush_attrs(&mut attrs, &mut contents, ctx);
        OutputSheet { contents }
    }
}
