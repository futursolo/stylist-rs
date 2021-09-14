use syn::{
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

use super::{
    super::{
        component_value::{ComponentValue, ComponentValueStream},
        css_ident::CssIdent,
    },
    fragment_spacing, CssScope, IntoOutputContext,
};
use crate::{
    output::{OutputFragment, OutputRule},
    spacing_iterator::SpacedIterator,
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
        let mut component_iter = ComponentValueStream::from(input);
        let mut prelude = vec![];
        let mut errors = vec![];

        // Recognize the type of @-rule
        // TODO: be sensitive to this detected type when validating the prelude and contained attributes
        if !["media", "supports"].contains(&name.to_output_string().as_str()) {
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
    pub fn condition_output(&self) -> Vec<OutputFragment> {
        let mut prelude = vec![OutputFragment::Str(format!(
            "@{} ",
            self.name.to_output_string()
        ))];
        prelude.extend(
            self.prelude
                .clone()
                .into_iter()
                .flat_map(|p| p.to_output_fragments())
                .spaced_with(fragment_spacing),
        );

        prelude
    }

    pub fn into_rule_output(self, ctx: &mut IntoOutputContext) -> OutputRule {
        let condition = self.condition_output();
        ctx.extend_errors(self.errors);

        OutputRule {
            condition,
            content: match self.contents {
                CssAtRuleContent::Scope(m) => m.into_rule_output(ctx),
                CssAtRuleContent::Empty(_) => Vec::new(),
            },
        }
    }

    pub fn into_rule_block_output(self, ctx: &mut IntoOutputContext) -> OutputRule {
        let condition = self.condition_output();
        ctx.extend_errors(self.errors);

        OutputRule {
            condition,
            content: match self.contents {
                CssAtRuleContent::Scope(m) => m.into_rule_block_output(ctx),
                CssAtRuleContent::Empty(_) => Vec::new(),
            },
        }
    }
}
