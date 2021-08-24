fn main() {
    let _ = env_logger::builder().is_test(true).try_init();
    let style = stylist::style! {
        @supports (display: grid) {
            background-color: grey;
        }
    };
    let result_reg = regex::Regex::new(
        r#"@supports \(display:grid\) \{
\.stylist-[[:alnum:]]+ \{
background-color: grey;
\}
\}
"#,
    )
    .unwrap();
    log::debug!("{}", style.get_style_str());
    assert!(result_reg.is_match(style.get_style_str()));
}
