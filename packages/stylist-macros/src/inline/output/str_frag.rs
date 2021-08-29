use super::{
    super::{component_value::PreservedToken, css_ident::CssIdent},
    MaybeStatic, Reify,
};
use proc_macro2::{Delimiter, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Expr, ExprLit, Ident, Lit, LitStr};

#[derive(Debug)]
pub enum OutputFragment {
    Raw(TokenStream),
    Token(PreservedToken),
    Str(LitStr),
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
            ' ' => Self::Str(LitStr::new(" ", Span::call_site())),
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

impl From<LitStr> for OutputFragment {
    fn from(t: LitStr) -> Self {
        Self::Str(t)
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
            return Self::Str(litstr.clone());
        }

        let ident_result = Ident::new("expr", Span::mixed_site());
        let ident_write_expr = Ident::new("write_expr", Span::mixed_site());
        // quote spanned here so that errors related to calling #ident_write_expr show correctly
        let quoted = quote_spanned! {expr.span()=>
            {
                fn #ident_write_expr<V: ::std::fmt::Display>(v: V) -> ::std::string::String {
                    use ::std::fmt::Write;
                    let mut #ident_result = ::std::string::String::new();
                    ::std::write!(&mut #ident_result, "{}", v).expect("");
                    #ident_result
                }
                #ident_write_expr(#expr).into()
            }
        };
        Self::Raw(quoted)
    }
}

impl OutputFragment {
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
    fn reify_str_value(self) -> Result<LitStr, TokenStream> {
        match self {
            Self::Raw(t) => Err(t),
            Self::Str(s) => Ok(s),
            Self::Token(t) => Ok(t.to_lit_str()),
            Self::Delimiter(kind, start) => Ok(LitStr::new(
                Self::str_for_delim(kind, start),
                Span::call_site(),
            )),
        }
    }
}

impl Reify for OutputFragment {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        match self.reify_str_value() {
            Err(t) => MaybeStatic::dynamic(t),
            Ok(lit) => MaybeStatic::statick(quote! { #lit.into() }),
        }
    }
}

pub fn fragment_spacing(l: &OutputFragment, r: &OutputFragment) -> Option<OutputFragment> {
    use OutputFragment::*;
    use PreservedToken::*;
    let needs_spacing = matches!(
        (l, r),
        (Delimiter(_, false), Token(Ident(_)))
            | (
                Token(Ident(_)) | Token(Literal(_)),
                Token(Ident(_)) | Token(Literal(_))
            )
    );
    needs_spacing.then(|| ' '.into())
}

pub fn fragment_coalesce(
    l: OutputFragment,
    r: OutputFragment,
) -> Result<OutputFragment, (OutputFragment, OutputFragment)> {
    match (l.reify_str_value(), r.reify_str_value()) {
        (Err(lt), Err(rt)) => Err((OutputFragment::Raw(lt), OutputFragment::Raw(rt))),
        (Ok(lt), Err(rt)) => Err((OutputFragment::Str(lt), OutputFragment::Raw(rt))),
        (Err(lt), Ok(rt)) => Err((OutputFragment::Raw(lt), OutputFragment::Str(rt))),
        (Ok(lt), Ok(rt)) => {
            // Two successive string literals can be combined into a single one
            let combined = lt.value() + &rt.value();
            let lit = LitStr::new(&combined, Span::call_site());
            Ok(OutputFragment::Str(lit))
        }
    }
}
