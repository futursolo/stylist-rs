// Props to the Yew team https://github.com/yewstack/yew/blob/master/build.rs

use std::env;

pub fn main() {
    let using_web_sys = cfg!(feature = "web_sys");
    let using_std_web = cfg!(feature = "std_web");
    let docs = cfg!(docs);

    if !docs {
        if using_web_sys && using_std_web {
            panic!("CSSinRust does not allow the `web_sys` and `std_web` cargo features to be used simultaneously");
        } else if !using_web_sys && !using_std_web {
            panic!("CSSinRust requires selecting either the `web_sys` or `std_web` cargo feature");
        }
    }

    let using_cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").is_ok();
    if using_cargo_web && using_web_sys {
        panic!("cargo-web is not compatible with web-sys");
    }
}
