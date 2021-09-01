#[test]
fn test_macro_integrations() {
    let t = trybuild::TestCases::new();

    t.pass("tests/macro_integrations/*-pass.rs");
    t.pass("tests/macro_literal_integrations/*-pass.rs");
    t.compile_fail("tests/macro_integrations/*-fail.rs");
    t.compile_fail("tests/macro_literal_integrations/*-fail.rs");
}
