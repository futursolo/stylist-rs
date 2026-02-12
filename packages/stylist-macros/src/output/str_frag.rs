use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Lit};

use super::{OutputCowString, Reify, ReifyContext};
use crate::inline::component_value::PreservedToken;
use crate::inline::css_ident::CssIdent;
use crate::literal::argument::Argument;

#[derive(Debug, Clone)]
pub enum OutputFragment {
    Expr(Expr),
    Arg(Argument),
    Token(PreservedToken),
    Delimiter(Delimiter, /* start: */ bool),
    Str(String),
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

impl From<Expr> for OutputFragment {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl From<Argument> for OutputFragment {
    fn from(arg: Argument) -> Self {
        Self::Arg(arg)
    }
}

impl OutputFragment {
    pub fn into_inner(self) -> OutputCowString {
        match self {
            Self::Token(t) => t.to_output_string().into(),
            Self::Delimiter(kind, start) => Self::str_for_delim(kind, start).to_string().into(),
            Self::Str(s) => s.into(),
            Self::Arg(arg) => OutputCowString::from_displayable_spanned(arg.name_token, arg.tokens),
            Self::Expr(expr) => {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(ref litstr),
                    ..
                }) = expr
                {
                    litstr.value().into()
                } else {
                    OutputCowString::from_displayable_spanned(expr.span(), expr)
                }
            }
        }
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

    fn as_string(&self) -> Option<String> {
        if let OutputCowString::Str(s) = self.clone().into_inner() {
            Some(s)
        } else {
            None
        }
    }
}

impl Reify for OutputFragment {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        let inner = self.into_inner().into_token_stream(ctx);
        quote! {
            ::stylist::ast::StringFragment {
                inner: #inner
            }
        }
    }
}

#[allow(clippy::result_large_err)]
pub fn fragment_coalesce(
    l: OutputFragment,
    r: OutputFragment,
) -> Result<OutputFragment, (OutputFragment, OutputFragment)> {
    match (l.as_string(), r.as_string()) {
        (Some(lt), Some(rt)) => {
            // Two successive string literals can be combined into a single one
            Ok(OutputFragment::Str(format!("{lt}{rt}")))
        }
        _ => Err((l, r)),
    }
}
