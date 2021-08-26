use convert_case::{Case, Casing};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Lookahead1, Parse, ParseBuffer, Result as ParseResult},
    spanned::Spanned,
    token, Ident, LitStr,
};

syn::custom_punctuation!(DoubleSub, --);

#[derive(Debug, Clone)]
pub enum IdentPart {
    Dash(Punct),
    Ident(Ident),
}

#[derive(Debug, Clone)]
pub struct CssIdent {
    parts: Vec<IdentPart>,
}

impl IdentPart {
    pub fn peek(lookahead: &Lookahead1, accept_dash: bool, accept_ident: bool) -> bool {
        let peek_dash = accept_dash && (lookahead.peek(token::Sub) || lookahead.peek(DoubleSub));
        let peek_ident = accept_ident && lookahead.peek(Ident::peek_any);
        peek_dash || peek_ident
    }
}

impl CssIdent {
    pub fn peek(lookahead: &Lookahead1) -> bool {
        IdentPart::peek(lookahead, true, true)
    }

    pub fn stringify(&self) -> String {
        self.parts
            .iter()
            .map(|p| match p {
                IdentPart::Dash(_) => "-".into(),
                IdentPart::Ident(t) => format!("{}", t.unraw()),
            })
            .collect()
    }

    pub fn quote_literal(&self) -> LitStr {
        let formatted = self.stringify();
        LitStr::new(&formatted, self.span())
    }

    pub fn quote_at_rule(&self) -> LitStr {
        let formatted = self.stringify().from_case(Case::Camel).to_case(Case::Kebab);
        LitStr::new(&formatted, self.span())
    }

    pub fn quote_attribute(&self) -> LitStr {
        let formatted = self.stringify().from_case(Case::Camel).to_case(Case::Kebab);
        LitStr::new(&formatted, self.span())
    }
}

impl IdentPart {
    fn parse_part(
        input: &ParseBuffer,
        accept_dash: bool,
        accept_ident: bool,
    ) -> ParseResult<IdentPart> {
        debug_assert!(accept_dash || accept_ident);
        let lookahead = input.lookahead1();
        if accept_dash && (lookahead.peek(token::Sub) || lookahead.peek(DoubleSub)) {
            let dash = input.parse::<Punct>()?;
            debug_assert!(dash.as_char() == '-', "expected a - character");
            Ok(IdentPart::Dash(dash))
        } else if accept_ident && lookahead.peek(Ident::peek_any) {
            Ok(IdentPart::Ident(input.call(Ident::parse_any)?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for CssIdent {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let mut parts = vec![IdentPart::parse_part(input, true, true)?];
        loop {
            let (joins_dash, joins_idents) = match parts.last().unwrap() {
                // Dashes always join identifiers, and only over dashes if jointly spaced
                IdentPart::Dash(s) => (s.spacing() == Spacing::Joint, true),
                // Identifiers join dashes, but never other dashes
                IdentPart::Ident(_) => (true, false),
            };
            if !IdentPart::peek(&input.lookahead1(), joins_dash, joins_idents) {
                break;
            }
            parts.push(IdentPart::parse_part(input, joins_dash, joins_idents)?);
        }
        Ok(Self { parts })
    }
}

impl ToTokens for IdentPart {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Self::Dash(d) => d.to_tokens(toks),
            Self::Ident(i) => i.to_tokens(toks),
        }
    }
}

impl ToTokens for CssIdent {
    fn to_tokens(&self, toks: &mut TokenStream) {
        for p in self.parts.iter() {
            p.to_tokens(toks);
        }
    }
}
