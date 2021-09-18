fn main() {
    let _ = stylist::css! {r#"
        background: ${used};
    "#, unused = 1000, used = "black"
    };
}
