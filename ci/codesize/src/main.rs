#[cfg(feature = "macros")]
mod macro_uses;

fn main() {
    #[cfg(feature = "macros")]
    macro_uses::use_stylist();
}
