use super::{
    super::{component_value::PreservedToken, css_ident::CssIdent},
    Reify,
};
use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::LitStr;

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
            Self::Token(t) => Ok(t.quote_literal()),
            Self::Delimiter(kind, start) => Ok(LitStr::new(
                Self::str_for_delim(kind, start),
                Span::call_site(),
            )),
        }
    }
}

impl Reify for OutputFragment {
    fn into_token_stream(self) -> TokenStream {
        match self.reify_str_value() {
            Err(t) => t,
            Ok(lit) => quote! { #lit.into() },
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
            let combined = lt.value() + &rt.value();
            let lit = LitStr::new(&combined, Span::call_site());
            Ok(OutputFragment::Str(lit))
        }
    }
}
