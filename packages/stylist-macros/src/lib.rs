#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(missing_debug_implementations)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod inline;
mod literal;

mod css;
mod css_var;
mod css_variables;
mod global_style;
mod output;
mod sheet;
mod spacing_iterator;
mod style;
mod styled_component;
mod styled_component_impl;
mod use_style;
mod utils;

#[proc_macro]
#[proc_macro_error]
pub fn sheet(input: TokenStream) -> TokenStream {
    sheet::macro_fn(input.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn style(input: TokenStream) -> TokenStream {
    style::macro_fn(input.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn global_style(input: TokenStream) -> TokenStream {
    global_style::macro_fn(input.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    css::macro_fn(input.into()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn use_style(input: TokenStream) -> TokenStream {
    use_style::macro_fn(input.into()).into()
}

#[proc_macro_attribute]
pub fn styled_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    styled_component::macro_fn(attr, item)
}

#[proc_macro_attribute]
pub fn styled_component_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    styled_component_impl::macro_fn(attr, item)
}

#[proc_macro_derive(CssVariables, attributes(css_vars))]
#[proc_macro_error]
pub fn css_variables(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    css_variables::macro_fn(input).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn css_var(input: TokenStream) -> TokenStream {
    css_var::macro_fn(input.into()).into()
}
