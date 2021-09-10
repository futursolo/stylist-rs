use crate::output::{OutputBlock, OutputBlockContent};
use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};

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
        let condition = self.qualifier.into_output(&mut ctx);
        let content = self.scope.into_block_output(&mut ctx);

        Ok(OutputBlock { condition, content })
    }

    // Into Output for a dangling block
    pub fn into_dangling_output(
        attrs: &mut Vec<CssAttribute>,
        ctx: &mut IntoOutputContext,
    ) -> OutputBlock {
        let mut output_attrs = Vec::new();

        for attr in attrs.drain(0..) {
            output_attrs.push(attr.into_output(&mut ctx))
        }

        OutputBlock {
            condition: Vec::new(),
            content: output_attrs,
        }
    }
}
