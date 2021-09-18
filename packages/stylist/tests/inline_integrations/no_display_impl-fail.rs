enum NoDisplay {
    ND,
}
fn main() {
    let expr = NoDisplay::ND;
    let _ = stylist::css! {
        background: ${expr};
    };
}
