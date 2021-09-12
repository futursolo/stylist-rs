use super::{Reify, ReifyContext};
use crate::{
    inline::{component_value::PreservedToken, css_ident::CssIdent},
    literal::argument::Argument,
};
use proc_macro2::{Delimiter, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Expr, ExprLit, Lit, LitStr};

#[derive(Debug, Clone)]
pub enum OutputFragment {
    Raw(TokenStream),
    Token(PreservedToken),
    Str(String),
    Delimiter(Delimiter, /*start:*/ bool),
}

impl From<char> for OutputFragment {
    fn from(c: char) -> Self {
        match c {
            '{' => Self::Delimiter(Delimiter::Brace, true),
            '}' => Self::Delimiter(Delimiter::Brace, false),
            '[' => Self::Delimiter(Delimiter::Bracket, true),
            ']' => Self::Delimiter(Delimiter::Bracket, false),
            '(' => Self::Delimiter(Delimiter::Parenthesis, true),
            ')' => Self::Delimiter(Delimiter::Parenthesis, false),
            ' ' => Self::Str(" ".into()),
            _ => unreachable!("Delimiter {} not recognized", c),
        }
    }
}

impl From<TokenStream> for OutputFragment {
    fn from(t: TokenStream) -> Self {
        Self::Raw(t)
    }
}

impl From<PreservedToken> for OutputFragment {
    fn from(t: PreservedToken) -> Self {
        Self::Token(t)
    }
}

impl From<CssIdent> for OutputFragment {
    fn from(i: CssIdent) -> Self {
        PreservedToken::Ident(i).into()
    }
}

impl<'a> From<&'a Expr> for OutputFragment {
    fn from(expr: &Expr) -> Self {
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(ref litstr),
            ..
        }) = expr
        {
            return Self::Str(litstr.value());
        }

        Self::from_displayable_spanned(expr, expr)
    }
}

impl<'a> From<&'a Argument> for OutputFragment {
    fn from(arg: &Argument) -> Self {
        Self::from_displayable_spanned(&arg.name_token, &arg.tokens)
    }
}

impl OutputFragment {
    fn from_displayable_spanned(source: impl Spanned, expr: impl ToTokens) -> Self {
        OutputFragment::Raw(quote_spanned! {source.span()=>
            (&{ #expr } as &dyn ::std::fmt::Display).to_string().into()
        })
    }

    fn str_for_delim(d: Delimiter, start: bool) -> &'static str {
        match (d, start) {
            (Delimiter::Brace, true) => "{",
            (Delimiter::Brace, false) => "}",
            (Delimiter::Bracket, true) => "[",
            (Delimiter::Bracket, false) => "]",
            (Delimiter::Parenthesis, true) => "(",
            (Delimiter::Parenthesis, false) => ")",
            (Delimiter::None, _) => unreachable!("only actual delimiters allowed"),
        }
    }
    /// Return the string literal that will be quoted, or a full tokenstream
    fn try_into_string(self) -> Result<String, TokenStream> {
        match self {
            Self::Raw(t) => Err(t),
            Self::Str(s) => Ok(s),
            Self::Token(t) => Ok(t.to_output_string()),
            Self::Delimiter(kind, start) => Ok(Self::str_for_delim(kind, start).into()),
        }
    }
    fn as_str(&self) -> Option<String> {
        self.clone().try_into_string().ok()
    }
}

impl Reify for OutputFragment {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        match self.try_into_string() {
            Err(t) => {
                ctx.uses_dynamic_argument();
                t
            }
            Ok(lit) => {
                let lit_str = LitStr::new(lit.as_ref(), Span::call_site());
                quote! { #lit_str.into() }
            }
        }
    }
}

pub fn fragment_coalesce(
    l: OutputFragment,
    r: OutputFragment,
) -> Result<OutputFragment, (OutputFragment, OutputFragment)> {
    match (l.as_str(), r.as_str()) {
        (Some(lt), Some(rt)) => {
            // Two successive string literals can be combined into a single one
            Ok(OutputFragment::Str(format!("{}{}", lt, rt)))
        }
        _ => Err((l, r)),
    }
}
