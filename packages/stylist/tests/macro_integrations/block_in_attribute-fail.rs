fn main() {
    let is_an_expression = "black";
    let _ = stylist::css! {
        .outer {
            border: ${is_an_expression};
            background-color: {should_be_expression};
            color: {another_expression};
        }
    };
}
