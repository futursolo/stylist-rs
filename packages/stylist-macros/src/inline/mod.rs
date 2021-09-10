pub mod component_value;
pub mod css_ident;

mod parse;

use crate::output::{ContextRecorder, Reify};
use log::debug;
use parse::{CssRootNode, IntoOutputContext};
use proc_macro2::TokenStream;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };

    debug!("Parsed as: {:?}", root);

    let into_output_ctx = IntoOutputContext::new();
    let output_root = root.into_output(&mut into_output_ctx);

    if let Some(m) = into_output_ctx.into_compile_errors() {
        m
    } else {
        let mut ctx = ContextRecorder::new();
        output_root.into_token_stream(&mut ctx)
    }
}
