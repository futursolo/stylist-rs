mod component_value;
mod css_ident;

mod output;
mod parse;

use log::debug;
use output::{ContextRecorder, Reify};
use parse::CssRootNode;
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };

    debug!("Parsed as: {:?}", root);

    let mut ctx = ContextRecorder::new();
    let quoted_sheet = root.into_output().into_token_stream(&mut ctx);

    if ctx.is_static() {
        quote! { {
            use ::stylist::vendor::once_cell::sync::Lazy;

            static SHEET_REF: Lazy<::stylist::ast::Sheet> = Lazy::new(
                || #quoted_sheet
            );

            SHEET_REF.clone()
        } }
    } else {
        quoted_sheet
    }
}
