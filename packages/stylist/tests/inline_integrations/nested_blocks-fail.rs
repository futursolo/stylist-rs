fn main() {
    let _ = stylist::css! {
        .outer {
            .inner {
                background-color: red;
            }
        }
    };
}
