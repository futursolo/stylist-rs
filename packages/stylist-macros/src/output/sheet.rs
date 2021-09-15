use super::{IntoCowVecTokens, OutputScopeContent, Reify, ReifyContext};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct OutputSheet {
    pub contents: Vec<OutputScopeContent>,
}

impl Reify for OutputSheet {
    fn into_token_stream(self, ctx: &mut ReifyContext) -> TokenStream {
        let contents = self
            .contents
            .into_cow_vec_tokens(quote! {::stylist::ast::ScopeContent}, ctx);

        ctx.uses_static(); // Sheet::from
        let quoted_sheet = quote! {
            {
                use ::std::convert::{From, Into};
                use ::stylist::ast::Sheet;
                Sheet::from(#contents)
            }
        };

        if ctx.is_static() {
            quote! { {
                use ::stylist::macros::vendor::once_cell::sync::Lazy;

                static SHEET_REF: Lazy<::stylist::ast::Sheet> = Lazy::new(
                    || #quoted_sheet
                );

                SHEET_REF.clone()
            } }
        } else {
            quoted_sheet
        }
    }
}
