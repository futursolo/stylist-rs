use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod argument;
mod css;
mod global_style;
mod sheet;
mod style;
mod to_tokens_with_args;

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
