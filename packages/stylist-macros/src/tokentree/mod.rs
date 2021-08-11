mod css_name;
mod css_tree;

use css_tree::{CssRootNode, OutputSheet};
use log::debug;
use proc_macro2::TokenStream;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };
    debug!("Parsed as: {:?}", root);

    use std::convert::TryInto;
    match <CssRootNode as TryInto<OutputSheet>>::try_into(root) {
        Ok(parsed) => parsed.into_token_stream(),
        Err(failed) => failed,
    }
}
