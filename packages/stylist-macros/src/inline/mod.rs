mod component_value;
mod css_ident;

mod output;
mod parse;

use log::debug;
use output::{AllowedUsage, Reify};
use parse::CssRootNode;
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };
    debug!("Parsed as: {:?}", root);

    let (quoted_sheet, allowed_usage) = root.into_output().into_context_aware_tokens().into_value();
    if AllowedUsage::Static <= allowed_usage {
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
