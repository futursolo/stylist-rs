#[test]
fn test_macro_integrations() {
    let t = trybuild::TestCases::new();

    t.pass("tests/macro_integrations/*-pass.rs");
    t.compile_fail("tests/macro_integrations/*-fail.rs");

    let _ = stylist::style! {
        .inner {
            background-color: ${"4e4e4e"};
        }
    };
}
