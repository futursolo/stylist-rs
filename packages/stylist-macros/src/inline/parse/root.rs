use super::{
    super::output::{OutputScopeContent, OutputSheet},
    normalize_scope_hierarchy, CssScopeContent, OutputSheetContent,
};
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug)]
pub struct CssRootNode {
    root_contents: Vec<CssScopeContent>,
}

impl Parse for CssRootNode {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let root_contents = CssScopeContent::consume_list_of_rules(input)?;
        Ok(Self { root_contents })
    }
}

impl CssRootNode {
    pub fn into_output(self) -> OutputSheet {
        let contents = normalize_scope_hierarchy(self.root_contents)
            .map(|c| match c {
                OutputSheetContent::QualifiedRule(block) => OutputScopeContent::Block(block),
                OutputSheetContent::AtRule(rule) => OutputScopeContent::AtRule(rule),
                OutputSheetContent::Error(err) => OutputScopeContent::Err(err),
            })
            .collect();
        OutputSheet { contents }
    }
}
