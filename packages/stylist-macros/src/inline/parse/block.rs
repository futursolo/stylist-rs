use super::{CssAttribute, CssBlockQualifier, CssScope};
use crate::output::{OutputBlock, OutputBlockContent};
use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};

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
    pub fn into_output(self) -> Result<OutputBlock, Vec<ParseError>> {
        let qualifier_result = self.qualifier.into_output();
        let scope_result = self.scope.into_block_output();

        let (condition, content) = match (qualifier_result, scope_result) {
            (Ok(m), Ok(n)) => (m, n),
            (Err(mut e1), Err(e2)) => {
                e1.extend(e2);
                return Err(e1);
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        };

        Ok(OutputBlock { condition, content })
    }

    // Into Output for a dangling block
    pub fn into_dangling_output(
        attrs: &mut Vec<CssAttribute>,
    ) -> Result<OutputBlock, Vec<ParseError>> {
        let mut errors = Vec::new();
        let mut output_attrs = Vec::new();

        for attr in attrs.drain(0..) {
            match attr.into_output() {
                Ok(m) => output_attrs.push(OutputBlockContent::StyleAttr(m)),
                Err(e) => errors.extend(e),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(OutputBlock {
                condition: Vec::new(),
                content: output_attrs,
            })
        }
    }
}
