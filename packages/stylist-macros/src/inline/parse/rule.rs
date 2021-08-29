use super::{
    super::{
        component_value::{ComponentValue, ComponentValueStream},
        css_ident::CssIdent,
        output::{OutputAtRule, OutputRuleContent},
    },
    fold_tokens_impl, reify_scope_contents, CssBlockQualifier, CssScope, OutputSheetContent,
};
use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

#[derive(Debug)]
pub enum CssAtRuleContent {
    Scope(CssScope),
    Empty(token::Semi),
}

#[derive(Debug)]
pub struct CssAtRule {
    at: token::At,
    name: CssIdent,
    prelude: Vec<ComponentValue>,
    contents: CssAtRuleContent,
    errors: Vec<ParseError>,
}

impl Parse for CssAtRule {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let at = input.parse()?;
        let name = input.parse::<CssIdent>()?;

        // Consume all tokens till the next ';' or the next block
        let mut component_iter = ComponentValueStream::from(input).peekable();
        let mut prelude = vec![];
        let mut errors = vec![];

        if !["media", "supports"].contains(&name.to_name_string().as_str()) {
            errors.push(ParseError::new_spanned(
                &name,
                format!("@-rule '{}' not supported", name),
            ));
        }

        let contents = loop {
            if input.peek(token::Semi) {
                let semi = input.parse()?;
                break CssAtRuleContent::Empty(semi);
            }
            if input.peek(token::Brace) {
                let scope = input.parse()?;
                break CssAtRuleContent::Scope(scope);
            }
            let next_token = component_iter
                .next()
                .ok_or_else(|| input.error("AtRule: unexpected end of input"))??;
            // unwrap okay, since we already peeked
            prelude.push(next_token);
        };

        Ok(Self {
            at,
            name,
            prelude,
            contents,
            errors,
        })
    }
}

impl CssAtRule {
    pub(super) fn fold_in_context(self, ctx: CssBlockQualifier) -> OutputSheetContent {
        let contents = match self.contents {
            CssAtRuleContent::Empty(_) => Vec::new(),
            CssAtRuleContent::Scope(scope) => {
                reify_scope_contents::<OutputRuleContent, _>(fold_tokens_impl(ctx, scope.contents))
            }
        };
        OutputSheetContent::AtRule(OutputAtRule {
            name: self.name.quote_at_rule().value(),
            prelude: self.prelude,
            contents,
            errors: self.errors,
        })
    }
}
