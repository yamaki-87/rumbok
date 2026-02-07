#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01_ok.rs");
    t.pass("tests/ui/03_ok_generic.rs");
    t.compile_fail("tests/ui/02_fail_tuple_struct.rs");
}
