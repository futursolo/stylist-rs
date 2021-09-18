use stylist::ast::{sheet, Sheet};
use stylist::Style;

use crate::utils::now;

pub fn bench_parse_simple() -> f64 {
    let start_time = now();
    for _ in 0..10_000_000 {
        let _sheet: Sheet = "color:red;".parse().expect("Failed to parse stylesheet.");
    }

    now() - start_time
}

pub fn bench_macro_simple() -> f64 {
    let start_time = now();
    for _ in 0..10_000_000 {
        let _sheet: Sheet = sheet!("color:red;");
    }

    now() - start_time
}

pub fn bench_macro_inline_simple() -> f64 {
    let start_time = now();
    for _ in 0..10_000_000 {
        let _sheet: Sheet = sheet!(color:red;);
    }

    now() - start_time
}

pub fn bench_parse_simple_no_cache() -> f64 {
    let start_time = now();
    for i in 0..100_000 {
        let _sheet: Sheet = format!("height: {}px;", i)
            .parse()
            .expect("Failed to parse stylesheet.");
    }

    now() - start_time
}

pub fn bench_parse_complex() -> f64 {
    let start_time = now();
    for i in 0..1_000_000 {
        let _sheet: Sheet = format!(
            r#"
            color:red;

            .class-name-a {{
                background: red;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            }}

            @media screen and (max-width: {i}px;) {{
                font-size: 0.9rem;

                .class-name-b {{
                    flex-direction: row;
                }}
            }}
        "#,
            i = i / 1000
        )
        .parse()
        .expect("Failed to parse stylesheet.");
    }

    now() - start_time
}

pub fn bench_macro_complex() -> f64 {
    let start_time = now();
    for i in 0..1_000_000 {
        let _sheet: Sheet = sheet!(
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
    }

    now() - start_time
}

pub fn bench_macro_inline_complex() -> f64 {
    let start_time = now();
    for i in 0..1_000_000 {
        let _sheet: Sheet = sheet!(
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
    }

    now() - start_time
}

pub fn bench_parse_complex_no_cache() -> f64 {
    let start_time = now();
    for i in 0..10_000 {
        let _sheet: Sheet = format!(
            r#"
                color:red;
                height: {}px;

                .class-name-a {{
                    background: red;

                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                }}

                @media screen and (max-width: 500px;) {{
                    font-size: 0.9rem;

                    .class-name-b {{
                        flex-direction: row;
                    }}
                }}
            "#,
            i
        )
        .parse()
        .expect("Failed to parse stylesheet.");
    }

    now() - start_time
}

pub fn bench_cached_lookup() -> f64 {
    let snippet = "color:red;";
    let mut sheets = Vec::new();

    for i in 1..100 {
        let sheet: Sheet = {
            let mut s = String::new();
            for _ in 0..i {
                s.push_str(snippet);
            }

            s.parse().expect("Failed to parse stylesheet.")
        };

        sheets.push(sheet);
    }

    let first_sheet = sheets.first().cloned().unwrap();

    for sheet in sheets {
        let _style = Style::new(sheet).expect("Failed to mount stylesheet.");
    }

    let start_time = now();
    for _ in 0..1_000_000 {
        let _style = Style::new(first_sheet.clone()).expect("Failed to create style.");
    }

    now() - start_time
}

pub fn bench_cached_lookup_big_sheet() -> f64 {
    let snippet = "color:red;";
    let mut sheets = Vec::new();

    for i in 1..100 {
        let sheet: Sheet = {
            let mut s = String::new();
            for _ in 0..i {
                s.push_str(snippet);
            }

            s.parse().expect("Failed to parse stylesheet.")
        };

        sheets.push(sheet);
    }

    let last_sheet = sheets.last().cloned().unwrap();

    for sheet in sheets {
        let _style = Style::new(sheet).expect("Failed to mount stylesheet.");
    }

    let start_time = now();
    for _ in 0..100_000 {
        let _style = Style::new(last_sheet.clone()).expect("Failed to create style.");
    }

    now() - start_time
}

pub fn bench_mounting() -> f64 {
    let mut sheets = Vec::new();

    for i in 0..2_000 {
        let sheet: Sheet = {
            let s = format!(r#"font-size: {}px;"#, i);

            s.parse().expect("Failed to parse stylesheet.")
        };

        sheets.push(sheet);
    }

    let start_time = now();
    for sheet in sheets {
        let _style = Style::new(sheet).expect("Failed to mount stylesheet.");
    }

    now() - start_time
}
