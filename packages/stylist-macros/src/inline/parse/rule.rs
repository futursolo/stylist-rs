use super::{
    super::{
        component_value::{ComponentValue, ComponentValueStream},
        css_ident::CssIdent,
        output::{OutputAtRule, OutputRuleContent},
    },
    normalize_hierarchy_impl, CssBlockQualifier, CssScope, MaybeStatic, OutputSheetContent, Reify,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
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
        if !ctx.is_empty() {
            return OutputSheetContent::Error(ParseError::new_spanned(
                self.prelude_span(),
                "Can not nest @-rules (yet)",
            ));
        }
        let contents = match self.contents {
            CssAtRuleContent::Empty(_) => MaybeStatic::statick(Vec::new()),
            CssAtRuleContent::Scope(scope) => normalize_hierarchy_impl(ctx, scope.contents)
                .map(|c| match c {
                    OutputSheetContent::AtRule(rule) => {
                        OutputRuleContent::AtRule(rule).into_token_stream()
                    }
                    OutputSheetContent::QualifiedRule(block) => {
                        OutputRuleContent::Block(block).into_token_stream()
                    }
                    OutputSheetContent::Error(err) => err.into_token_stream(),
                })
                .collect(),
        };
        OutputSheetContent::AtRule(OutputAtRule {
            name: self.name.quote_at_rule().value(),
            prelude: self.prelude,
            contents,
            errors: self.errors,
        })
    }
}

impl CssAtRule {
    fn prelude_span(&self) -> PreludeSpan {
        PreludeSpan { rule: self }
    }
}

struct PreludeSpan<'a> {
    rule: &'a CssAtRule,
}

impl<'a> ToTokens for PreludeSpan<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let rule = self.rule;
        rule.at.to_tokens(toks);
        rule.name.to_tokens(toks);
        for c in rule.prelude.iter() {
            c.to_tokens(toks);
        }
    }
}
