use super::{
    component_value::{
        ComponentValue, ComponentValueStream, InjectedExpression, PreservedToken, SimpleBlock,
    },
    css_ident::CssIdent,
    output::{
        OutputAtRule, OutputAttribute, OutputQualifiedRule, OutputQualifier, OutputRuleContent,
        OutputScopeContent, OutputSheet, Reify,
    },
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::iter::Peekable;
use syn::{
    braced,
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    spanned::Spanned,
    token,
};

#[derive(Debug)]
pub struct CssRootNode {
    root_contents: Vec<CssScopeContent>,
}

#[derive(Debug, Clone)]
pub struct CssScopeQualifier {
    qualifiers: Vec<ComponentValue>,
    qualifier_errors: Vec<ParseError>,
}

#[derive(Debug)]
pub struct CssScope {
    brace: token::Brace,
    contents: Vec<CssScopeContent>,
}

#[derive(Debug)]
pub struct CssQualifiedRule {
    qualifier: CssScopeQualifier,
    scope: CssScope,
}

#[derive(Debug)]
pub enum CssScopeContent {
    Attribute(CssAttribute),
    AtRule(CssAtRule),
    Nested(CssQualifiedRule),
}

#[derive(Debug)]
pub enum CssAttributeName {
    Identifier(CssIdent),
    InjectedExpr(InjectedExpression),
}

#[derive(Debug)]
pub struct CssAttributeValue {
    values: Vec<ParseResult<ComponentValue>>,
}

#[derive(Debug)]
pub struct CssAttribute {
    name: CssAttributeName,
    colon: token::Colon,
    value: CssAttributeValue,
    terminator: token::Semi,
}

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
// =====================================================================
// =====================================================================
// Parsing implementation
// =====================================================================
// =====================================================================
impl Parse for CssRootNode {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let root_contents = CssScopeContent::consume_list_of_rules(input)?;
        Ok(Self { root_contents })
    }
}

impl CssScopeContent {
    // (5.4.1) Consume a list of rules
    fn consume_list_of_rules(input: &ParseBuffer) -> ParseResult<Vec<Self>> {
        let mut contents = Vec::new();
        while !input.is_empty() {
            // Not handled: <CDO-token> <CDC-token>
            contents.push(input.parse()?);
        }
        Ok(contents)
    }
}

impl Parse for CssScopeContent {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        // Fork the stream. Peeking a component value will still consume tokens from the stream!
        let forked_input = input.fork();
        let mut component_peek = ComponentValueStream::from(&forked_input).multipeek();
        let next_input = component_peek
            .peek()
            .cloned()
            .ok_or_else(|| forked_input.error("Scope: unexpected end of input"))??;
        // Steps from 5.4.4. Consume a list of declarations
        // At-rule first
        if let ComponentValue::Token(PreservedToken::Punct(ref p)) = next_input {
            if p.as_char() == '@' {
                let atrule = input.parse()?;
                return Ok(Self::AtRule(atrule));
            }
        }
        // If it starts with an <ident-token>, it might be an attribute.
        if next_input.maybe_to_attribute_name().is_some() {
            // peek another token to see if it's colon
            let maybe_colon = component_peek.peek();
            if let Some(Ok(ComponentValue::Token(PreservedToken::Punct(p)))) = maybe_colon {
                if p.as_char() == ':' {
                    let attr = input.parse()?;
                    return Ok(Self::Attribute(attr));
                }
            }
        }
        // It isn't. All that's left now is that it's a qualified rule.
        let rule = input.parse()?;
        Ok(Self::Nested(rule))
    }
}

impl Parse for CssAttribute {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let mut component_iter = ComponentValueStream::from(input);
        // Advance the real iterator
        let name = component_iter
            .next()
            .ok_or_else(|| input.error("Attribute: unexpected end of input"))??;
        let name_span = name.span();
        let name = name.maybe_to_attribute_name().ok_or_else(|| {
            ParseError::new(
                name_span,
                "expected an identifier or interpolated expression",
            )
        })?;

        let colon = input.parse()?;
        let value = input.parse()?;
        let terminator = input.parse()?;
        Ok(CssAttribute {
            name,
            colon,
            value,
            terminator,
        })
    }
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let brace = braced!(inner in input);
        let contents = CssScopeContent::consume_list_of_rules(&inner)?;
        Ok(Self { brace, contents })
    }
}

impl Parse for CssAttributeValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        // Consume all tokens till the next ';'
        let mut component_iter = ComponentValueStream::from(input).peekable();
        let mut values = vec![];
        loop {
            if input.peek(token::Semi) {
                break;
            }
            let next_token = component_iter
                .next()
                .ok_or_else(|| input.error("AttributeValue: unexpected end of input"))??;
            let parsed_token = if !next_token.is_attribute_token() {
                let error_message = if matches!(
                    next_token,
                    ComponentValue::Block(SimpleBlock::Braced { .. })
                ) {
                    "expected a valid part of an attribute, got a block. Did you mean to write `${..}` to inject an expression?"
                } else {
                    "expected a valid part of an attribute"
                };
                Err(ParseError::new_spanned(next_token, error_message))
            } else {
                Ok(next_token)
            };
            // unwrap okay, since we already peeked
            values.push(parsed_token);
        }
        Ok(Self { values })
    }
}

impl Parse for CssScopeQualifier {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        // Consume all tokens till the next '{'-block
        let mut component_iter = ComponentValueStream::from(input).peekable();
        let mut qualifiers = vec![];
        let mut qualifier_errors = vec![];
        loop {
            if input.peek(token::Brace) {
                break;
            }
            let next_token = component_iter
                .next()
                .ok_or_else(|| input.error("ScopeQualifier: unexpected end of input"))??;
            qualifier_errors.extend(next_token.validate_selector_token()?);
            qualifiers.push(next_token);
        }
        // FIXME: reparse scope qualifiers for more validation?
        Ok(Self {
            qualifiers,
            qualifier_errors,
        })
    }
}

impl Parse for CssQualifiedRule {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let qualifier = input.parse()?;
        let scope = input.parse()?;
        Ok(Self { qualifier, scope })
    }
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

impl ComponentValue {
    fn maybe_to_attribute_name(self) -> Option<CssAttributeName> {
        match self {
            ComponentValue::Token(PreservedToken::Ident(i)) => {
                Some(CssAttributeName::Identifier(i))
            }
            ComponentValue::Expr(expr) => Some(CssAttributeName::InjectedExpr(expr)),
            _ => None,
        }
    }
}

impl ToTokens for CssScopeQualifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for q in self.qualifiers.iter() {
            q.to_tokens(tokens);
        }
    }
}

impl Default for CssScopeQualifier {
    fn default() -> Self {
        Self {
            qualifiers: vec![],
            qualifier_errors: vec![],
        }
    }
}
// =====================================================================
// =====================================================================
// Convert into output
// =====================================================================
// =====================================================================

/// We want to normalize the input a bit. For that, we want to pretend that e.g.
/// the sample input
///
/// ```css
/// outer-attribute: some;
/// foo-bar: zet;
/// .nested {
///     @media print {
///         only-in-print: foo;
///     }
///     and-always: red;
/// }
/// ```
///
/// gets processed as if written in the (more verbose) shallowly nested style:
///
/// ```css
/// {
///     outer-attribute: some;
///     foo-bar: zet;
/// }
/// @media print {
///     .nested {
///         only-in-print: foo;
///     }
/// }
/// .nested {
///     and-always: red;
/// }
///
/// ```
///
/// Errors in nested items are reported as spanned TokenStreams.
///
fn fold_normalized_scope_hierarchy<'it>(
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
) -> impl 'it + Iterator<Item = ParseResult<OutputSheetContent>> {
    fold_tokens_impl(Default::default(), it)
}

enum OutputSheetContent {
    AtRule(OutputAtRule),
    QualifiedRule(OutputQualifiedRule),
}

fn fold_tokens_impl<'it>(
    context: CssScopeQualifier,
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
) -> impl 'it + Iterator<Item = ParseResult<OutputSheetContent>> {
    // Step one: collect attributes into blocks, flatten and lift nested blocks
    struct Wrapper<I: Iterator> {
        it: Peekable<I>,
        qualifier: OutputQualifier,
    }

    enum WrappedCase {
        AttributeCollection(OutputQualifiedRule),
        AtRule(CssAtRule),
        Qualified(CssQualifiedRule),
    }

    impl<I: Iterator<Item = CssScopeContent>> Iterator for Wrapper<I> {
        type Item = WrappedCase;
        fn next(&mut self) -> Option<Self::Item> {
            match self.it.peek() {
                None => return None,
                Some(CssScopeContent::Attribute(_)) => {
                    let mut attributes = Vec::new();
                    while let Some(CssScopeContent::Attribute(attr)) = self
                        .it
                        .next_if(|i| matches!(i, CssScopeContent::Attribute(_)))
                    {
                        attributes.push(attr.into_output().into_token_stream());
                    }
                    let replacement = OutputQualifiedRule {
                        qualifier: self.qualifier.clone().into_token_stream(),
                        attributes,
                    };
                    return Some(WrappedCase::AttributeCollection(replacement));
                }
                _ => {}
            }
            match self.it.next() {
                Some(CssScopeContent::AtRule(r)) => Some(WrappedCase::AtRule(r)),
                Some(CssScopeContent::Nested(b)) => Some(WrappedCase::Qualified(b)),
                _ => unreachable!("Should have returned after peeking"),
            }
        }
    }

    Wrapper {
        it: it.into_iter().peekable(),
        qualifier: context.clone().into_output(),
    }
    .flat_map(move |w| match w {
        WrappedCase::AttributeCollection(attrs) => {
            let result = Ok(OutputSheetContent::QualifiedRule(attrs));
            Box::new(std::iter::once(result))
        }
        WrappedCase::AtRule(r) => {
            let result = Ok(r.fold_in_context(context.clone()));
            Box::new(std::iter::once(result))
        }
        WrappedCase::Qualified(b) => b.fold_in_context(context.clone()),
    })
}

fn reify_scope_contents<
    O: From<OutputSheetContent> + Reify,
    It: Iterator<Item = ParseResult<OutputSheetContent>>,
>(
    scope: It,
) -> Vec<TokenStream> {
    scope.map(|i| i.map(O::from).into_token_stream()).collect()
}

impl From<OutputSheetContent> for OutputRuleContent {
    fn from(c: OutputSheetContent) -> Self {
        match c {
            OutputSheetContent::QualifiedRule(block) => Self::Block(block),
            OutputSheetContent::AtRule(rule) => Self::AtRule(rule),
        }
    }
}

impl From<OutputSheetContent> for OutputScopeContent {
    fn from(c: OutputSheetContent) -> Self {
        match c {
            OutputSheetContent::QualifiedRule(block) => Self::Block(block),
            OutputSheetContent::AtRule(rule) => Self::AtRule(rule),
        }
    }
}

impl CssQualifiedRule {
    fn fold_in_context(
        self,
        ctx: CssScopeQualifier,
    ) -> Box<dyn Iterator<Item = ParseResult<OutputSheetContent>>> {
        let own_ctx = self.qualifier;
        if !own_ctx.qualifiers.is_empty() && !ctx.qualifiers.is_empty() {
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
            let err = ParseError::new_spanned(own_ctx, "Can not nest qualified blocks (yet)");
            return Box::new(std::iter::once(Err(err)));
        }
        let relevant_ctx = if !own_ctx.qualifiers.is_empty() {
            own_ctx
        } else {
            ctx
        };
        Box::new(fold_tokens_impl(relevant_ctx, self.scope.contents))
    }
}

impl CssAtRule {
    fn fold_in_context(self, ctx: CssScopeQualifier) -> OutputSheetContent {
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

impl CssRootNode {
    pub fn into_output(self) -> OutputSheet {
        let contents = reify_scope_contents::<OutputScopeContent, _>(
            fold_normalized_scope_hierarchy(self.root_contents),
        );
        OutputSheet { contents }
    }
}

impl CssScopeQualifier {
    fn into_output(self) -> OutputQualifier {
        OutputQualifier {
            selectors: self.qualifiers,
            errors: self.qualifier_errors,
        }
    }
}

impl CssAttribute {
    fn into_output(self) -> OutputAttribute {
        let key_tokens = self.name.reify_name();
        let values = self.value.values;

        OutputAttribute {
            key: key_tokens,
            values,
        }
    }
}

impl CssAttributeName {
    fn reify_name(self) -> TokenStream {
        match self {
            Self::Identifier(name) => {
                let quoted_literal = name.quote_attribute();
                quote! { #quoted_literal.into() }
            }
            Self::InjectedExpr(expr) => expr.reify(),
        }
    }
}
