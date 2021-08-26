#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(missing_debug_implementations)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod stringly;
mod tokentree;

mod css;
mod global_style;
mod sheet;
mod style;

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

#[cfg(test)]
mod test {
    use log::debug;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse() {
        init();
        let input = r#"
        &, .some-class, --someid, struct, .${color_red} {
            color: ${color_red};
        }
        "#;
        let output = super::sheet::macro_fn(input.parse().unwrap());
        debug!("{}", output);
    }
}
