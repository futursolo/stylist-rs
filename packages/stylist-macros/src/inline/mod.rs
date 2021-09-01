pub mod component_value;
pub mod css_ident;

mod parse;

use crate::output::{ContextRecorder, Reify};
use log::debug;
use parse::CssRootNode;
use proc_macro2::TokenStream;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };

    debug!("Parsed as: {:?}", root);

    let mut ctx = ContextRecorder::new();
    root.into_output().into_token_stream(&mut ctx)
}
