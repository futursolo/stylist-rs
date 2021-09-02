use super::{ContextRecorder, Reify};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, LitStr};

#[derive(Debug)]
pub enum OutputCowString {
    Str(String),
    Raw(TokenStream),
}

impl From<String> for OutputCowString {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl OutputCowString {
    pub fn from_displayable_spanned(source: impl Spanned, expr: impl ToTokens) -> Self {
        Self::Raw(quote_spanned! {source.span()=>
            (&{ #expr } as &dyn ::std::fmt::Display).to_string().into()
        })
    }
}

impl Reify for OutputCowString {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::Raw(t) => {
                ctx.uses_dynamic_argument();
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
