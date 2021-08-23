mod css_name;
mod output;
mod parsed;

use log::debug;
use output::{OutputSheet, Reify};
use parsed::CssRootNode;
use proc_macro2::TokenStream;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };
    debug!("Parsed as: {:?}", root);

    use std::convert::TryInto;
    <CssRootNode as TryInto<OutputSheet>>::try_into(root).reify()
}
