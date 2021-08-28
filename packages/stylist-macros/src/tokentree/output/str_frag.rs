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
}

impl Reify for OutputFragment {
    fn reify(self) -> TokenStream {
        match self {
            Self::Raw(t) => t,
            Self::Str(lit) => quote! { #lit.into() },
            Self::Token(t) => Self::from(t.quote_literal()).reify(),
            Self::Delimiter(kind, start) => {
                let lit = LitStr::new(Self::str_for_delim(kind, start), Span::call_site());
                Self::from(lit).reify()
            }
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
