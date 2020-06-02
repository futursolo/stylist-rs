// Props to the Yew team https://github.com/yewstack/yew/blob/master/build.rs

use std::env;

pub fn main() {
    let using_cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").is_ok();
    if using_cargo_web {
        panic!("cargo-web is not compatible with web-sys");
    }
}
