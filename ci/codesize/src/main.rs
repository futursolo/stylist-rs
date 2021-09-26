#[cfg(feature = "macros")]
mod macro_uses;
#[cfg(feature = "parser")]
mod parser_uses;
#[cfg(feature = "yew_integration")]
mod yew_uses;

fn main() {
    #[cfg(feature = "macros")]
    macro_uses::use_stylist();
    #[cfg(feature = "yew_integration")]
    yew_uses::use_stylist();
    #[cfg(feature = "parser")]
    parser_uses::use_stylist();
}
