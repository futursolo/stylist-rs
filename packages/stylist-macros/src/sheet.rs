use proc_macro2::TokenStream;

use std::collections::HashMap;

use litrs::StringLit;
use proc_macro_error::{abort, abort_call_site};
use std::convert::TryFrom;

use stylist_core::ast::Sheet;

use crate::to_tokens_with_args::ToTokensWithArgs;

pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    let first_token = match tokens.next() {
        Some(m) => m,
        None => abort_call_site!("expected at least one argument"),
    };

    let s_literal = match StringLit::try_from(first_token.clone()) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error2(),
    };

    let sheet: Sheet = match s_literal.value().parse() {
        Ok(m) => m,

        Err(e) => abort!(first_token.span(), "{}", e.to_string()),
    };

    let args = HashMap::new();

    sheet.to_tokens_with_args(&args)
}
