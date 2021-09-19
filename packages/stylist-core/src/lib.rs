#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![deny(clippy::cognitive_complexity)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

mod error;
pub use error::{Error, Result, ResultDisplay};
pub mod ast;
pub mod bow;

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub mod parser;
#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub mod tokens;

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Sheet;
    use ast::ToStyleStr;

    #[test]
    fn test_scoped_complex() {
        let style: Sheet = r#"
                background-color: black;
                .with-class {
                    color: red;
                }
                @media screen and (max-width: 600px) {
                    color: yellow;
                }
                @supports (display: grid) {
                    display: grid;
                }

                header, footer {
                    border: 1px solid black;

                    @supports (max-width: 500px) {
                        max-width: 500px;

                        @media screen and (max-width: 500px) {
                            display: flex;
                            flex-direction: row;
                        }
                    }
                }
            "#
        .parse()
        .expect("Failed to create Style.");

        assert_eq!(
            style.to_style_str(Some("test-style-cls")),
            r#".test-style-cls {
    background-color: black;
}
.test-style-cls .with-class {
    color: red;
}
@media screen and (max-width: 600px) {
    .test-style-cls {
        color: yellow;
    }
}
@supports (display: grid) {
    .test-style-cls {
        display: grid;
    }
}
.test-style-cls header, .test-style-cls footer {
    border: 1px solid black;
}
@supports (max-width: 500px) {
    .test-style-cls header, .test-style-cls footer {
        max-width: 500px;
    }
}
@supports (max-width: 500px) {
    @media screen and (max-width: 500px) {
        .test-style-cls header, .test-style-cls footer {
            display: flex;
            flex-direction: row;
        }
    }
}
"#,
        )
    }
}
