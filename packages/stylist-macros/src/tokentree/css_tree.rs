use super::css_name::{Identifier, PunctuatedName};
use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{quote, quote_spanned};
use std::convert::TryInto;
use std::iter::FromIterator;
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
struct InjectedExpression {
    dollar: token::Dollar,
    braces: token::Brace,
    expr: Box<Expr>,
}

#[derive(Debug, Clone)]
enum CssComponentValue {
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
struct CssScopeQualifier {
    qualifiers: Punctuated<CssComponentValue, Token![,]>,
}

#[derive(Debug)]
struct CssScope {
    brace: token::Brace,
    contents: Vec<CssScopeContent>,
}

#[derive(Debug)]
struct CssQualifiedRule {
    qualifier: CssScopeQualifier,
    scope: CssScope,
}

#[derive(Debug)]
enum CssScopeContent {
    Attribute(CssAttribute),
    AtRule(CssAtRule),
    Nested(CssQualifiedRule),
}

#[derive(Debug)]
enum CssAttributeName {
    Identifier(Identifier),
    InjectedExpr(InjectedExpression),
}

#[derive(Debug)]
struct CssAttributeValue {
    values: Punctuated<CssComponentValue, Token![,]>,
}

#[derive(Debug)]
struct CssAttribute {
    name: CssAttributeName,
    colon: token::Colon,
    value: CssAttributeValue,
    terminator: token::Semi,
}

#[derive(Debug)]
enum CssAtRuleContent {
    Scope(CssScope),
    Empty(token::Semi),
}

#[derive(Debug)]
struct CssAtRule {
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
// =====================================================================
// =====================================================================
// Output structs
// =====================================================================
// =====================================================================
#[derive(Debug)]
pub struct OutputSheet {
    contents: Vec<OutputSheetContent>,
}
#[derive(Debug)]
enum OutputSheetContent {
    AtRule(OutputAtRule),
    QualifiedRule(OutputQualifiedRule),
}
#[derive(Debug)]
struct OutputAtRule {
    name: Identifier,
    prelude: Vec<CssComponentValue>,
    contents: Vec<OutputSheetContent>,
}
#[derive(Debug)]
struct OutputQualifiedRule {
    qualifier: CssScopeQualifier,
    attributes: Vec<CssAttribute>,
}

impl OutputSheet {
    pub fn into_token_stream(self) -> TokenStream {
        let Self { contents } = self;
        let ident_stylesheet = Ident::new("stylesheet", Span::mixed_site());
        let ident_part = Ident::new("part", Span::mixed_site());

        let blocks = contents.into_iter().map(|c| {
            let scope_tokens = match c {
                OutputSheetContent::QualifiedRule(block) => block.into_scope_content(),
                OutputSheetContent::AtRule(rule) => rule.into_scope_content(),
            };
            quote! {
                let #ident_part = { #scope_tokens };
                #ident_stylesheet.0.push( #ident_part );
            }
        });

        quote! {
            {
                let mut #ident_stylesheet = ::stylist::ast::Sheet::new();
                #( #blocks )*
                #ident_stylesheet
            }
        }
    }
}

impl OutputAtRule {
    fn into_token_stream(self) -> TokenStream {
        let ident_condition = Ident::new("at_condition", Span::mixed_site());
        let ident_attributes = Ident::new("attributes", Span::mixed_site());
        let name_tokens = self.name.quote_at_rule();
        let prelude_tokens = self.prelude.into_iter().map(|p| {
            let write_component = p.write_component(&ident_condition);
            quote! {
                ::std::write!(&mut #ident_condition, " ").expect("");
                #write_component
            }
        });
        let content_tokens = self.contents.into_iter().map(|c| {
            let scope_tokens = match c {
                OutputSheetContent::QualifiedRule(block) => block.into_rule_content(),
                OutputSheetContent::AtRule(rule) => rule.into_rule_content(),
            };
            quote! {
                let part = { #scope_tokens };
                #ident_attributes.push(part);
            }
        });

        quote! {
            {
                use ::std::fmt::Write;
                let mut #ident_condition = ::std::string::String::new();
                ::std::write!(&mut #ident_condition, "@{}", #name_tokens).expect("");
                #( #prelude_tokens )*

                let mut #ident_attributes = ::std::vec::Vec::new();
                #( #content_tokens )*
                ::stylist::ast::Rule {
                    condition: #ident_condition,
                    content: #ident_attributes,
                }
            }
        }
    }
    pub fn into_scope_content(self) -> TokenStream {
        let block_tokens = self.into_token_stream();
        quote! {
            ::stylist::ast::ScopeContent::Rule( #block_tokens )
        }
    }
    pub fn into_rule_content(self) -> TokenStream {
        let block_tokens = self.into_token_stream();
        quote! {
            ::stylist::ast::RuleContent::Rule( #block_tokens )
        }
    }
}

impl OutputQualifiedRule {
    fn into_token_stream(self) -> TokenStream {
        let ident_attributes = Ident::new("attributes", Span::mixed_site());
        let ident_attr = Ident::new("attr", Span::mixed_site());
        let condition = self.qualifier.into_token_stream();
        let attr_tokens = self.attributes.into_iter().map(|attr| {
            let attr_tokens = attr.into_token_stream();
            quote! {
                let #ident_attr = { #attr_tokens };
                #ident_attributes.push( #ident_attr );
            }
        });

        quote! {
            {
                let mut #ident_attributes = ::std::vec::Vec::new();
                #( #attr_tokens )*

                ::stylist::ast::Block {
                    condition: ::std::option::Option::Some(#condition),
                    style_attributes: #ident_attributes,
                }
            }
        }
    }

    pub fn into_scope_content(self) -> TokenStream {
        let block_tokens = self.into_token_stream();
        quote! {
            ::stylist::ast::ScopeContent::Block( #block_tokens )
        }
    }
    pub fn into_rule_content(self) -> TokenStream {
        let block_tokens = self.into_token_stream();
        quote! {
            ::stylist::ast::RuleContent::Block( #block_tokens )
        }
    }
}

impl CssAttribute {
    fn into_token_stream(self) -> TokenStream {
        let ident_writable_name = Ident::new("writer_name", Span::mixed_site());
        let ident_writable_value = Ident::new("writer_value", Span::mixed_site());
        let name_tokens = self.name.write_qualifier(&ident_writable_name);
        let value_tokens = self.value.write_value(&ident_writable_value);

        quote! {
            {
                use ::std::fmt::Write;
                let mut #ident_writable_name = ::std::string::String::new();
                #name_tokens
                let mut #ident_writable_value = ::std::string::String::new();
                #value_tokens

                ::stylist::ast::StyleAttribute {
                    key: #ident_writable_name,
                    value: #ident_writable_value,
                }
            }
        }
    }
}

impl CssScopeQualifier {
    fn into_token_stream(self) -> TokenStream {
        let ident_qualifier = Ident::new("writer", Span::mixed_site());
        let items = self
            .qualifiers
            .into_pairs()
            .map(|p| p.into_value().write_component(&ident_qualifier));

        let items = Itertools::intersperse(
            items,
            quote! {
                ::std::write!(#ident_qualifier, ", ").expect("");
            },
        );

        quote! {
            {
                use ::std::fmt::Write;
                let mut #ident_qualifier = ::std::string::String::new();
                #( #items )*
                #ident_qualifier
            }
        }
    }
}

impl CssComponentValue {
    pub fn write_component(self, writable: &Ident) -> TokenStream {
        match self {
            Self::Identifier(name) => {
                let name_toks = name.quote();
                quote! {
                    ::std::write!(&mut #writable, "{}", #name_toks).expect("");
                }
            }
            Self::InjectedExpr(expr) => {
                let injected = *expr.expr;
                quote_spanned! {expr.braces.span=>
                    {
                        fn write_expr<V: ::std::fmt::Display>(w: &mut String, v: V) {
                            ::std::write!(w, "{}", v).expect("");
                        }
                        let expr = { #injected };
                        write_expr(&mut #writable, expr);
                    }
                }
            }
            Self::ResultClass(_) => {
                quote! {
                    ::std::write!(&mut #writable, "&").expect("");
                }
            }
            Self::Function { name, args, .. } => {
                let name_toks = name.quote();
                let write_args = args.into_iter().map(|arg| arg.write_component(writable));
                let write_args = Itertools::intersperse(
                    write_args,
                    quote! {
                        ::std::write!(&mut #writable, ", ").expect("");
                    },
                );
                quote! {
                    ::std::write!(&mut #writable, "{}(", #name_toks).expect("");
                    #( #write_args )*
                    ::std::write!(&mut #writable, ")").expect("");
                }
            }
        }
    }
}

impl CssAttributeName {
    pub fn write_qualifier(self, writable: &Ident) -> TokenStream {
        match self {
            Self::Identifier(name) => {
                let name_toks = name.quote_attribute();
                quote! {
                    ::std::write!(&mut #writable, "{}", #name_toks).expect("");
                }
            }
            Self::InjectedExpr(expr) => {
                let injected = *expr.expr;
                quote_spanned! {expr.braces.span=>
                    {
                        fn write_expr<V: ::std::fmt::Display>(w: &mut str, v: V) {
                            ::std::write!(w, "{}", v).expect("");
                        }
                        let expr = { #injected };
                        write_expr(&mut #writable, expr);
                    }
                }
            }
        }
    }
}

impl CssAttributeValue {
    pub fn write_value(self, writable: &Ident) -> TokenStream {
        let values = self
            .values
            .into_pairs()
            .map(|p| p.into_value().write_component(writable));
        let values = Itertools::intersperse(
            values,
            quote! {
                ::std::write!(&mut #writable, ", ").expect("");
            },
        );

        quote! {
            #( #values )*
        }
    }
}
// =====================================================================
// =====================================================================
// Convert into output
// =====================================================================
// =====================================================================
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

enum CollectTokenError<T> {
    Result(T),
    Error(TokenStream),
}

impl<T> CollectTokenError<T> {
    fn into_error(self) -> Result<T, TokenStream> {
        match self {
            CollectTokenError::Result(t) => Ok(t),
            CollectTokenError::Error(e) => Err(e),
        }
    }
}

impl<I, T> FromIterator<Result<I, TokenStream>> for CollectTokenError<T>
where
    T: FromIterator<I>,
{
    fn from_iter<It>(it: It) -> Self
    where
        It: IntoIterator<Item = Result<I, TokenStream>>,
    {
        let mut items = Vec::new();
        let mut errors = Vec::new();
        for i in it.into_iter() {
            match i {
                Ok(i) => items.push(i),
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            CollectTokenError::Result(T::from_iter(items))
        } else {
            CollectTokenError::Error(TokenStream::from_iter(errors))
        }
    }
}

impl TryInto<OutputSheet> for CssRootNode {
    type Error = TokenStream;
    fn try_into(self) -> Result<OutputSheet, TokenStream> {
        let contents = fold_tokens(self.root_contents, &CssScopeQualifier::default())
            .collect::<CollectTokenError<_>>()
            .into_error()?;
        Ok(OutputSheet { contents })
    }
}

fn fold_tokens<'it>(
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
    context: &'it CssScopeQualifier,
) -> impl 'it + Iterator<Item = Result<OutputSheetContent, TokenStream>> {
    // Step one: collect attributes into blocks, flatten and lift nested blocks
    struct Wrapper<'a, I: Iterator> {
        it: Peekable<I>,
        context: &'a CssScopeQualifier,
    }

    enum WrappedCase {
        AttributeCollection(OutputQualifiedRule),
        AtRule(CssAtRule),
        Qualified(CssQualifiedRule),
    }

    impl<'a, I: Iterator<Item = CssScopeContent>> Iterator for Wrapper<'a, I> {
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
                        attributes.push(attr)
                    }
                    let replacement = OutputQualifiedRule {
                        qualifier: self.context.clone(),
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
        context,
    }
    .flat_map(move |w| match w {
        WrappedCase::AttributeCollection(attrs) => {
            let content = Ok(OutputSheetContent::QualifiedRule(attrs));
            let it = std::iter::once(content);
            Box::new(it) as Box<dyn '_ + Iterator<Item = _>>
        }
        WrappedCase::AtRule(r) => {
            let inner = r.try_into_ctx(context);
            Box::new(std::iter::once(inner))
        }
        WrappedCase::Qualified(b) => b.try_into_ctx(context),
    })
}

impl CssQualifiedRule {
    fn try_into_ctx(
        self,
        ctx: &CssScopeQualifier,
    ) -> Box<dyn '_ + Iterator<Item = Result<OutputSheetContent, TokenStream>>> {
        let own_ctx = &self.qualifier;
        if !own_ctx.qualifiers.is_empty() && !ctx.qualifiers.is_empty() {
            // TODO: figure out how to combine contexts
            let err = quote_spanned! {own_ctx.span()=>
                {
                    ::std::compile_error!("Can not nest qualified blocks (yet)")
                }
            };
            return Box::new(std::iter::once(Err(err)));
        }
        let relevant_ctx = if !own_ctx.qualifiers.is_empty() {
            own_ctx
        } else {
            ctx
        };
        // FIXME: no real reason to collect here
        // it's a bit tricky, since 'relevant_ctx' borrows local ctx
        #[allow(clippy::needless_collect)]
        let collected = fold_tokens(self.scope.contents, relevant_ctx).collect::<Vec<_>>();
        Box::new(collected.into_iter())
    }
}

impl CssAtRule {
    fn try_into_ctx(self, ctx: &CssScopeQualifier) -> Result<OutputSheetContent, TokenStream> {
        let contents = match self.contents {
            CssAtRuleContent::Empty(_) => Vec::new(),
            CssAtRuleContent::Scope(scope) => fold_tokens(scope.contents, ctx)
                .collect::<CollectTokenError<Vec<_>>>()
                .into_error()?,
        };
        Ok(OutputSheetContent::AtRule(OutputAtRule {
            name: self.name,
            prelude: self.prelude,
            contents,
        }))
    }
}
