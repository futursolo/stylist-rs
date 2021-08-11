#[allow(dead_code)]
#[test]
fn css_macro() {
    let t = trybuild::TestCases::new();

    t.pass("tests/css_macro/*-pass.rs");
    t.compile_fail("tests/css_macro/*-fail.rs");
}
