use super::{normalize_hierarchy_impl, CssBlockQualifier, CssScope, OutputSheetContent};
use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug)]
pub struct CssQualifiedRule {
    qualifier: CssBlockQualifier,
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
    pub(super) fn fold_in_context(
        self,
        ctx: CssBlockQualifier,
    ) -> Box<dyn Iterator<Item = OutputSheetContent>> {
        let own_ctx = self.qualifier;
        if !own_ctx.is_empty() && !ctx.is_empty() {
            // TODO: figure out how to combine contexts
            // !Warning!: simply duplicating the containing blocks will (if no special care is taken)
            // also duplicate injected expressions, which will evaluate them multiple times, which can be
            // unexpected and confusing to the user.
            // !Warning!: when the qualifiers contain several selectors each, this can lead to an explosion
            // of emitted blocks. Consider
            // .one, .two, .three { .inner-one, .inner-two, .inner-three { background: ${injected_expr} } }
            // Following emotion, this would expand to 9 blocks and evaluate `injected_expr` 9 times.
            // A possibility would be collecting appearing expressions once up front and putting replacements
            // into the blocks.
            return Box::new(std::iter::once(OutputSheetContent::Error(
                ParseError::new_spanned(own_ctx, "Can not nest qualified blocks (yet)"),
            )));
        }
        let relevant_ctx = if !own_ctx.is_empty() { own_ctx } else { ctx };
        Box::new(normalize_hierarchy_impl(relevant_ctx, self.scope.contents))
    }
}
