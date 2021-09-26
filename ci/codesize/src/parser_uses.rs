use stylist::Style;

fn basic_parsing() {
    Style::new(
        r#"
            background-color: red;
            .nested {
                background-color: blue;
                width: 100px
            }
        "#,
    )
    .expect("Failed to create style");
}

pub fn use_stylist() {
    basic_parsing();
}
