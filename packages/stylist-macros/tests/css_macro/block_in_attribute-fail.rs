fn main() {
    let _ = stylist::css! {
        .outer {
            border: ${is_an_expression};
            background-color: {should_be_expression};
        }
    };
}
