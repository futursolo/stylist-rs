pub mod component_value;
pub mod css_ident;

mod parse;

use crate::output::{ContextRecorder, Reify};
use log::debug;
use parse::CssRootNode;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Error as ParseError;

fn error_to_token_stream(errors: Vec<ParseError>) -> TokenStream {
    let tokens: Vec<TokenStream> = errors.into_iter().map(|m| m.into_compile_error()).collect();

    quote! {
        {
            { #( #tokens )* }

            ::stylist::ast::Sheet::from(Vec::new())
        }
    }
}

pub fn macro_fn(input: TokenStream) -> TokenStream {
    let root = match syn::parse2::<CssRootNode>(input) {
        Ok(parsed) => parsed,
        Err(failed) => return failed.to_compile_error(),
    };

    debug!("Parsed as: {:?}", root);

    let mut ctx = ContextRecorder::new();
    match root.into_output() {
        Ok(m) => m.into_token_stream(&mut ctx),
        Err(e) => error_to_token_stream(e),
    }
}
