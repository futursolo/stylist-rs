use super::{ContextRecorder, Reify};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, LitStr};

#[derive(Debug)]
pub enum OutputCowString {
    Str(String),
    Raw(TokenStream, ContextRecorder),
}

impl From<String> for OutputCowString {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl OutputCowString {
    pub fn from_displayable_spanned(source: impl Spanned, expr: impl Reify) -> Self {
        let mut inner_context = ContextRecorder::default();
        let expr = expr.into_token_stream(&mut inner_context);
        inner_context.uses_static(); // .to_string().into()
        Self::Raw(
            quote_spanned! {source.span()=>
                (&{ #expr } as &dyn ::std::fmt::Display).to_string().into()
            },
            inner_context,
        )
    }
}

impl Reify for OutputCowString {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::Raw(t, ref inner) => {
                ctx.uses_nested(inner);
                t
            }
            Self::Str(lit) => {
                let lit_str = LitStr::new(lit.as_ref(), Span::call_site());
                quote! {
                    ::std::borrow::Cow::<str>::Borrowed(
                            #lit_str
                    )
                }
            }
        }
    }
}
