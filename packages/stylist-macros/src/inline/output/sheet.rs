use crate::inline::output::maybe_static::ExpressionContext;

use super::{MaybeStatic, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputSheet {
    pub contents: MaybeStatic<Vec<TokenStream>>,
}

impl Reify for OutputSheet {
    fn into_token_stream(self) -> MaybeStatic<TokenStream> {
        let Self { contents } = self;
        let (contents, content_context) = contents
            .into_cow_vec_tokens(quote! {::stylist::ast::ScopeContent})
            .into_value();

        let quoted_sheet = quote! {
            {
                use ::std::convert::{From, Into};
                ::stylist::ast::Sheet::from(#contents)
            }
        };
        if ExpressionContext::Static <= content_context {
            MaybeStatic::statick(quote! { {
                use ::stylist::vendor::once_cell::sync::Lazy;

                static SHEET_REF: Lazy<::stylist::ast::Sheet> = Lazy::new(
                    || #quoted_sheet
                );

                SHEET_REF.clone()
            } })
        } else {
            MaybeStatic::dynamic(quoted_sheet)
        }
    }
}
