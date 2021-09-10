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

        let push_attrs_into_contents =
            |attrs: &mut Vec<CssAttribute>, contents: &mut Vec<OutputScopeContent>| {
                if attrs.is_empty() {
                    return;
                }

                contents.push(OutputScopeContent::Block(
                    CssQualifiedRule::into_dangling_output(attrs, ctx),
                ));
            };

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => {
                    attrs.push(m);
                }
                CssScopeContent::AtRule(m) => {
                    push_attrs_into_contents(&mut attrs, &mut contents);

                    contents.push(OutputScopeContent::Rule(m.into_rule_output(ctx)));
                }

                CssScopeContent::Nested(m) => {
                    push_attrs_into_contents(&mut attrs, &mut contents);

                    contents.push(OutputScopeContent::Block(m.into_output(ctx)));
                }
            }
        }

        push_attrs_into_contents(&mut attrs, &mut contents);

        OutputSheet { contents }
    }
}
