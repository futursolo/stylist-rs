use super::css_name::{Identifier, PunctuatedName};
use super::output::{
    OutputAtRule, OutputAttribute, OutputQualifiedRule, OutputQualifier, OutputSheet, Reify,
};
use crate::tokentree::output::OutputRuleContent;
use crate::tokentree::output::OutputScopeContent;
use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{quote, quote_spanned};
use std::iter::Peekable;
use syn::parse::{Parse, ParseBuffer, Result as ParseResult};
use syn::spanned::Spanned;
use syn::Ident;
use syn::{braced, parenthesized, token};
use syn::{punctuated::Punctuated, Expr, Token};

#[derive(Debug)]
pub struct CssRootNode {
    root_contents: Vec<CssScopeContent>,
}

#[derive(Debug, Clone)]
pub struct InjectedExpression {
    dollar: token::Dollar,
    braces: token::Brace,
    expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum CssComponentValue {
    Identifier(PunctuatedName),
    InjectedExpr(InjectedExpression),
    ResultClass(token::And),
    Function {
        name: PunctuatedName,
        brace: token::Paren,
        args: Punctuated<CssComponentValue, Token![,]>,
    },
}

#[derive(Debug, Clone)]
pub struct CssScopeQualifier {
    qualifiers: Punctuated<CssComponentValue, Token![,]>,
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
    Identifier(Identifier),
    InjectedExpr(InjectedExpression),
}

#[derive(Debug)]
pub struct CssAttributeValue {
    values: Punctuated<CssComponentValue, Token![,]>,
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
    name: Identifier,
    prelude: Vec<CssComponentValue>,
    contents: CssAtRuleContent,
}
// =====================================================================
// =====================================================================
// Parsing implementation
// =====================================================================
// =====================================================================
impl Parse for CssRootNode {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let root_contents = CssScope::parse_contents(input)?;
        Ok(Self { root_contents })
    }
}

impl CssScope {
    fn parse_contents(input: &ParseBuffer) -> ParseResult<Vec<CssScopeContent>> {
        let mut contents = Vec::new();
        while !input.is_empty() {
            contents.push(input.parse()?);
        }
        Ok(contents)
    }
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let brace = braced!(inner in input);
        let contents = Self::parse_contents(&inner)?;
        Ok(Self { brace, contents })
    }
}

impl Parse for CssScopeContent {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::At) {
            let atrule = input.parse()?;
            return Ok(Self::AtRule(atrule));
        }
        let next_header: CssScopeQualifier = input.parse()?;
        let lookahead = input.lookahead1();
        if let Some(name) = next_header.can_be_attribute() {
            if lookahead.peek(token::Colon) {
                // An attribute!
                let colon = input.parse()?;
                let value = input.parse()?;
                let terminator = input.parse()?;
                return Ok(Self::Attribute(CssAttribute {
                    colon,
                    name,
                    value,
                    terminator,
                }));
            }
        }
        if lookahead.peek(token::Brace) {
            let nested = input.parse()?;
            Ok(Self::Nested(CssQualifiedRule {
                qualifier: next_header,
                scope: nested,
            }))
        } else {
            Err(lookahead.error())
        }
    }
}

impl CssAttributeName {
    fn maybe_from_component(component: &CssComponentValue) -> Option<Self> {
        match component {
            CssComponentValue::ResultClass(_) => None,
            CssComponentValue::Function { .. } => None,
            CssComponentValue::Identifier(id) => {
                Identifier::maybe_from_punctuated(id).map(Self::Identifier)
            }
            CssComponentValue::InjectedExpr(expr) => {
                Some(CssAttributeName::InjectedExpr(expr.clone()))
            }
        }
    }
}

impl CssScopeQualifier {
    fn can_be_attribute(&self) -> Option<CssAttributeName> {
        if self.qualifiers.len() != 1 || self.qualifiers.trailing_punct() {
            return None;
        }
        let first = self.qualifiers.first().unwrap();
        CssAttributeName::maybe_from_component(first)
    }
}

impl Parse for CssComponentValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Dollar) {
            let dollar = input.parse()?;
            let inner;
            let braces = braced!(inner in input);
            let expr = Box::new(inner.parse()?);
            Ok(Self::InjectedExpr(InjectedExpression {
                dollar,
                braces,
                expr,
            }))
        } else if lookahead.peek(token::And) {
            let bang = input.parse()?;
            Ok(Self::ResultClass(bang))
        } else if PunctuatedName::peek(&lookahead) {
            let identifier = input.parse()?;
            if input.peek(token::Paren) {
                let inner;
                let brace = parenthesized!(inner in input);
                let args = inner.parse_terminated(CssComponentValue::parse)?;
                Ok(Self::Function {
                    name: identifier,
                    brace,
                    args,
                })
            } else {
                Ok(Self::Identifier(identifier))
            }
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for CssAttributeValue {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let values = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { values })
    }
}

impl Parse for CssScopeQualifier {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let qualifiers = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { qualifiers })
    }
}

impl Parse for CssAtRule {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let at = input.parse()?;
        let name = input.parse()?;
        let mut prelude = Vec::new();
        let contents = loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(token::Brace) {
                let scope = input.parse()?;
                break CssAtRuleContent::Scope(scope);
            } else if lookahead.peek(token::Semi) {
                let semi = input.parse()?;
                break CssAtRuleContent::Empty(semi);
            } else {
                let component = input.parse()?;
                prelude.push(component);
            }
        };

        Ok(Self {
            at,
            name,
            prelude,
            contents,
        })
    }
}
impl ToTokens for InjectedExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dollar.to_tokens(tokens);
        self.braces.surround(tokens, |toks| {
            self.expr.to_tokens(toks);
        });
    }
}

impl ToTokens for CssComponentValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CssComponentValue::Identifier(name) => name.to_tokens(tokens),
            CssComponentValue::ResultClass(and) => and.to_tokens(tokens),
            CssComponentValue::Function { name, brace, args } => {
                name.to_tokens(tokens);
                brace.surround(tokens, |toks| {
                    args.to_tokens(toks);
                });
            }
            CssComponentValue::InjectedExpr(expr) => expr.to_tokens(tokens),
        }
    }
}

impl ToTokens for CssScopeQualifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.qualifiers.to_tokens(tokens);
    }
}

impl Default for CssScopeQualifier {
    fn default() -> Self {
        Self {
            qualifiers: Punctuated::new(),
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
fn fold_normalized_scope_hierarchy<'it, R: 'it>(
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
    handle_item: impl 'it + Copy + Fn(OutputSheetContent) -> R,
) -> impl 'it + Iterator<Item = Result<R, TokenStream>> {
    fold_tokens_impl(Default::default(), it, handle_item)
}

enum OutputSheetContent {
    AtRule(OutputAtRule),
    QualifiedRule(OutputQualifiedRule),
}

fn fold_tokens_impl<'it, R: 'it>(
    context: CssScopeQualifier,
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
    handle_item: impl 'it + Copy + Fn(OutputSheetContent) -> R,
) -> impl 'it + Iterator<Item = Result<R, TokenStream>> {
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
                        attributes.push(attr.into_output().reify());
                    }
                    let replacement = OutputQualifiedRule {
                        qualifier: self.qualifier.clone().reify(),
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

    let context_conditions = context
        .qualifiers
        .pairs()
        .flat_map(|p| p.into_value().clone().reify())
        .collect();
    Wrapper {
        it: it.into_iter().peekable(),
        qualifier: OutputQualifier {
            selectors: context_conditions,
        },
    }
    .flat_map(move |w| match w {
        WrappedCase::AttributeCollection(attrs) => {
            let result = Ok(handle_item(OutputSheetContent::QualifiedRule(attrs)));
            Box::new(std::iter::once(result)) as Box<dyn '_ + Iterator<Item = _>>
        }
        WrappedCase::AtRule(r) => {
            let result = Ok(handle_item(r.fold_in_context(context.clone())));
            Box::new(std::iter::once(result))
        }
        WrappedCase::Qualified(b) => {
            let nested = b.fold_in_context(context.clone());
            Box::new(nested.map(move |i| i.map(|c| handle_item(c))))
        }
    })
}

impl CssQualifiedRule {
    fn fold_in_context(
        self,
        ctx: CssScopeQualifier,
    ) -> Box<dyn Iterator<Item = Result<OutputSheetContent, TokenStream>>> {
        let own_ctx = self.qualifier;
        if !own_ctx.qualifiers.is_empty() && !ctx.qualifiers.is_empty() {
            // TODO: figure out how to combine contexts
            let err = quote_spanned! {own_ctx.span()=>
                ::std::compile_error!("Can not nest qualified blocks (yet)")
            };
            return Box::new(std::iter::once(Err(err)));
        }
        let relevant_ctx = if !own_ctx.qualifiers.is_empty() {
            own_ctx
        } else {
            ctx
        };
        let collected = fold_tokens_impl(relevant_ctx, self.scope.contents, |c| c);
        Box::new(collected)
    }
}

impl CssAtRule {
    fn fold_in_context(self, ctx: CssScopeQualifier) -> OutputSheetContent {
        let contents = match self.contents {
            CssAtRuleContent::Empty(_) => Vec::new(),
            CssAtRuleContent::Scope(scope) => fold_tokens_impl(ctx, scope.contents, |c| match c {
                OutputSheetContent::QualifiedRule(block) => OutputRuleContent::Block(block),
                OutputSheetContent::AtRule(rule) => OutputRuleContent::AtRule(rule),
            })
            .map(|c| c.reify())
            .collect(),
        };
        let name_lit = self.name.quote_at_rule();
        OutputSheetContent::AtRule(OutputAtRule {
            name: quote! { #name_lit.into() },
            prelude: self.prelude.into_iter().flat_map(|p| p.reify()).collect(),
            contents,
        })
    }
}
// =====================================================================
// =====================================================================
// Output structs + quoting
// =====================================================================
// =====================================================================
impl Reify for InjectedExpression {
    fn reify(self) -> TokenStream {
        let injected = *self.expr;
        let ident_result = Ident::new("expr", Span::mixed_site());
        quote_spanned! {self.braces.span=>
            {
                fn write_expr<V: ::std::fmt::Display>(v: V) -> ::std::string::String {
                    use ::std::fmt::Write;
                    let mut #ident_result = ::std::string::String::new();
                    ::std::write!(&mut #ident_result, "{}", v).expect("");
                    #ident_result
                }
                write_expr(#injected).into()
            }
        }
    }
}

impl CssRootNode {
    pub fn into_output(self) -> OutputSheet {
        let contents = fold_normalized_scope_hierarchy(self.root_contents, |c| match c {
            OutputSheetContent::QualifiedRule(block) => OutputScopeContent::Block(block),
            OutputSheetContent::AtRule(rule) => OutputScopeContent::AtRule(rule),
        })
        .map(|c| c.reify())
        .collect();
        OutputSheet { contents }
    }
}

impl CssAttribute {
    fn into_output(self) -> OutputAttribute {
        let key_tokens = self.name.reify();
        let value_tokens = Itertools::intersperse(
            self.value
                .values
                .into_pairs()
                .flat_map(|p| p.into_value().reify()),
            quote! { ", ".into() },
        )
        .collect();

        OutputAttribute {
            key: key_tokens,
            values: value_tokens,
        }
    }
}

impl CssComponentValue {
    // Reifies into a Vec of TokenStreams of type
    // for<I: Into<Cow<'static, str>>> T: From<I>
    // including ::stylist::ast::Selector and ::stylist::ast::StringFragment
    pub fn reify(self) -> Vec<TokenStream> {
        match self {
            Self::Identifier(name) => {
                let quoted_literal = name.quote_literal();
                vec![quote! { #quoted_literal.into() }]
            }
            Self::InjectedExpr(expr) => {
                vec![expr.reify()]
            }
            Self::ResultClass(_) => {
                vec![quote! { "&".into() }]
            }
            Self::Function { name, args, .. } => {
                let name_toks = name.quote_literal();
                let mut write_args = Itertools::intersperse(
                    args.into_iter().flat_map(|arg| arg.reify()),
                    quote! { ", ".into() },
                )
                .collect::<Vec<_>>();
                write_args.insert(0, quote! { #name_toks.into() });
                write_args.insert(1, quote! { "(".into() });
                write_args.push(quote! { ")".into() });
                write_args
            }
        }
    }
}

impl CssAttributeName {
    pub fn reify(self) -> TokenStream {
        match self {
            Self::Identifier(name) => {
                let quoted_literal = name.quote_attribute();
                vec![quote! { #quoted_literal.into() }]
            }
            Self::InjectedExpr(expr) => expr.reify(),
        }
    }
}
