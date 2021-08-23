fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let style = stylist::style! {
        .outer {
            @media print {
                background-color: grey;
            }
            @page {
                margin: ${"2cm"};
            }
        }
    };
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
    log::debug!("{}", style.get_style_str());
    assert!(result_reg.is_match(style.get_style_str()));
}
