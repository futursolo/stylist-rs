mod component_value;
mod css_ident;
mod output;
mod parsed;
mod spacing_iterator;

use log::debug;
use output::Reify;
use parsed::CssRootNode;
use proc_macro2::TokenStream;

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };
    debug!("Parsed as: {:?}", root);

    root.into_output().reify()
}
