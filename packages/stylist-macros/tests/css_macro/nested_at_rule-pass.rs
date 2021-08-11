fn main() {
    let sheet = stylist::css! {
        .outer {
            @media print {
                background-color: grey;
            }
            @page {
                margin: ${"2cm"};
            }
        }
    };
    let css = stylist::Style::new(sheet).unwrap();
    let result_reg = regex::Regex::new(
        r#"@media print \{
\.stylist-[[:alnum:]]+ \.outer \{
background-color: grey;
\}
\}
@page \{
\.stylist-[[:alnum:]]+ .outer \{
margin: 2cm;
\}
\}
"#,
    )
    .unwrap();
    assert!(result_reg.is_match(css.get_style_str()));
}
