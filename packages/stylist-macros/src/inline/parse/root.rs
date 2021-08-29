use super::{
    super::output::{OutputScopeContent, OutputSheet},
    fold_normalized_scope_hierarchy, reify_scope_contents, CssScopeContent,
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
        let contents = reify_scope_contents::<OutputScopeContent, _>(
            fold_normalized_scope_hierarchy(self.root_contents),
        );
        OutputSheet { contents }
    }
}
