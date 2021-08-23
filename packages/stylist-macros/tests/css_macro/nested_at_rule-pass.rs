fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let css = stylist::css! {
        .outer {
            @media print {
                background-color: grey;
            }
            @page {
                margin: ${"2cm"};
            }
        }
    }
    .to_style();
    let result_reg = regex::Regex::new(
        r#"@media print \{
\.stylist-[[:alnum:]]+ \.outer \{
background-color: grey;
\}
\}
@page  \{
\.stylist-[[:alnum:]]+ .outer \{
margin: 2cm;
\}
\}
"#,
    )
    .unwrap();
    log::debug!("{}", css.get_style_str());
    assert!(result_reg.is_match(css.get_style_str()));
}
