use stylist::ast::{sheet, Sheet};
use stylist::Style;

fn use_simple_sheet() {
    let sheet: Sheet = sheet!("color:red;");
    Style::new(sheet).unwrap();
}

fn use_macro_inline_simple() {
    let sheet: Sheet = sheet!(color:red;);
    Style::new(sheet).unwrap();
}

fn use_macro_complex() {
    let i = 513;
    let sheet: Sheet = sheet!(
        r#"
        color:red;

        .class-name-a {
            background: red;

            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }

        @media screen and (max-width: ${i}px;) {
            font-size: 0.9rem;

            .class-name-b {
                flex-direction: row;
            }
        }
    "#,
        i = i / 1000
    );
    Style::new(sheet).unwrap();
}

fn use_macro_inline_complex() {
    let i = 1_000_101;
    let sheet: Sheet = sheet!(
        color: red;

        .class-name-a {
            background: red;

            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }

        @media screen and (max-width: ${i / 1000}px;) {
            font-size: 0.9rem;

            .class-name-b {
                flex-direction: row;
            }
        }
    );
    Style::new(sheet).unwrap();
}

fn use_large_sheets() {
    let snippet = sheet!("color:red;");
    let mut sheet_parts = vec![];
    let mut sheets = vec![];

    for _ in 1..100 {
        sheet_parts.extend_from_slice(&snippet);
        let sheet: Sheet = Sheet::from(sheet_parts.clone());
        sheets.push(sheet);
    }

    for sheet in sheets {
        Style::new(sheet).expect("Failed to mount stylesheet.");
    }
}

pub fn use_stylist() {
    use_simple_sheet();
    use_macro_inline_simple();
    use_macro_complex();
    use_macro_inline_complex();
    use_large_sheets();
}
