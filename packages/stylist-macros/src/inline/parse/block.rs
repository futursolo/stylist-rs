use crate::output::{OutputBlock, OutputRuleBlockContent};
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

use super::{CssAttribute, CssBlockQualifier, CssScope, IntoOutputContext};

#[derive(Debug)]
pub struct CssQualifiedRule {
    pub qualifier: CssBlockQualifier,
    scope: CssScope,
}

impl Parse for CssQualifiedRule {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let qualifier = input.parse()?;
        let scope = input.parse()?;
        Ok(Self { qualifier, scope })
    }
}

impl CssQualifiedRule {
    pub fn into_output(self, ctx: &mut IntoOutputContext) -> OutputBlock {
        let condition = self.qualifier.into_output(ctx);
        let content = self.scope.into_rule_block_output(ctx);

        OutputBlock { condition, content }
    }

    // Into Output for a dangling block
    pub fn into_dangling_output(
        attrs: Vec<CssAttribute>,
        ctx: &mut IntoOutputContext,
    ) -> OutputBlock {
        let mut output_attrs = Vec::new();

        for attr in attrs {
            output_attrs.push(OutputRuleBlockContent::StyleAttr(attr.into_output(ctx)))
        }

        OutputBlock {
            condition: Vec::new(),
            content: output_attrs,
        }
    }
}
